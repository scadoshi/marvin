use crate::{chat, ui::horizontal_line, user_commands::save_chat_history::SaveChatHistory};

pub trait ExitProcess {
    fn exit_process(&mut self) -> anyhow::Result<()>;
}

impl ExitProcess for chat::State {
    fn exit_process(&mut self) -> anyhow::Result<()> {
        self.save_chat_history()?;
        horizontal_line();
        println!("Farewell!");
        Ok(())
    }
}
