//! `/save`: writes the history to disk.

#![expect(
    clippy::print_stdout,
    reason = "marvin is a terminal chat, so printing is its interface"
)]

use crate::chat::{CHATS_DIR_NAME, Chat};
use crate::ui;

impl Chat {
    /// Saves the history to this chat's file and prints where.
    ///
    /// # Errors
    ///
    /// Returns an error if writing the file fails.
    pub(crate) fn save_chat_history(&mut self) -> anyhow::Result<()> {
        self.clear_input();
        self.save_chat_history_to_file()?;
        ui::horizontal_line();
        println!(
            "Saved chat (ID = {}) history to the {}/ directory",
            self.id(),
            CHATS_DIR_NAME
        );
        Ok(())
    }
}
