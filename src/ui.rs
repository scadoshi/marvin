//! Terminal output shared by the chat and its commands.

#![expect(
    clippy::print_stdout,
    reason = "marvin is a terminal chat, so printing is its interface"
)]

use std::io::{self, Write};

/// Characters in a [`horizontal_line`].
const WIDTH: usize = 50;
/// What [`horizontal_line`] repeats.
const HORIZONTAL_LINE_STR: &str = "-";
/// Prints a divider [`WIDTH`] dashes wide.
pub(crate) fn horizontal_line() {
    println!("{}", HORIZONTAL_LINE_STR.repeat(WIDTH));
}

/// Greets the user and announces the new chat's ID, printing three dots half a second apart.
pub(crate) fn welcome_message(chat_id: u16) {
    println!("Hello, I am Marvin, your personal AI assistant!");
    print!("Initiating new chat state (ID = {chat_id})");
    for _ in 0..3 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        print!(".");
        let _ = io::stdout().flush();
    }
    println!();
    horizontal_line();
}
