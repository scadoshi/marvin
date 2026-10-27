use crate::chat::Chat;

impl Chat {
    pub(crate) fn import_chat_history(&mut self, id: u16) {
        self.clear_input();
        self.append_chat_history_from_file_infallible(id);
    }
}
