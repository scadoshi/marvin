mod agent_tools;
mod anthropic;
mod chat;
mod runner;
mod ui;
mod user_commands;

use crate::{chat::Chat, runner::Runner};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = Chat::new().await?;
    Runner::run(state).await?;
    Ok(())
}
