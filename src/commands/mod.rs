pub mod clear;
pub mod exit;
pub mod help;
pub mod history;
pub mod model;
pub mod summarize;

use crate::chat;

pub trait Command {
    fn execute(state: &mut chat::State) -> impl Future<Output = anyhow::Result<()>>;
}
