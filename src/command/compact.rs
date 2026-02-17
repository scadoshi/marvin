use crate::{chat::State, command::AsyncCommand, ui::horizontal_line};
use rig::message::Message;

pub struct Compact;

impl AsyncCommand for Compact {
    async fn execute(state: &mut State) -> anyhow::Result<()> {
        horizontal_line();
        state.clear_input();
        let prompt = "Provide a concise context summary of our conversation that could be used to continue this chat. Include key decisions made, current task state, and any important details. Write it as a brief paragraph, not a list.";
        let response = state.send(Message::user(prompt)).await?;
        state.clear_history();
        state.add_to_history(Message::assistant(response.clone()));
        println!(
            "Compaction completed in {} words",
            response.split_whitespace().count()
        );
        Ok(())
    }
}
