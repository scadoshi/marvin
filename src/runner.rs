use crate::chat::{input::Input, State};
use crate::command::{
    clear::Clear, exit::Exit, help::Help, history::History, model::Model, summarize::Summarize,
    AsyncCommand, Command,
};
use crate::ui::horizontal_line;

pub struct Runner;

impl Runner {
    pub async fn run(mut state: State) -> anyhow::Result<()> {
        println!(
            "Your AI Agent ({}) Preamble: {}",
            state.model(),
            state.config().preamble()
        );
        horizontal_line();
        println!("Type a message and click enter to submit");
        loop {
            horizontal_line();
            if state.input().is_empty() {
                state.get_input();
            }
            match state.input() {
                Input::ClearCommand => {
                    Clear::execute(&mut state)?;
                    continue;
                }
                Input::HelpCommand => {
                    Help::execute(&mut state)?;
                    continue;
                }
                Input::HistoryCommand => {
                    History::execute(&mut state)?;
                    continue;
                }
                Input::ModelCommand => {
                    Model::execute(&mut state)?;
                    continue;
                }
                Input::SummarizeCommand => {
                    Summarize::execute(&mut state).await?;
                    continue;
                }
                Input::Empty => continue,
                Input::ExitCommand => {
                    Exit::execute(&mut state)?;
                    break;
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
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            println!("Please try again");
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
