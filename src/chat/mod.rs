pub mod config;
pub mod input;
use std::collections::HashSet;
use std::sync::Arc;

use futures::StreamExt;
pub use input::ChatInput;
use rig::agent::{Agent, MultiTurnStreamItem};
use rig::client::CompletionClient;
use rig::completion::Chat as ChatTrait;
use rig::message::Message;
use rig::providers::anthropic::Client;
use rig::providers::anthropic::completion::CompletionModel;
use rig::providers::anthropic::streaming::{PartialUsage, StreamingCompletionResponse};
use rig::streaming::{StreamedAssistantContent, StreamingChat};

use crate::agent_tools::web::tavily::TavilyClient;
use crate::agent_tools::{math, web};
use crate::anthropic::{self, ModelInfo};
use crate::chat::config::Config;
use crate::ui::{horizontal_line, welcome_message};

pub static PREAMBLE: &str = include_str!("preamble.txt");

pub struct Chat {
    id: u16,
    config: Config,
    model_options: Vec<ModelInfo>,
    agent: Agent<CompletionModel>,
    history: Vec<Message>,
    input: ChatInput,
    total_input_tokens_used: usize,
    total_output_tokens_used: usize,
    tavily_client: Arc<TavilyClient>,
}

pub const CHATS_DIR_NAME: &str = "chats";
fn next_chat_id() -> anyhow::Result<u16> {
    std::fs::create_dir_all(CHATS_DIR_NAME)?;
    let existing_chat_ids: HashSet<u16> = std::fs::read_dir(CHATS_DIR_NAME)?
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .filter(|ent| ent.path().extension().and_then(|ostr| ostr.to_str()) == Some("json"))
        .filter_map(|ent| {
            ent.path()
                .file_prefix()
                .and_then(|prfx| prfx.to_str())
                .and_then(|str| str.parse::<u16>().ok())
        })
        .collect();
    match (0..u16::MAX).find(|id| !existing_chat_ids.contains(id)) {
        Some(id) => Ok(id),
        None => Err(anyhow::anyhow!(
            "Chat count has hit its limit of {}",
            u16::MAX
        )),
    }
}

/// Marvin on `model`, with the math and web tools.
fn agent(
    api_key: &str,
    model: &ModelInfo,
    tavily_client: &Arc<TavilyClient>,
) -> anyhow::Result<Agent<CompletionModel>> {
    Ok(Client::new(api_key)?
        .agent(&model.id)
        .name("Marvin")
        .preamble(PREAMBLE)
        .max_tokens(model.max_tokens)
        .tools(math::tools())
        .tools(web::tools(tavily_client))
        .default_max_turns(100)
        .build())
}

