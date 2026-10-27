//! `/exit`: saves the chat and says goodbye.

use crate::chat::Chat;
use crate::ui;

impl Chat {
    /// Saves the history and prints a farewell. The caller ends the loop.
    ///
    /// # Errors
    ///
    /// Returns an error if saving the history fails.
    pub(crate) fn exit_process(&mut self) -> anyhow::Result<()> {
        self.save_chat_history()?;
        ui::horizontal_line();
        println!("Farewell!");
        Ok(())
    }
}
