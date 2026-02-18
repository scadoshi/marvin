use crate::chat::{input::Input, State};
use crate::command::clear_context::ClearContext;
use crate::command::compact_context::CompactContext;
use crate::command::exit_process::ExitProcess;
use crate::command::show_help_message::ShowHelpMessage;
use crate::command::show_message_history::ShowMessageHistory;
use crate::command::show_token_usage::ShowTokenUsage;
use crate::command::summarize_context::SummarizeContext;
use crate::command::switch_model::SwitchModel;
use crate::ui::horizontal_line;
use rig::message::Message;

pub struct Runner;

impl Runner {
    pub async fn run(mut state: State) -> anyhow::Result<()> {
        println!("Agent: {}", state.model());
        horizontal_line();
        println!("Type a message and click enter to submit");
        loop {
            horizontal_line();
            if state.input().is_empty() {
                state.get_input();
            }
            match state.input() {
                Input::ClearCommand => {
                    state.clear_context()?;
                    continue;
                }
                Input::HelpCommand => {
                    state.show_help_message();
                    continue;
                }
                Input::HistoryCommand => {
                    state.show_message_history();
                    continue;
                }
                Input::TokensCommand => {
                    state.show_token_usage();
                    continue;
                }
                Input::ModelCommand => {
                    state.switch_model()?;
                    continue;
                }
                Input::SummarizeCommand => {
                    state.summarize_context().await?;
                    continue;
                }
                Input::CompactCommand => {
                    state.compact_context().await?;
                    continue;
                }
                Input::Empty => continue,
                Input::ExitCommand => {
                    state.exit_process();
                    break;
                }
                Input::Message(message) => {
                    if state.input().is_empty() {
                        println!("Type a message and click enter");
                        state.clear_input();
                        continue;
                    }
                    let message = message.to_owned();
                    state.clear_input();
                    state.stream(Message::user(message)).await;
                }
            }
        }
        Ok(())
    }
}
