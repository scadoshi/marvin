use crate::chat;

pub trait ImportChatHistory {
    fn import_chat_history(&mut self, id: u16);
}

impl ImportChatHistory for chat::State {
    fn import_chat_history(&mut self, id: u16) {
        self.clear_input();
        self.append_history_from_file_infallible(id);
    }
}
