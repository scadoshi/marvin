use crate::chat::Chat;
use crate::ui::horizontal_line;

impl Chat {
    pub(crate) fn exit_process(&mut self) -> anyhow::Result<()> {
        self.save_chat_history()?;
        horizontal_line();
        println!("Farewell!");
        Ok(())
    }
}
