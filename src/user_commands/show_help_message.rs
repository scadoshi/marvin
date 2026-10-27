use crate::chat::Chat;
use crate::ui;
use crate::user_commands::show_chat_history::HISTORY_LEN;

impl Chat {
    pub(crate) fn show_help_message(&mut self) {
        self.clear_input();
        ui::horizontal_line();
        println!("Commands:");
        println!("  /model     - switch between available Claude models");
        println!("  /tokens    - show token usage");
        println!("  /history   - show last {HISTORY_LEN} messages from chat history");
        println!("  /save      - save chat history to file");
        println!("  /import    - import chat history from a saved file");
        println!("  /summarize - ask the agent to summarize the conversation");
        println!("  /compact   - condense conversation history");
        println!("  /clear     - clear chat history");
        println!("  /help      - list all available commands");
        println!("  /exit      - print farewell message and end the session");
    }
}
