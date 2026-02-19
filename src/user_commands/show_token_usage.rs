use crate::{
    chat::{self},
    ui::horizontal_line,
};

pub trait ShowTokenUsage {
    fn show_token_usage(&mut self);
}

impl ShowTokenUsage for chat::State {
    fn show_token_usage(&mut self) {
        self.clear_input();
        horizontal_line();
        println!(
            "Total Input Tokens Used: {}",
            self.total_input_tokens_used()
        );
        println!(
            "Total Output Tokens Used: {}",
            self.total_output_tokens_used()
        );
    }
}
