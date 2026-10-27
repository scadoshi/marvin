use rig::message::{AssistantContent, Message, UserContent};

use crate::chat::Chat;
use crate::ui::horizontal_line;

pub const HISTORY_LEN: usize = 10;
/// Characters of a message shown before it is cut off with `...`.
const TRUNCATE_AT: usize = 300;

/// The first [`TRUNCATE_AT`] characters of `text`, trimmed, with `...` when anything was cut.
fn preview(text: &str) -> String {
    let truncated = text.chars().take(TRUNCATE_AT).collect::<String>();
    let end = if text.chars().nth(TRUNCATE_AT).is_some() {
        "..."
    } else {
        ""
    };
    format!("{}{end}", truncated.trim())
}

impl Chat {
    pub(crate) fn show_chat_history(&mut self) {
        if self.chat_history().is_empty() {
            self.clear_input();
            horizontal_line();
            println!("No chat history");
            return;
        }
        horizontal_line();
        println!("Showing last {HISTORY_LEN} messages");
        horizontal_line();
        self.clear_input();
        let messages: Vec<_> = self
            .chat_history()
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
                        println!("*User*: \"{}\"", preview(text.text()));
                    }
                    UserContent::Image(_) => println!("*User*: *image*"),
                    UserContent::Audio(_) => println!("*User*: *audio*"),
                    UserContent::Video(_) => println!("*User*: *video*"),
                    UserContent::Document(_) => println!("*User*: *document*"),
                    UserContent::ToolResult(_) => println!("*User*: *tool result*"),
                },
                Message::Assistant { content, .. } => match content.first() {
                    AssistantContent::Text(text) => {
                        println!("*Assistant*: \"{}\"", preview(text.text()));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_short_message_shows_whole() {
        assert_eq!(preview("  hello  "), "hello");
    }

    #[test]
    fn a_long_message_is_cut_with_an_ellipsis() {
        let long = "a".repeat(TRUNCATE_AT + 1);
        assert_eq!(preview(&long), format!("{}...", "a".repeat(TRUNCATE_AT)));
    }

    #[test]
    fn a_short_non_ascii_message_is_not_marked_as_cut() {
        let accented = "é".repeat(TRUNCATE_AT - 1);
        assert_eq!(preview(&accented), accented);
    }
}
