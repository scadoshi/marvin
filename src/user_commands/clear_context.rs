//! `/clear`: empties the history.

use crate::chat::Chat;
use crate::ui;

impl Chat {
    /// Empties the history so the next message starts fresh.
    ///
    /// # Errors
    ///
    /// Never returns an error.
    pub(crate) fn clear_context(&mut self) -> anyhow::Result<()> {
        ui::horizontal_line();
        println!("Chat history cleared");
        self.clear_chat_history();
        self.clear_input();
        Ok(())
    }
}
