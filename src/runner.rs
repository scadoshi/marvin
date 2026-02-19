use crate::chat::{input::Input, State};
use crate::ui::horizontal_line;
use crate::user_commands::{
    clear_context::ClearContext, compact_context::CompactContext, exit_process::ExitProcess,
    import_chat_history::ImportChatHistory, save_chat_history::SaveChatHistory,
    show_chat_history::ShowChatHistory, show_context_summary::ShowContextSummary,
    show_help_message::ShowHelpMessage, show_token_usage::ShowTokenUsage,
    switch_model::SwitchModel,
};
use rig::message::Message;

pub struct Runner;

impl Runner {
    pub async fn run(mut state: State) -> anyhow::Result<()> {
        println!("Agent: {}", state.model());
        horizontal_line();
        println!("Type a message and click enter to submit");
        loop {
            horizontal_line();
            if state.input().is_none() {
                state.get_input();
            }
            match state.input() {
                Input::ClearContext => {
                    state.clear_context()?;
                    continue;
                }
                Input::ShowHelpMessage => {
                    state.show_help_message();
                    continue;
                }
                Input::ShowChatHistory => {
                    state.show_chat_history();
                    continue;
                }
                Input::SaveChatHistory => {
                    state.save_chat_history()?;
                    continue;
                }
                Input::ImportChatHistory(id) => {
                    state.import_chat_history(*id);
                    continue;
                }
                Input::ShowTokenUsage => {
                    state.show_token_usage();
                    continue;
                }
                Input::SwitchModel => {
                    state.switch_model()?;
                    continue;
                }
                Input::ShowContextSummary => {
                    state.show_context_summary().await?;
                    continue;
                }
                Input::CompactContext => {
                    state.compact_context().await?;
                    continue;
                }
                Input::None => continue,
                Input::ExitProcess => {
                    state.exit_process()?;
                    break;
                }
                Input::SendMessage(message) => {
                    if message.is_empty() {
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
