//! `/model`: switches the chat to another model.

#![expect(
    clippy::print_stdout,
    reason = "marvin is a terminal chat, so printing is its interface"
)]

use std::io::BufRead;

use crate::chat::Chat;
use crate::chat::input::ChatInput;
use crate::ui;

impl Chat {
    /// Lists the models and rebuilds the agent on the one picked by number, read from `reader`.
    ///
    /// Another message or an out-of-range number asks again. A blank line or a slash command leaves the model as it is, and the command stays as input for the main loop.
    pub(crate) fn switch_model(&mut self, reader: &mut impl BufRead) {
        ui::horizontal_line();
        self.clear_input();
        println!("Current model: {}", self.model());
        for (number, model) in (1..).zip(self.model_options()) {
            println!("{number}. {}", model.display_name);
        }
        ui::horizontal_line();
        loop {
            println!("Select a model");
            ui::horizontal_line();
            self.read_input(reader);
            match self.input() {
                ChatInput::SendMessage(message) => {
                    let Some(selection) = self
                        .model_options()
                        .iter()
                        .enumerate()
                        .find(|(i, _)| {
                            message
                                .parse::<usize>()
                                .ok()
                                .and_then(|u| u.checked_sub(1))
                                .is_some_and(|u| u == *i)
                        })
                        .map(|(_, selection)| selection.to_owned())
                    else {
                        continue;
                    };
                    self.clear_input();
                    self.set_agent(&selection);
                    println!("Model updated: {}", self.model());
                    break;
                }
                _ => break, // any other command should go to main loop for triaging
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::anthropic::ModelInfo;
    use crate::chat::input::ChatInput;
    use crate::chat::tests::{chat_against, test_model};

    fn chat_with_two_models() -> crate::chat::Chat {
        let mut chat = chat_against("http://127.0.0.1:9");
        chat.set_model_options(vec![
            test_model(),
            ModelInfo {
                id: String::from("claude-second"),
                display_name: String::from("Claude Second"),
                ..test_model()
            },
        ]);
        chat
    }

    #[tokio::test]
    async fn a_number_switches_the_model_after_bad_tries() {
        let mut chat = chat_with_two_models();
        chat.switch_model(&mut "9\nfoo\n2\n".as_bytes());
        assert_eq!(chat.model(), "Claude Second");
        assert!(chat.input().is_none());
    }

    #[tokio::test]
    async fn a_command_leaves_the_model_and_goes_to_the_main_loop() {
        let mut chat = chat_with_two_models();
        chat.switch_model(&mut "/help\n".as_bytes());
        assert_eq!(chat.model(), "Claude Test");
        assert!(matches!(chat.input(), ChatInput::ShowHelpMessage));
    }

    #[tokio::test]
    async fn the_end_of_input_leaves_the_model_and_exits() {
        let mut chat = chat_with_two_models();
        chat.switch_model(&mut "".as_bytes());
        assert_eq!(chat.model(), "Claude Test");
        assert!(matches!(chat.input(), ChatInput::ExitProcess));
    }
}
