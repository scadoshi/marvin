use crate::{chat, commands::Command, ui::horizontal_line};

pub struct Exit;

impl Command for Exit {
    async fn execute(state: &mut chat::State) -> anyhow::Result<()> {
        if !state.history().is_empty() {
            let exit_prompt = "The user is requesting to exit with the /exit command. Please bid them farewell in a way that reflects the contents of this chat";
            if let Ok(response) = state.send_assistant_message(exit_prompt).await {
                horizontal_line();
                println!("{}: {}", state.model(), response);
                state.record_assistant_message(exit_prompt);
                state.record_assistant_message(response);
            }
        }
        Ok(())
    }
}
