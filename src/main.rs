use crate::runner::Runner;

mod anthropic;
mod chat;
mod config;
mod runner;
mod ui;
mod user_commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = chat::State::new().await?;
    Runner::run(state).await?;
    Ok(())
}
