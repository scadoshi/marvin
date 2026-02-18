use std::io::{stdout, Write};

const WIDTH: usize = 50;
const HORIZONTAL_LINE_STR: &str = "-";
pub fn horizontal_line() {
    println!("{}", HORIZONTAL_LINE_STR.repeat(WIDTH));
}

pub fn welcome_message() -> anyhow::Result<()> {
    println!("Hello, I am Marvin, your personal AI assistant!");
    print!("Initiating new chat state");
    for _ in 0..3 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        print!(".");
        stdout().flush()?;
    }
    println!();
    horizontal_line();
    Ok(())
}
