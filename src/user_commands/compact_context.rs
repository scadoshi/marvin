//! `/compact`: replaces the history with a summary the model writes.

use rig::message::Message;

use crate::chat::Chat;
use crate::ui;

impl Chat {
    /// Asks the model to summarize the chat, then keeps only that summary as one assistant message.
    ///
    /// # Errors
    ///
    /// Returns an error if the summary request fails. The history is then kept, with the summary prompt added.
    pub(crate) async fn compact_context(&mut self) -> anyhow::Result<()> {
        ui::horizontal_line();
        self.clear_input();
        let prompt = "Provide a concise context summary of our conversation that could be used to continue this chat. Include key decisions made, current task state, and any important details. Write it as a brief paragraph, not a list.";
        let response = self.send(Message::user(prompt)).await?;
        self.clear_chat_history();
        self.add_to_chat_history(Message::assistant(response.clone()));
        println!(
            "Compaction completed in {} words",
            response.split_whitespace().count()
        );
        Ok(())
    }
}
