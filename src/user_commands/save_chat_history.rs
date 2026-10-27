use crate::chat::{CHATS_DIR_NAME, Chat};
use crate::ui::horizontal_line;

impl Chat {
    pub(crate) fn save_chat_history(&mut self) -> anyhow::Result<()> {
        self.clear_input();
        self.save_chat_history_to_file()?;
        horizontal_line();
        println!(
            "Saved chat (ID = {}) history to the {}/ directory",
            self.id(),
            CHATS_DIR_NAME
        );
        Ok(())
    }
}
