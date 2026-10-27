use crate::chat::Chat;
use crate::ui::horizontal_line;

pub trait ShowTokenUsage {
    fn show_token_usage(&mut self);
}

trait Formatted {
    fn formatted(self) -> String;
}

impl Formatted for usize {
    fn formatted(self) -> String {
        let mut str = self.to_string();
        for i in (1..str.len()).rev().skip(2).step_by(3) {
            str.insert(i, ',');
        }
        str
    }
}

impl ShowTokenUsage for Chat {
    fn show_token_usage(&mut self) {
        self.clear_input();
        horizontal_line();
        println!(
            "Total Input Tokens Used: {}",
            self.total_input_tokens_used().formatted()
        );
        println!(
            "Total Output Tokens Used: {}",
            self.total_output_tokens_used().formatted()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatted() {
        assert_eq!(234_567_890.formatted(), "234,567,890".to_string());
    }

    #[test]
    fn formatted_groups_by_three_from_the_right() {
        for (n, want) in [
            (0, "0"),
            (999, "999"),
            (1_000, "1,000"),
            (12_345, "12,345"),
            (1_000_000, "1,000,000"),
        ] {
            assert_eq!(n.formatted(), want, "{n}");
        }
    }
}
