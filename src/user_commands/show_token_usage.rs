use crate::chat::Chat;
use crate::ui;

fn formatted(n: usize) -> String {
    let mut str = n.to_string();
    for i in (1..str.len()).rev().skip(2).step_by(3) {
        str.insert(i, ',');
    }
    str
}

impl Chat {
    pub(crate) fn show_token_usage(&mut self) {
        self.clear_input();
        ui::horizontal_line();
        println!(
            "Total Input Tokens Used: {}",
            formatted(self.total_input_tokens_used())
        );
        println!(
            "Total Output Tokens Used: {}",
            formatted(self.total_output_tokens_used())
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub(crate) fn test_formatted() {
        assert_eq!(formatted(234_567_890), "234,567,890".to_string());
    }

    #[test]
    pub(crate) fn formatted_groups_by_three_from_the_right() {
        for (n, want) in [
            (0, "0"),
            (999, "999"),
            (1_000, "1,000"),
            (12_345, "12,345"),
            (1_000_000, "1,000,000"),
        ] {
            assert_eq!(formatted(n), want, "{n}");
        }
    }
}
