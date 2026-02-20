use crate::{chat::Chat, ui::horizontal_line};
use rig::message::Message;

pub trait CompactContext {
    fn compact_context(&mut self) -> impl Future<Output = anyhow::Result<()>>;
}

impl CompactContext for Chat {
    async fn compact_context(&mut self) -> anyhow::Result<()> {
        horizontal_line();
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
