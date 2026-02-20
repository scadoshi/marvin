pub mod config;
pub mod input;
pub use input::ChatInput;

use crate::{
    agent_tools::add::Adder,
    anthropic::{get_models::GetAnthropicModels, ModelInfo},
    chat::config::Config,
    ui::{horizontal_line, welcome_message},
};
use futures::StreamExt;
use rig::{
    agent::{Agent, MultiTurnStreamItem},
    client::CompletionClient,
    completion::Chat as ChatTrait,
    message::Message,
    providers::anthropic::{
        completion::CompletionModel,
        streaming::{PartialUsage, StreamingCompletionResponse},
        Client,
    },
    streaming::{StreamedAssistantContent, StreamingChat},
};
use std::collections::HashSet;

pub static PREAMBLE: &str = include_str!("preamble.txt");

pub struct Chat {
    id: u16,
    config: Config,
    model_options: Vec<ModelInfo>,
    agent: Agent<CompletionModel>,
    chat_history: Vec<Message>,
    input: ChatInput,
    total_input_tokens_used: usize,
    total_output_tokens_used: usize,
}

pub const CHATS_DIR_NAME: &str = "chats";
fn next_chat_id() -> anyhow::Result<u16> {
    std::fs::create_dir_all(CHATS_DIR_NAME)?;
    let existing_chat_ids: HashSet<u16> = std::fs::read_dir(CHATS_DIR_NAME)?
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .filter(|ent| ent.path().extension().and_then(|ostr| ostr.to_str()) == Some("json"))
        .flat_map(|ent| {
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

impl Chat {
    pub async fn new() -> anyhow::Result<Self> {
        let id = next_chat_id()?;
        welcome_message(id);
        let config = Config::from_env()?;
        let model_options = config.get_models().await?;
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
            if let Some((_, ModelInfo { id, .. })) = model_options
                .iter()
                .enumerate()
                .find(|(i, _)| (i + 1).to_string() == input.trim())
            {
                agent = Client::new(config.anthropic_api_key())?
                    .agent(id)
                    .name("Marvin")
                    .preamble(PREAMBLE)
                    .tool(Adder)
                    .default_max_turns(100)
                    .build();
                horizontal_line();
                break;
            } else if input.trim() == "/exit" {
                println!("Farewell!");
                std::process::exit(0);
            } else {
                input.clear();
            }
        }
        Ok(Self {
            id,
            config,
            model_options,
            agent,
            chat_history: Vec::new(),
            input: ChatInput::new(),
            total_input_tokens_used: 0,
            total_output_tokens_used: 0,
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
            .map(|model| model.display_name.as_str())
            .unwrap_or("")
    }
    pub fn set_agent(&mut self, model: ModelInfo) -> anyhow::Result<()> {
        self.agent = Client::new(self.config().anthropic_api_key())?
            .agent(model.id)
            .preamble(PREAMBLE)
            .build();
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
                    self.chat_history
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
                    eprintln!("Stream Error: {}", e);
                }
            }
        }
    }
    pub fn chat_history(&self) -> &[Message] {
        self.chat_history.as_slice()
    }
    pub fn clear_chat_history(&mut self) {
        self.chat_history.clear();
    }
    pub fn add_to_chat_history(&mut self, message: impl Into<Message>) {
        self.chat_history.push(message.into());
    }
    pub fn save_chat_history_to_file(&self) -> anyhow::Result<()> {
        let file_path = format!("{}/{}.json", CHATS_DIR_NAME, self.id());
        let file = std::fs::File::create(file_path)?;
        let chat_history_json = serde_json::to_value(self.chat_history())?;
        serde_json::to_writer_pretty(file, &chat_history_json)?;
        Ok(())
    }
    pub fn append_chat_history_from_file_infallible(&mut self, id: u16) {
        let file_path = format!("{}/{}.json", CHATS_DIR_NAME, id);
        let file_result = std::fs::File::open(file_path);
        match file_result {
            Ok(mut file) => {
                println!("chat_history with ID: {} found!", id);
                let file_str = {
                    let mut file_str = String::new();
                    let Ok(_) =
                        <std::fs::File as std::io::Read>::read_to_string(&mut file, &mut file_str)
                    else {
                        println!("Failed to read file");
                        return;
                    };
                    file_str
                };
                let Ok(chat_history) = serde_json::from_str::<Vec<Message>>(&file_str) else {
                    println!("Failed to serialize file to `Vec<Message>`");
                    return;
                };
                self.chat_history.extend(chat_history);
            }
            Err(e) => println!("Failed to get chat_history: {}", e),
        }
    }
    pub fn input(&self) -> &ChatInput {
        &self.input
    }
    pub fn get_input(&mut self) {
        let mut input_str = String::new();
        match std::io::stdin().read_line(&mut input_str) {
            Ok(_) => self.input = ChatInput::from(input_str),
            Err(e) => {
                eprintln!("Error: {}", e);
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
