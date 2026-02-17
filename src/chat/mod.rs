pub mod input;

use crate::{chat::input::Input, config::Config, ui::horizontal_line};
use futures::StreamExt;
use rig::{
    agent::{Agent, MultiTurnStreamItem},
    completion::Chat,
    message::Message,
    providers::anthropic::{
        completion::{CompletionModel, CLAUDE_3_5_HAIKU, CLAUDE_4_OPUS, CLAUDE_4_SONNET},
        streaming::{PartialUsage, StreamingCompletionResponse},
    },
    streaming::{StreamedAssistantContent, StreamingChat},
};
use std::io::stdin;

#[derive(Clone)]
pub struct State {
    config: Config,
    model_options: Vec<String>,
    model: String,
    agent: Agent<CompletionModel>,
    history: Vec<Message>,
    input: Input,
    total_input_tokens_used: usize,
    total_output_tokens_used: usize,
}

impl State {
    pub fn new() -> Self {
        let config = Config::from_env();
        let model_options = vec![
            String::from(CLAUDE_3_5_HAIKU),
            String::from(CLAUDE_4_OPUS),
            String::from(CLAUDE_4_SONNET),
        ];
        assert!(!model_options.is_empty());
        let model = CLAUDE_3_5_HAIKU.to_string();
        let agent = config.build_agent(&model);
        Self {
            config,
            model_options,
            model,
            agent,
            history: Vec::new(),
            input: Input::new(),
            total_input_tokens_used: 0,
            total_output_tokens_used: 0,
        }
    }
    pub fn refresh_agent(&mut self) {
        self.agent = self.config().build_agent(self.model());
    }
    pub fn config(&self) -> &Config {
        &self.config
    }
    pub fn model_options(&self) -> &[String] {
        self.model_options.as_slice()
    }
    pub fn model(&self) -> &str {
        &self.model
    }
    pub fn set_model(&mut self, model: impl Into<String>) -> &mut Self {
        self.model = model.into();
        self
    }
    pub async fn send(&mut self, message: impl Into<Message>) -> anyhow::Result<String> {
        let message = message.into();
        self.add_to_history(message.clone());
        let response = self.agent.chat(message, self.history().to_owned()).await?;
        self.add_to_history(Message::assistant(response.clone()));
        Ok(response)
    }
    pub async fn stream(&mut self, message: impl Into<Message>) {
        horizontal_line();
        let message: Message = message.into();
        self.add_to_history(message.clone());
        let mut stream = self
            .agent
            .stream_chat(message, self.history().to_owned())
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
                    eprintln!("Stream Error: {}", e);
                }
            }
        }
    }
    pub fn history(&self) -> &[Message] {
        self.history.as_slice()
    }
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
    pub fn add_to_history(&mut self, message: impl Into<Message>) {
        self.history.push(message.into());
    }
    pub fn input(&self) -> &Input {
        &self.input
    }
    pub fn get_input(&mut self) {
        let mut input_str = String::new();
        match stdin().read_line(&mut input_str) {
            Ok(_) => self.input = Input::from(input_str),
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