impl Chat {
    pub async fn new() -> anyhow::Result<Self> {
        let id = next_chat_id()?;
        welcome_message(id);
        let config = Config::from_env()?;
        let tavily_client = Arc::new(TavilyClient::from(&config));
        let model_options =
            anthropic::fetch_models(anthropic::API_BASE, config.anthropic_api_key()).await?;
        let agent: Agent<CompletionModel>;
        let mut input = String::new();
        println!("Current model: not set");
        for (i, model) in model_options.iter().enumerate() {
            println!("{}. {}", i + 1, model.display_name);
        }
        horizontal_line();
        loop {
            println!("Select a model");
            horizontal_line();
            std::io::stdin().read_line(&mut input)?;
            if let Some(model) = model_options
                .iter()
                .enumerate()
                .find(|(i, _)| (i + 1).to_string() == input.trim())
                .map(|(_, model)| model)
            {
                agent = self::agent(config.anthropic_api_key(), model, &tavily_client)?;
                horizontal_line();
                break;
            } else if input.trim() == "/exit" {
                println!("Farewell!");
                std::process::exit(0);
            }
            input.clear();
        }
        Ok(Self {
            id,
            config,
            model_options,
            agent,
            history: Vec::new(),
            input: ChatInput::new(),
            total_input_tokens_used: 0,
            total_output_tokens_used: 0,
            tavily_client,
        })
    }
    pub fn id(&self) -> u16 {
        self.id
    }
    pub fn config(&self) -> &Config {
        &self.config
    }
    pub fn model_options(&self) -> &[ModelInfo] {
        self.model_options.as_slice()
    }
    pub fn model(&self) -> &str {
        self.model_options()
            .iter()
            .find(|model| model.id == self.agent.model.model)
            .map_or("", |model| model.display_name.as_str())
    }
    pub fn set_agent(&mut self, model: &ModelInfo) -> anyhow::Result<()> {
        self.agent = agent(
            self.config().anthropic_api_key(),
            model,
            &self.tavily_client,
        )?;
        Ok(())
    }
    pub async fn send(&mut self, message: impl Into<Message>) -> anyhow::Result<String> {
        let message = message.into();
        self.add_to_chat_history(message.clone());
        let response = self
            .agent
            .chat(message, self.chat_history().to_owned())
            .await?;
        self.add_to_chat_history(Message::assistant(response.clone()));
        Ok(response)
    }
    pub async fn stream(&mut self, message: impl Into<Message>) {
        horizontal_line();
        let message: Message = message.into();
        self.add_to_chat_history(message.clone());
        let mut stream = self
            .agent
            .stream_chat(message, self.chat_history().to_owned())
            .await;
        while let Some(result) = stream.next().await {
            match result {
                Ok(MultiTurnStreamItem::FinalResponse(final_response)) => {
                    self.history
                        .push(Message::assistant(final_response.response()));
                }
                Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(
                    text,
                ))) if !text.text().trim().is_empty() => {
                    print!("{}", text.text());
                }
                Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Final(
                    StreamingCompletionResponse {
                        usage:
                            PartialUsage {
                                output_tokens,
                                input_tokens,
                            },
                    },
                ))) => {
                    println!();
                    self.add_output_tokens_used(output_tokens);
                    if let Some(input_tokens) = input_tokens {
                        self.add_input_tokens_used(input_tokens);
                    }
                }
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Stream Error: {e}");
                }
            }
        }
    }
    pub fn chat_history(&self) -> &[Message] {
        self.history.as_slice()
    }
    pub fn clear_chat_history(&mut self) {
        self.history.clear();
    }
    pub fn add_to_chat_history(&mut self, message: impl Into<Message>) {
        self.history.push(message.into());
    }
    pub fn save_chat_history_to_file(&self) -> anyhow::Result<()> {
        let file_path = format!("{}/{}.json", CHATS_DIR_NAME, self.id());
        let file = std::fs::File::create(file_path)?;
        let chat_history_json = serde_json::to_value(self.chat_history())?;
        serde_json::to_writer_pretty(file, &chat_history_json)?;
        Ok(())
    }
    pub fn input(&self) -> &ChatInput {
        &self.input
    }
    pub fn read_input(&mut self) {
        let mut input_str = String::new();
        match std::io::stdin().read_line(&mut input_str) {
            Ok(_) => self.input = ChatInput::from(input_str),
            Err(e) => {
                eprintln!("Error: {e}");
                println!("Input failed");
                self.clear_input();
            }
        }
    }
    pub fn clear_input(&mut self) {
        self.input.clear();
    }
    pub fn total_input_tokens_used(&self) -> usize {
        self.total_input_tokens_used
    }
    pub fn add_input_tokens_used(&mut self, input_tokens: usize) {
        self.total_input_tokens_used += input_tokens;
    }
    pub fn total_output_tokens_used(&self) -> usize {
        self.total_output_tokens_used
    }
    pub fn add_output_tokens_used(&mut self, output_tokens: usize) {
        self.total_output_tokens_used += output_tokens;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn the_agent_is_marvin_with_room_for_long_tool_chains() {
        let model = ModelInfo {
            id: String::from("claude-test"),
            created_at: String::new(),
            display_name: String::from("Claude Test"),
            r#type: String::from("model"),
            max_tokens: 1024,
        };
        let tavily = Arc::new(TavilyClient::new("test"));
        let agent = agent("test", &model, &tavily).unwrap();
        assert_eq!(agent.name.as_deref(), Some("Marvin"));
        assert_eq!(agent.default_max_turns, Some(100));
        assert_eq!(agent.model.model, "claude-test");
    }
}
