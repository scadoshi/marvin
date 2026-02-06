const WIDTH: usize = 100;
const HORIZONTAL_LINE_STR: &str = "=";

pub fn horizontal_line() {
    println!("{}", HORIZONTAL_LINE_STR.repeat(WIDTH));
}
