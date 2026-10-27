//! `/clear`: empties the history.

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
