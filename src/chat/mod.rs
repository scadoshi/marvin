pub mod input;

use crate::{chat::input::Input, config::Config};
use rig::{
    agent::Agent,
    completion::Chat,
    message::Message,
    providers::anthropic::completion::{
        CompletionModel, CLAUDE_3_5_HAIKU, CLAUDE_3_5_SONNET, CLAUDE_3_7_SONNET, CLAUDE_4_OPUS,
        CLAUDE_4_SONNET,
    },
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
}

impl State {
    pub fn new() -> Self {
        let config = Config::from_env();
        let model_options = vec![
            String::from(CLAUDE_3_5_HAIKU),
            String::from(CLAUDE_3_5_HAIKU),
            String::from(CLAUDE_3_5_SONNET),
            String::from(CLAUDE_3_7_SONNET),
            String::from(CLAUDE_4_OPUS),
            String::from(CLAUDE_4_SONNET),
        ];
        let model = CLAUDE_3_5_HAIKU.to_string();
        let agent = config.build_agent(&model);
        let history = Vec::new();
        let input = Input::new();
        Self {
            config,
            model_options,
            model,
            agent,
            history,
            input,
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
    pub async fn send_assistant_message(
        &mut self,
        message: impl Into<String>,
    ) -> anyhow::Result<String> {
        Ok(self
            .agent
            .chat(Message::assistant(message), self.history().into())
            .await?)
    }
    pub async fn send_user_message(
        &mut self,
        message: impl Into<String>,
    ) -> anyhow::Result<String> {
        Ok(self
            .agent
            .chat(Message::user(message), self.history().into())
            .await?)
    }
    pub fn record_assistant_message(&mut self, message: impl Into<String>) {
        self.history.push(Message::assistant(message.into()));
    }
    pub fn record_user_message(&mut self, message: impl Into<String>) {
        self.history.push(Message::user(message.into()));
    }
    pub fn history(&self) -> &[Message] {
        self.history.as_slice()
    }
    pub fn clear_history(&mut self) {
        self.history.clear();
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
}
