mod agent_tools;
mod anthropic;
mod chat;
mod runner;
#[cfg(test)]
mod test_server;
mod ui;
mod user_commands;

use crate::chat::Chat;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Some(chat) = Chat::new().await? else {
        return Ok(());
    };
    runner::run(chat).await
}
