//! The main loop: reads a line, then runs the slash command or sends the message to the model.

use rig::message::Message;

use crate::chat::{Chat, ChatInput};
use crate::ui;

/// Reads and handles input until `/exit` or the end of input.
///
/// # Errors
///
/// Returns an error if saving the history fails, on `/save` or `/exit`, or if `/compact` cannot get a summary from the model. A failed `/import` is printed instead.
pub(crate) async fn run(mut chat: Chat) -> anyhow::Result<()> {
    println!("Agent: {}", chat.model());
    ui::horizontal_line();
    println!("Type a message and click enter to submit");
    loop {
        ui::horizontal_line();
        if chat.input().is_none() {
            chat.read_input();
        }
        match chat.input() {
            ChatInput::ClearContext => chat.clear_context(),
            ChatInput::ShowHelpMessage => chat.show_help_message(),
            ChatInput::ShowChatHistory => chat.show_chat_history(),
            ChatInput::SaveChatHistory => chat.save_chat_history()?,
            ChatInput::ImportChatHistory(id) => {
                if let Err(e) = chat.import_chat_history(*id) {
                    println!("{e:#}");
                }
            }
            ChatInput::ShowTokenUsage => chat.show_token_usage(),
            ChatInput::SwitchModel => chat.switch_model(),
            ChatInput::ShowContextSummary => chat.show_context_summary().await,
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
