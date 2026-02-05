use dotenvy::dotenv;
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Chat;
use rig::message::{AssistantContent, Message, UserContent};
use rig::providers::anthropic;
use rig::providers::anthropic::completion::{
    CLAUDE_3_5_HAIKU, CLAUDE_3_5_SONNET, CLAUDE_3_7_SONNET, CLAUDE_4_OPUS, CLAUDE_4_SONNET,
};
use std::io::stdin;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let preamble = std::env::var("PREAMBLE").expect("PREAMBLE not found");
    let model_choices = [
        CLAUDE_3_5_HAIKU,
        CLAUDE_3_5_SONNET,
        CLAUDE_3_7_SONNET,
        CLAUDE_4_SONNET,
        CLAUDE_4_OPUS,
    ];
    let mut model = CLAUDE_3_5_HAIKU;
    let client = anthropic::Client::from_env();
    let mut chat_history = Vec::<Message>::new();
    let mut agent = client.agent(model).preamble(preamble.as_str()).build();
    let width = 100;
    println!("Your AI Agent ({}) Preamble: {}", model, preamble);
    println!("{}", "=".repeat(width));
    println!("Type a message and click enter to submit");
    let mut user_input = String::new();
    loop {
        println!("{}", "=".repeat(width));
        stdin().read_line(&mut user_input)?;

        // exit
        if user_input.trim().contains("/exit") {
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

        // switch model
        if user_input.trim().contains("/model") {
            println!("{}", "=".repeat(width));
            user_input.clear();
            println!("Choose a model!");
            for (i, model) in model_choices.iter().enumerate() {
                println!("    {}. {}", i, model);
            }
            println!("{}", "=".repeat(width));
            loop {
                println!("Make a selection:");
                stdin().read_line(&mut user_input)?;
                let Some((_, &selection)) = model_choices
                    .iter()
                    .enumerate()
                    .find(|(i, _)| user_input.trim().parse::<usize>().is_ok_and(|u| u == *i))
                else {
                    continue;
                };
                model = selection;
                agent = client.agent(model).preamble(preamble.as_str()).build();
                println!("Model updated to: {}", selection);
                user_input.clear();
                println!("{}", "=".repeat(width));
                break;
            }
            continue;
        }

        const TRUNCATE_AT: usize = 300;
        const HISTORY_LEN: usize = 10;
        // show history
        if user_input.trim().contains("/history") {
            if chat_history.is_empty() {
                user_input.clear();
                println!("{}", "=".repeat(width));
                println!("No chat history to show");
                continue;
            }
            println!("{}", "=".repeat(width));
            println!("Showing last {} messages", HISTORY_LEN);
            println!("{}", "=".repeat(width));
            user_input.clear();
            let history_to_show: Vec<_> =
                chat_history.iter().rev().take(HISTORY_LEN).rev().collect();
            for (i, message) in history_to_show.iter().enumerate() {
                match message {
                    Message::User { content } => match content.first() {
                        UserContent::Text(text) => {
                            let text = text.text();
                            let truncated = text.chars().take(TRUNCATE_AT).collect::<String>();
                            let end = if text.len() > TRUNCATE_AT {
                                String::from("...")
                            } else {
                                String::new()
                            };
                            println!("*User*: \"{}{}\"", truncated.trim(), end);
                        }
                        UserContent::Image(_) => println!("*User*: *image*"),
                        UserContent::Audio(_) => println!("*User*: *audio*"),
                        UserContent::Video(_) => println!("*User*: *video*"),
                        UserContent::Document(_) => println!("*User*: *document*"),
                        UserContent::ToolResult(_) => println!("*User*: *tool result*"),
                    },
                    Message::Assistant { content, .. } => match content.first() {
                        AssistantContent::Text(text) => {
                            let text = text.text();
                            let truncated = text.chars().take(TRUNCATE_AT).collect::<String>();
                            let end = if text.len() > TRUNCATE_AT {
                                String::from("...")
                            } else {
                                String::new()
                            };
                            println!("*Assistant*: \"{}{}\"", truncated.trim(), end);
                        }
                        AssistantContent::Image(_) => println!("*Assistant*: *image*"),
                        AssistantContent::ToolCall(_) => println!("*Assistant*: *tool call*"),
                        AssistantContent::Reasoning(_) => println!("*Assistant*: *reasoning*"),
                    },
                }
                if let Some(final_i) = history_to_show.len().checked_sub(1)
                    && i != final_i
                {
                    println!("---");
                }
                println!();
            }
            continue;
        }

        // clear chat history
        if user_input.trim() == "/clear" {
            println!("{}", "=".repeat(width));
            println!("Clearing chat history");
            chat_history.clear();
            user_input.clear();
            continue;
        }

        // help
        if user_input.trim() == "/help" {
            user_input.clear();
            println!("{}", "=".repeat(width));
            println!("Commands: ");
            println!("/model - switch models");
            println!("/history - show last {} messages", HISTORY_LEN);
            println!("/clear - clear context");
            println!("/exit - end application");
            continue;
        }

        // regular message
        if user_input.trim().is_empty() {
            user_input.clear();
            println!("Type a message and click enter to submit");
            continue;
        }
        let user_message = Message::user(&user_input);
        user_input.clear();
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
    }
    Ok(())
}
