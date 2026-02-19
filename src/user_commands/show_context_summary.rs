use rig::message::Message;

use crate::chat;

pub trait ShowContextSummary {
    fn show_context_summary(&mut self) -> impl Future<Output = anyhow::Result<()>>;
}

impl ShowContextSummary for chat::State {
    async fn show_context_summary(&mut self) -> anyhow::Result<()> {
        self.clear_input();
        if self.history().is_empty() {
            println!("Nothing to summarize");
        } else {
            let prompt = "Summarize our conversation so far in 2-4 sentences. Focus on the key topics discussed and any conclusions reached.";
            self.stream(Message::user(prompt)).await;
        }
        Ok(())
    }
}
