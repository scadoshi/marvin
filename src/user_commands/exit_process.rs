use crate::chat::Chat;
use crate::ui;

impl Chat {
    pub(crate) fn exit_process(&mut self) -> anyhow::Result<()> {
        self.save_chat_history()?;
        ui::horizontal_line();
        println!("Farewell!");
        Ok(())
    }
}
