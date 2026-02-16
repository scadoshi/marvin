use crate::{chat::State, command::Command, ui::horizontal_line};

pub struct Tokens;

impl Command for Tokens {
    fn execute(state: &mut State) -> anyhow::Result<()> {
        state.clear_input();
        horizontal_line();
        println!(
            "Total Input Tokens Used: {}",
            state.total_input_tokens_used()
        );
        println!(
            "Total Output Tokens Used: {}",
            state.total_output_tokens_used()
        );
        Ok(())
    }
}
