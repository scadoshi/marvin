#[derive(Debug, Clone, Default)]
pub enum ChatInput {
    ExitProcess,
    ShowChatHistory,
    SaveChatHistory,
    ImportChatHistory(u16),
    ShowTokenUsage,
    ClearContext,
    SwitchModel,
    ShowHelpMessage,
    ShowContextSummary,
    CompactContext,
    SendMessage(String),
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
        } else if value.contains("/import")
            && let Some(id) = value.split_whitespace().find_map(|v| v.parse::<u16>().ok())
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

impl ChatInput {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn clear(&mut self) {
        *self = Self::None;
    }
    pub fn is_none(&self) -> bool {
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
        let cases: [(&str, fn(&ChatInput) -> bool); 10] = [
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
    fn blank_input_is_none() {
        assert!(parse("").is_none());
        assert!(parse("  \n").is_none());
    }

    #[test]
    fn import_without_an_id_is_a_message() {
        assert!(matches!(parse("/import"), ChatInput::SendMessage(m) if m == "/import"));
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
