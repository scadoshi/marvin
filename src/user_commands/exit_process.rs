//! `/exit`: saves the chat, if anything was said, and says goodbye.

#![expect(
    clippy::print_stdout,
    reason = "marvin is a terminal chat, so printing is its interface"
)]

use crate::chat::Chat;
use crate::ui;

impl Chat {
    /// Saves the history, unless it is empty, and prints a farewell. The caller ends the loop.
    ///
    /// # Errors
    ///
    /// Returns an error if saving the history fails.
    pub(crate) fn exit_process(&mut self) -> anyhow::Result<()> {
        if !self.chat_history().is_empty() {
            self.save_chat_history()?;
        }
        ui::horizontal_line();
        println!("Farewell!");
        Ok(())
    }
}
