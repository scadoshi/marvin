use crate::{
    chat::{self, CHATS_DIR_NAME},
    ui::horizontal_line,
};

pub trait SaveChatHistory {
    fn save_chat_history(&mut self) -> anyhow::Result<()>;
}

impl SaveChatHistory for chat::State {
    fn save_chat_history(&mut self) -> anyhow::Result<()> {
        self.clear_input();
        self.save_history_to_file()?;
        horizontal_line();
        println!(
            "Saved chat (ID = {}) history to the {}/ directory",
            self.id(),
            CHATS_DIR_NAME
        );
        Ok(())
    }
}
