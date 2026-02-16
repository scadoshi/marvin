use crate::command::AsyncCommand;
use rig::message::Message;

pub struct Summarize;

impl AsyncCommand for Summarize {
    async fn execute(state: &mut crate::chat::State) -> anyhow::Result<()> {
        state.clear_input();
        if state.history().is_empty() {
            println!("Nothing to summarize");
        } else {
            let summarize_prompt = "Summarize our conversation so far in 2-4 sentences. Focus on the key topics discussed and any conclusions reached.";
            state.stream(Message::user(summarize_prompt)).await;
        }
        Ok(())
    }
}
