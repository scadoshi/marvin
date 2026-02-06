use rig::message::{AssistantContent, Message, UserContent};

use crate::{commands::Command, ui::horizontal_line};

pub const HISTORY_LEN: usize = 10;

pub struct History;

impl Command for History {
    async fn execute(state: &mut crate::chat::State) -> anyhow::Result<()> {
        const TRUNCATE_AT: usize = 300;
        if state.history().is_empty() {
            state.clear_input();
            horizontal_line();
            println!("No chat history");
            return Ok(());
        }
        horizontal_line();
        println!("Showing last {} messages", HISTORY_LEN);
        horizontal_line();
        state.clear_input();
        let messages: Vec<_> = state
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
        Ok(())
    }
}
