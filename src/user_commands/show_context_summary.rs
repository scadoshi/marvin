//! `/summarize`: asks the model to summarize the chat.

#![expect(
    clippy::print_stdout,
    reason = "marvin is a terminal chat, so printing is its interface"
)]

use rig::message::Message;

use crate::chat::Chat;

impl Chat {
    /// Streams a short summary of the chat from the model. The request and reply join the history. Stream errors are printed.
    pub(crate) async fn show_context_summary(&mut self) {
        self.clear_input();
        if self.chat_history().is_empty() {
            println!("Nothing to summarize");
        } else {
            let prompt = "Summarize our conversation so far in 2-4 sentences. Focus on the key topics discussed and any conclusions reached.";
            self.stream(Message::user(prompt)).await;
        }
    }
}
