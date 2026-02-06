#[derive(Debug, Clone, Default)]
pub enum Input {
    ExitCommand,
    HistoryCommand,
    ClearCommand,
    ModelCommand,
    HelpCommand,
    SummarizeCommand,
    Message(String),
    #[default]
    Empty,
}

impl<T> From<T> for Input
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let value = value.as_ref().trim().to_lowercase();
        if value == "/exit" {
            Self::ExitCommand
        } else if value == "/history" {
            Self::HistoryCommand
        } else if value == "/clear" {
            Self::ClearCommand
        } else if value == "/model" {
            Self::ModelCommand
        } else if value == "/help" {
            Self::HelpCommand
        } else if value == "/summarize" {
            Self::SummarizeCommand
        } else if value.is_empty() {
            Self::Empty
        } else {
            Self::Message(value.into())
        }
    }
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn clear(&mut self) {
        *self = Self::Empty;
    }
    pub fn is_empty(&self) -> bool {
        matches!(self, Input::Empty)
    }
}
