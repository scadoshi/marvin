use rig::message::{AssistantContent, Message, UserContent};

use crate::{chat, ui::horizontal_line};

pub const HISTORY_LEN: usize = 10;

pub trait ShowChatHistory {
    fn show_chat_history(&mut self);
}

impl ShowChatHistory for chat::State {
    fn show_chat_history(&mut self) {
        const TRUNCATE_AT: usize = 300;
        if self.history().is_empty() {
            self.clear_input();
            horizontal_line();
            println!("No chat history");
            return;
        }
        horizontal_line();
        println!("Showing last {} messages", HISTORY_LEN);
        horizontal_line();
        self.clear_input();
        let messages: Vec<_> = self
            .history()
            .iter()
            .rev()
            .take(HISTORY_LEN)
            .rev()
            .collect();
        for (i, message) in messages.iter().enumerate() {
            println!();
            match message {
                Message::User { content } => match content.first() {
                    UserContent::Text(text) => {
                        let text = text.text();
                        let truncated = text.chars().take(TRUNCATE_AT).collect::<String>();
                        let end = if text.len() > TRUNCATE_AT {
                            String::from("...")
                        } else {
                            String::new()
                        };
                        println!("*User*: \"{}{}\"", truncated.trim(), end);
                    }
                    UserContent::Image(_) => println!("*User*: *image*"),
                    UserContent::Audio(_) => println!("*User*: *audio*"),
                    UserContent::Video(_) => println!("*User*: *video*"),
                    UserContent::Document(_) => println!("*User*: *document*"),
                    UserContent::ToolResult(_) => println!("*User*: *tool result*"),
                },
                Message::Assistant { content, .. } => match content.first() {
                    AssistantContent::Text(text) => {
                        let text = text.text();
                        let truncated = text.chars().take(TRUNCATE_AT).collect::<String>();
                        let end = if text.len() > TRUNCATE_AT {
                            String::from("...")
                        } else {
                            String::new()
                        };
                        println!("*Assistant*: \"{}{}\"", truncated.trim(), end);
                    }
                    AssistantContent::Image(_) => println!("*Assistant*: *image*"),
                    AssistantContent::ToolCall(_) => println!("*Assistant*: *tool call*"),
                    AssistantContent::Reasoning(_) => println!("*Assistant*: *reasoning*"),
                },
            }
            println!();
            if let Some(final_i) = messages.len().checked_sub(1)
                && i != final_i
            {
                println!("---");
            }
        }
    }
}
