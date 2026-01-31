use dotenvy::dotenv;
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Chat;
use rig::message::Message;
use rig::providers::anthropic;
use rig::providers::anthropic::completion::CLAUDE_3_5_HAIKU;
use std::io::stdin;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let preamble = std::env::var("PREAMBLE").expect("PREAMBLE not found");
    let model = CLAUDE_3_5_HAIKU;
    let client = anthropic::Client::from_env();
    let mut chat_history = Vec::<Message>::new();
    let agent = client.agent(model).preamble(preamble.as_str()).build();
    let width = 100;
    println!("Your AI Agent ({}) Preamble: {}", model, preamble);
    println!("{}", "=".repeat(width));
    println!("Type a message and click enter to submit");
    let mut user_input = String::new();
    loop {
        println!("{}", "=".repeat(width));
        stdin().read_line(&mut user_input)?;
        if user_input.trim().contains("/exit") && !user_input.trim().contains("\\/exit") {
            if !chat_history.is_empty() {
                let exit_prompt = Message::assistant(
                    "The user is requesting to exit with the /exit command. Please bid them farewell in a way that reflects the contents of this chat.",
                );
                if let Ok(response) = agent.chat(exit_prompt.clone(), chat_history.clone()).await {
                    println!("{}", "=".repeat(width));
                    println!("{}: {}", model, response);
                    chat_history.push(exit_prompt);
                    chat_history.push(Message::assistant(response));
                }
            }
            break;
        }
        if user_input.trim().is_empty() {
            println!("Type a message and click enter to submit");
            continue;
        }
        let user_message = Message::user(&user_input);
        match agent.chat(user_message.clone(), chat_history.clone()).await {
            Ok(response) => {
                println!("{}", "=".repeat(width));
                println!("{}: {}", model, response);
                chat_history.push(user_message);
                chat_history.push(Message::assistant(response));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                println!("Please try again.");
            }
        }
        user_input.clear();
    }
    Ok(())
}
