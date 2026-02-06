use crate::chat::input::Input;
use crate::chat::State;
use crate::commands::clear::Clear;
use crate::commands::exit::Exit;
use crate::commands::help::Help;
use crate::commands::history::History;
use crate::commands::model::Model;
use crate::commands::summarize::Summarize;
use crate::commands::Command;
use crate::ui::horizontal_line;

pub struct Runner;

impl Runner {
    pub async fn run(mut state: State) -> anyhow::Result<()> {
        // testing
        for _ in 0..5 {
            state.record_user_message("Testing some messages back and forth");
            state.record_assistant_message(
                "Sounds good. Continue testing or go ahead and ask a question",
            );
        }
        // testing
        println!(
            "Your AI Agent ({}) Preamble: {}",
            state.model(),
            state.config().preamble()
        );
        horizontal_line();
        println!("Type a message and click enter to submit");
        loop {
            horizontal_line();
            state.get_input();
            if !state.input().is_empty() {
                match state.input() {
                    Input::Empty => continue,
                    Input::ExitCommand => {
                        Exit::execute(&mut state).await?;
                        break;
                    }
                    Input::HelpCommand => {
                        Help::execute(&mut state).await?;
                        continue;
                    }
                    Input::SummarizeCommand => {
                        Summarize::execute(&mut state).await?;
                        continue;
                    }
                    Input::ModelCommand => {
                        Model::execute(&mut state).await?;
                        continue;
                    }
                    Input::ClearCommand => {
                        Clear::execute(&mut state).await?;
                        continue;
                    }
                    Input::HistoryCommand => {
                        History::execute(&mut state).await?;
                        continue;
                    }
                    Input::Message(message) => {
                        let message = message.to_owned();
                        if state.input().is_empty() {
                            println!("Type a message and click enter");
                            continue;
                        }
                        state.clear_input();
                        match state.send_user_message(message.clone()).await {
                            Ok(response) => {
                                horizontal_line();
                                println!("{}: {}", state.model(), response);
                                state.record_user_message(message);
                                state.record_assistant_message(response);
                            }
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                println!("Please try again");
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
