use crate::{
    commands::{history::HISTORY_LEN, Command},
    ui::horizontal_line,
};

pub struct Help;

impl Command for Help {
    async fn execute(state: &mut crate::chat::State) -> anyhow::Result<()> {
        state.clear_input();
        horizontal_line();
        println!("Commands: ");
        println!("/model - switch models");
        println!("/history - show last {} messages", HISTORY_LEN);
        println!("/summarize - summarize chat history");
        println!("/clear - clear context");
        println!("/exit - end application");
        Ok(())
    }
}
