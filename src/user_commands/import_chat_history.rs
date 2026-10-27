//! `/import <id>`: loads a saved chat.

#![expect(
    clippy::print_stdout,
    reason = "marvin is a terminal chat, so printing is its interface"
)]

use anyhow::Context;
use rig::message::Message;

use crate::chat::{CHATS_DIR_NAME, Chat};

impl Chat {
    /// Appends the messages saved under chat `id` to the history.
    ///
    /// # Errors
    ///
    /// Returns an error if `<CHATS_DIR_NAME>/<id>.json` cannot be read or does not hold a list of messages.
    pub(crate) fn import_chat_history(&mut self, id: u16) -> anyhow::Result<()> {
        self.clear_input();
        let file_path = format!("{CHATS_DIR_NAME}/{id}.json");
        let file_str = std::fs::read_to_string(file_path)
            .with_context(|| format!("Failed to get chat_history with ID: {id}"))?;
        println!("chat_history with ID: {id} found!");
        let chat_history = serde_json::from_str::<Vec<Message>>(&file_str)
            .context("Failed to deserialize file to `Vec<Message>`")?;
        for message in chat_history {
            self.add_to_chat_history(message);
        }
        Ok(())
    }
}
