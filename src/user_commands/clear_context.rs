use crate::chat::Chat;
use crate::ui::horizontal_line;

impl Chat {
    pub(crate) fn clear_context(&mut self) -> anyhow::Result<()> {
        horizontal_line();
        println!("Chat history cleared");
        self.clear_chat_history();
        self.clear_input();
        Ok(())
    }
}
