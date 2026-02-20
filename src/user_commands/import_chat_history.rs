use crate::chat::Chat;

pub trait ImportChatHistory {
    fn import_chat_history(&mut self, id: u16);
}

impl ImportChatHistory for Chat {
    fn import_chat_history(&mut self, id: u16) {
        self.clear_input();
        self.append_chat_history_from_file_infallible(id);
    }
}
