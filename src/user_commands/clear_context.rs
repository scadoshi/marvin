//! `/clear`: empties the history.

#![expect(
    clippy::print_stdout,
    reason = "marvin is a terminal chat, so printing is its interface"
)]

use crate::chat::Chat;
use crate::ui;

impl Chat {
    /// Empties the history so the next message starts fresh.
    pub(crate) fn clear_context(&mut self) {
        ui::horizontal_line();
        println!("Chat history cleared");
        self.clear_chat_history();
        self.clear_input();
    }
}
