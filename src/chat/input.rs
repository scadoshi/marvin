//! Parses a line of user input into a slash command or a message.

use std::io::BufRead;

/// One line of user input: a slash command, a message for the model, or nothing.
#[derive(Debug, Clone, Default)]
pub(crate) enum ChatInput {
    /// `/exit`: save the history and quit.
    ExitProcess,
    /// `/history`: print the latest messages.
    ShowChatHistory,
    /// `/save`: write the history to this chat's file.
    SaveChatHistory,
    /// `/import <id>`: append a saved chat's messages to the history.
    ImportChatHistory(u16),
    /// `/tokens`: print the token totals.
    ShowTokenUsage,
    /// `/clear`: empty the history.
    ClearContext,
    /// `/model`: pick another model.
    SwitchModel,
    /// `/help`: list the commands.
    ShowHelpMessage,
    /// `/summarize`: ask the model to summarize the chat.
    ShowContextSummary,
    /// `/compact`: replace the history with a summary.
    CompactContext,
    /// Any other line, trimmed, to send to the model.
    SendMessage(String),
    /// A blank line, or input that has been handled.
    #[default]
    None,
}

impl<T> From<T> for ChatInput
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let message = value.as_ref().trim();
        let value = message.to_lowercase();
        if value == "/exit" {
            Self::ExitProcess
        } else if value == "/history" {
            Self::ShowChatHistory
        } else if value == "/save" {
            Self::SaveChatHistory
        } else if let Some(id) = value
            .strip_prefix("/import ")
            .and_then(|id| id.trim().parse::<u16>().ok())
        {
            Self::ImportChatHistory(id)
        } else if value == "/tokens" {
            Self::ShowTokenUsage
        } else if value == "/clear" {
            Self::ClearContext
        } else if value == "/model" {
            Self::SwitchModel
        } else if value == "/help" {
            Self::ShowHelpMessage
        } else if value == "/summarize" {
            Self::ShowContextSummary
        } else if value == "/compact" {
            Self::CompactContext
        } else if value.is_empty() {
            Self::None
        } else {
            Self::SendMessage(message.to_owned())
        }
    }
}

/// Reads one line from `reader` and parses it. The end of input reads as `/exit`.
///
/// # Errors
///
/// Returns an error if reading from `reader` fails.
pub(crate) fn read(reader: &mut impl BufRead) -> std::io::Result<ChatInput> {
    let mut line = String::new();
    if reader.read_line(&mut line)? == 0 {
        return Ok(ChatInput::ExitProcess);
    }
    Ok(ChatInput::from(line))
}

impl ChatInput {
    /// No input, [`ChatInput::None`].
    pub(crate) fn new() -> Self {
        Self::default()
    }
    /// Marks the input as handled by setting it to [`ChatInput::None`].
    pub(crate) fn clear(&mut self) {
        *self = Self::None;
    }
    /// Whether there is no input waiting.
    pub(crate) fn is_none(&self) -> bool {
        matches!(self, ChatInput::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> ChatInput {
        ChatInput::from(input)
    }

    #[test]
    fn each_command_parses_case_insensitively_and_trimmed() {
        type Case = (&'static str, fn(&ChatInput) -> bool);
        let cases: [Case; 10] = [
            ("/exit", |i| matches!(i, ChatInput::ExitProcess)),
            ("/history", |i| matches!(i, ChatInput::ShowChatHistory)),
            ("/save", |i| matches!(i, ChatInput::SaveChatHistory)),
            ("/tokens", |i| matches!(i, ChatInput::ShowTokenUsage)),
            ("/clear", |i| matches!(i, ChatInput::ClearContext)),
            ("/model", |i| matches!(i, ChatInput::SwitchModel)),
            ("/help", |i| matches!(i, ChatInput::ShowHelpMessage)),
            ("/summarize", |i| matches!(i, ChatInput::ShowContextSummary)),
            ("/compact", |i| matches!(i, ChatInput::CompactContext)),
            ("/import 7", |i| {
                matches!(i, ChatInput::ImportChatHistory(7))
            }),
        ];
        for (command, is) in cases {
            assert!(is(&parse(command)), "{command}");
            let shouted = format!("  {}\n", command.to_uppercase());
            assert!(is(&parse(&shouted)), "{shouted:?}");
        }
    }

    #[test]
    fn a_line_is_read_and_parsed() {
        let mut lines = "/help\nhello\n".as_bytes();
        assert!(matches!(
            read(&mut lines).unwrap(),
            ChatInput::ShowHelpMessage
        ));
        assert!(matches!(read(&mut lines).unwrap(), ChatInput::SendMessage(m) if m == "hello"));
    }

    #[test]
    fn the_end_of_input_exits() {
        let mut empty = "".as_bytes();
        assert!(matches!(read(&mut empty).unwrap(), ChatInput::ExitProcess));
    }

    #[test]
    fn blank_input_is_none() {
        assert!(parse("").is_none());
        assert!(parse("  \n").is_none());
    }

    #[test]
    fn import_without_an_id_is_a_message() {
        assert!(matches!(parse("/import"), ChatInput::SendMessage(m) if m == "/import"));
    }

    #[test]
    fn import_inside_a_message_is_a_message() {
        for message in ["how do I /import 3 files", "/importer 3", "/import 3 4"] {
            assert!(
                matches!(parse(message), ChatInput::SendMessage(ref m) if m == message),
                "{message}"
            );
        }
    }

    #[test]
    fn a_message_keeps_its_case() {
        assert!(
            matches!(parse("Explain Rust's Arc<T> to Scotty"), ChatInput::SendMessage(m) if m == "Explain Rust's Arc<T> to Scotty")
        );
    }

    #[test]
    fn anything_else_is_a_trimmed_message() {
        assert!(
            matches!(parse("  hello there\n"), ChatInput::SendMessage(m) if m == "hello there")
        );
    }
}
