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
        let value = value.as_ref().trim().to_lowercase();
        if value == "/exit" {
            Self::ExitProcess
        } else if value == "/history" {
            Self::ShowChatHistory
        } else if value == "/save" {
            Self::SaveChatHistory
        } else if value.contains("/import")
            && let Some(id) = value
                .split_whitespace()
                .flat_map(|v| v.parse::<u16>().ok())
                .next()
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
            Self::SendMessage(value)
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
