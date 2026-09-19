use crate::{
    chat::{Chat, ChatInput},
    ui::horizontal_line,
    user_commands::{
        ClearContext, CompactContext, ExitProcess, ImportChatHistory, SaveChatHistory,
        ShowChatHistory, ShowContextSummary, ShowHelpMessage, ShowTokenUsage, SwitchModel,
    },
};
use rig::message::Message;

pub struct Runner;

impl Runner {
    pub async fn run(mut chat: Chat) -> anyhow::Result<()> {
        println!("Agent: {}", chat.model());
        horizontal_line();
        println!("Type a message and click enter to submit");
        loop {
            horizontal_line();
            if chat.input().is_none() {
                chat.get_input();
            }
            match chat.input() {
                ChatInput::ClearContext => chat.clear_context()?,
                ChatInput::ShowHelpMessage => chat.show_help_message(),
                ChatInput::ShowChatHistory => chat.show_chat_history(),
                ChatInput::SaveChatHistory => chat.save_chat_history()?,
                ChatInput::ImportChatHistory(id) => chat.import_chat_history(*id),
                ChatInput::ShowTokenUsage => chat.show_token_usage(),
                ChatInput::SwitchModel => chat.switch_model()?,
                ChatInput::ShowContextSummary => chat.show_context_summary().await?,
                ChatInput::CompactContext => chat.compact_context().await?,
                ChatInput::None => {}
                ChatInput::ExitProcess => {
                    chat.exit_process()?;
                    break;
                }
                ChatInput::SendMessage(message) => {
                    if message.is_empty() {
                        println!("Type a message and click enter");
                        chat.clear_input();
                        continue;
                    }
                    let message = message.to_owned();
                    chat.clear_input();
                    chat.stream(Message::user(message)).await;
                }
            }
        }
        Ok(())
    }
}
