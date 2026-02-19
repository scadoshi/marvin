use crate::{chat, ui::horizontal_line, user_commands::show_chat_history::HISTORY_LEN};

pub trait ShowHelpMessage {
    fn show_help_message(&mut self);
}

impl ShowHelpMessage for chat::State {
    fn show_help_message(&mut self) {
        self.clear_input();
        horizontal_line();
        println!("Commands:");
        println!("  /model     - switch between available Claude models");
        println!("  /tokens    - show token usage");
        println!(
            "  /history   - show last {} messages from chat history",
            HISTORY_LEN
        );
        println!("  /save      - save chat history to file");
        println!("  /import    - import chat history from a saved file");
        println!("  /summarize - ask the agent to summarize the conversation");
        println!("  /compact   - condense conversation history");
        println!("  /clear     - clear chat history");
        println!("  /help      - list all available commands");
        println!("  /exit      - print farewell message and end the session");
    }
}
