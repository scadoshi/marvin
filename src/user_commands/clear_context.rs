use crate::chat::Chat;
use crate::ui;

impl Chat {
    pub(crate) fn clear_context(&mut self) -> anyhow::Result<()> {
        ui::horizontal_line();
        println!("Chat history cleared");
        self.clear_chat_history();
        self.clear_input();
        Ok(())
    }
}
