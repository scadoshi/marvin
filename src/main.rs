//! marvin, a terminal chat client for Anthropic models with math and web tools the model can call.

mod agent_tools;
mod anthropic;
mod chat;
mod runner;
#[cfg(test)]
mod test_server;
mod ui;
mod user_commands;

use crate::chat::Chat;

/// Starts a chat and runs it until the user leaves.
///
/// # Errors
///
/// Returns an error if the chat cannot start or the loop stops on an error.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Some(chat) = Chat::new().await? else {
        return Ok(());
    };
    runner::run(chat).await
}
