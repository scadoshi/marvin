use std::io::{self, Write};

const WIDTH: usize = 50;
const HORIZONTAL_LINE_STR: &str = "-";
pub(crate) fn horizontal_line() {
    println!("{}", HORIZONTAL_LINE_STR.repeat(WIDTH));
}

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
