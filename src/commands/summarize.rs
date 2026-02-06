use crate::{commands::Command, ui::horizontal_line};

pub struct Summarize;

impl Command for Summarize {
    async fn execute(state: &mut crate::chat::State) -> anyhow::Result<()> {
        state.clear_input();
        if !state.history().is_empty() {
            let summarize_prompt = "The user is requesting a summary of the conversation up to this point with the /summary command. Please respond with a brief summary of the conversation";
            match state.send_assistant_message(summarize_prompt).await {
                Ok(response) => {
                    horizontal_line();
                    println!("{}: {}", state.model(), response);
                    state.record_assistant_message(summarize_prompt);
                    state.record_assistant_message(response);
                }
                Err(e) => {
                    eprint!("Error: {}", e);
                    println!("Please try again");
                }
            }
        } else {
            println!("Nothing to summarize");
        }
        Ok(())
    }
}
