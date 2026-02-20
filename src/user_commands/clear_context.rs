use crate::{chat::Chat, ui::horizontal_line};

pub trait ClearContext {
    fn clear_context(&mut self) -> anyhow::Result<()>;
}

impl ClearContext for Chat {
    fn clear_context(&mut self) -> anyhow::Result<()> {
        horizontal_line();
        println!("Chat history cleared");
        self.clear_chat_history();
        self.clear_input();
        Ok(())
    }
}
