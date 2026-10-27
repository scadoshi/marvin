//! The chat session: the agent, the message history and token counts, and streaming replies to the terminal.

#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "marvin is a terminal chat, so printing is its interface"
)]

pub(crate) mod config;
pub(crate) mod input;
use std::collections::HashSet;
use std::fmt;
use std::io::BufRead;
use std::sync::Arc;

use futures::StreamExt;
pub(crate) use input::ChatInput;
use rig::agent::{Agent, MultiTurnStreamItem};
use rig::client::CompletionClient;
use rig::completion::Chat as ChatTrait;
use rig::message::Message;
use rig::providers::anthropic::Client;
use rig::providers::anthropic::completion::CompletionModel;
use rig::providers::anthropic::streaming::{PartialUsage, StreamingCompletionResponse};
use rig::streaming::{StreamedAssistantContent, StreamingChat};

use crate::agent_tools::web::tavily::TavilyClient;
use crate::agent_tools::{math, web};
use crate::anthropic::{self, ModelInfo};
use crate::chat::config::Config;
use crate::ui;

/// System prompt for the agent, read from `preamble.txt` at compile time.
pub(crate) static PREAMBLE: &str = include_str!("preamble.txt");

/// A chat session with one model at a time and the messages so far.
pub(crate) struct Chat {
    id: u16,
    client: Client,
    model_options: Vec<ModelInfo>,
    agent: Agent<CompletionModel>,
    history: Vec<Message>,
    input: ChatInput,
    total_input_tokens_used: usize,
    total_output_tokens_used: usize,
    tavily_client: Arc<TavilyClient>,
}

/// Directory, relative to the working directory, that holds saved chats as `<id>.json`.
pub(crate) const CHATS_DIR_NAME: &str = "chats";
/// Lowest ID with no saved chat in [`CHATS_DIR_NAME`], creating the directory if needed.
///
/// # Errors
///
/// Returns an error if the directory cannot be created or read, or if every ID up to `u16::MAX` is taken.
fn next_chat_id() -> anyhow::Result<u16> {
    std::fs::create_dir_all(CHATS_DIR_NAME)?;
    let existing_chat_ids: HashSet<u16> = std::fs::read_dir(CHATS_DIR_NAME)?
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .filter(|ent| ent.path().extension().and_then(|ostr| ostr.to_str()) == Some("json"))
        .filter_map(|ent| {
            ent.path()
                .file_prefix()
                .and_then(|prfx| prfx.to_str())
                .and_then(|str| str.parse::<u16>().ok())
        })
        .collect();
    match (0..=u16::MAX).find(|id| !existing_chat_ids.contains(id)) {
        Some(id) => Ok(id),
        None => Err(anyhow::anyhow!(
            "Chat count has hit its limit of {}",
            u16::MAX
        )),
    }
}

/// Marvin on `model`, with the math and web tools.
fn agent(
    client: &Client,
    model: &ModelInfo,
    tavily_client: &Arc<TavilyClient>,
) -> Agent<CompletionModel> {
    client
        .agent(&model.id)
        .name("Marvin")
        .preamble(PREAMBLE)
        .max_tokens(model.max_tokens)
        .tools(math::tools())
        .tools(web::tools(tavily_client))
        .default_max_turns(100)
        .build()
}

/// Prints the chat's ID, model and history length, not its agent or client.
impl fmt::Debug for Chat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Chat")
            .field("id", &self.id)
            .field("model", &self.model())
            .field("messages", &self.history.len())
            .finish_non_exhaustive()
    }
}

/// Asks for a model by its number until one is picked. `None` on `/exit` or the end of input.
///
/// # Errors
///
/// Returns an error if reading from `reader` fails.
fn select_model<'models>(
    reader: &mut impl BufRead,
    model_options: &'models [ModelInfo],
) -> std::io::Result<Option<&'models ModelInfo>> {
    let mut input = String::new();
    loop {
        println!("Select a model");
        ui::horizontal_line();
        input.clear();
        if reader.read_line(&mut input)? == 0 || input.trim() == "/exit" {
            return Ok(None);
        }
        if let Some(model) = input
            .trim()
            .parse::<usize>()
            .ok()
            .and_then(|n| n.checked_sub(1))
            .and_then(|i| model_options.get(i))
        {
            return Ok(Some(model));
        }
    }
}

impl Chat {
    /// Starts a chat: loads the config and models and asks which model to use. `None` when the user leaves instead.
    ///
    /// # Errors
    ///
    /// Returns an error if no chat ID is free, a key is missing, the models cannot be listed, the client cannot be built, or reading the choice from stdin fails.
    pub(crate) async fn new() -> anyhow::Result<Option<Self>> {
        let id = next_chat_id()?;
        ui::welcome_message(id);
        let config = Config::from_env()?;
        let tavily_client = Arc::new(TavilyClient::from(&config));
        let model_options =
            anthropic::fetch_models(anthropic::API_BASE, config.anthropic_api_key()).await?;
        let client = Client::new(config.anthropic_api_key())?;
        println!("Current model: not set");
        for (number, model) in (1..).zip(&model_options) {
            println!("{number}. {}", model.display_name);
        }
        ui::horizontal_line();
        let Some(model) = select_model(&mut std::io::stdin().lock(), &model_options)? else {
            println!("Farewell!");
            return Ok(None);
        };
        let agent = self::agent(&client, model, &tavily_client);
        ui::horizontal_line();
        Ok(Some(Self {
            id,
            client,
            model_options,
            agent,
            history: Vec::new(),
            input: ChatInput::new(),
            total_input_tokens_used: 0,
            total_output_tokens_used: 0,
            tavily_client,
        }))
    }
    /// This chat's ID, which also names its save file.
    pub(crate) fn id(&self) -> u16 {
        self.id
    }
    /// Replaces the models offered by `/model`.
    #[cfg(test)]
    pub(crate) fn set_model_options(&mut self, model_options: Vec<ModelInfo>) {
        self.model_options = model_options;
    }
    /// Models the API key can use, in the order the API listed them.
    pub(crate) fn model_options(&self) -> &[ModelInfo] {
        self.model_options.as_slice()
    }
    /// Display name of the agent's model, or an empty string if it is not among [`Chat::model_options`].
    pub(crate) fn model(&self) -> &str {
        self.model_options()
            .iter()
            .find(|model| model.id == self.agent.model.model)
            .map_or("", |model| model.display_name.as_str())
    }
    /// Rebuilds the agent on `model`. The history is kept.
    pub(crate) fn set_agent(&mut self, model: &ModelInfo) {
        self.agent = agent(&self.client, model, &self.tavily_client);
    }
    /// Sends `message` with the history and returns the whole reply. Both join the history once the reply arrives.
    ///
    /// # Errors
    ///
    /// Returns an error if the model request fails. The history is then unchanged.
    pub(crate) async fn send(&mut self, message: impl Into<Message>) -> anyhow::Result<String> {
        let message = message.into();
        let history = self.chat_history().to_owned();
        let response = self.agent.chat(message.clone(), history).await?;
        self.add_to_chat_history(message);
        self.add_to_chat_history(Message::assistant(response.clone()));
        Ok(response)
    }
    /// Sends `message` with the history and prints the reply as it streams.
    ///
    /// The message joins the history before the request and the final reply after it. Token usage is added to the totals. Stream errors are printed, not returned.
    pub(crate) async fn stream(&mut self, message: impl Into<Message>) {
        ui::horizontal_line();
        let message: Message = message.into();
        let history = self.chat_history().to_owned();
        self.add_to_chat_history(message.clone());
        let mut stream = self.agent.stream_chat(message, history).await;
        while let Some(result) = stream.next().await {
            match result {
                Ok(MultiTurnStreamItem::FinalResponse(final_response)) => {
                    self.history
                        .push(Message::assistant(final_response.response()));
                }
                Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(
                    text,
                ))) if !text.text().trim().is_empty() => {
                    print!("{}", text.text());
                }
                Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Final(
                    StreamingCompletionResponse {
                        usage:
                            PartialUsage {
                                output_tokens,
                                input_tokens,
                            },
                    },
                ))) => {
                    println!();
                    self.add_output_tokens_used(output_tokens);
                    if let Some(input_tokens) = input_tokens {
                        self.add_input_tokens_used(input_tokens);
                    }
                }
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Stream Error: {e}");
                }
            }
        }
    }
    /// Messages so far, oldest first.
    pub(crate) fn chat_history(&self) -> &[Message] {
        self.history.as_slice()
    }
    /// Drops every message.
    pub(crate) fn clear_chat_history(&mut self) {
        self.history.clear();
    }
    /// Appends `message` to the history.
    pub(crate) fn add_to_chat_history(&mut self, message: impl Into<Message>) {
        self.history.push(message.into());
    }
    /// Writes the history as pretty JSON to `<CHATS_DIR_NAME>/<id>.json`, replacing any earlier save.
    ///
    /// # Errors
    ///
    /// Returns an error if the history cannot be serialized or the file cannot be created or written.
    pub(crate) fn save_chat_history_to_file(&self) -> anyhow::Result<()> {
        let file_path = format!("{}/{}.json", CHATS_DIR_NAME, self.id());
        let file = std::fs::File::create(file_path)?;
        let chat_history_json = serde_json::to_value(self.chat_history())?;
        serde_json::to_writer_pretty(file, &chat_history_json)?;
        Ok(())
    }
    /// The input waiting to be handled.
    pub(crate) fn input(&self) -> &ChatInput {
        &self.input
    }
    /// Reads a line from `reader` as the next input. A read error is printed and leaves no input.
    pub(crate) fn read_input(&mut self, reader: &mut impl BufRead) {
        match input::read(reader) {
            Ok(input) => self.input = input,
            Err(e) => {
                eprintln!("Error: {e}");
                println!("Input failed");
                self.clear_input();
            }
        }
    }
    /// Marks the input as handled.
    pub(crate) fn clear_input(&mut self) {
        self.input.clear();
    }
    /// Input tokens reported across every reply so far.
    pub(crate) fn total_input_tokens_used(&self) -> usize {
        self.total_input_tokens_used
    }
    /// Adds to the input token total, saturating at `usize::MAX`.
    pub(crate) fn add_input_tokens_used(&mut self, input_tokens: usize) {
        self.total_input_tokens_used = self.total_input_tokens_used.saturating_add(input_tokens);
    }
    /// Output tokens reported across every reply so far.
    pub(crate) fn total_output_tokens_used(&self) -> usize {
        self.total_output_tokens_used
    }
    /// Adds to the output token total, saturating at `usize::MAX`.
    pub(crate) fn add_output_tokens_used(&mut self, output_tokens: usize) {
        self.total_output_tokens_used = self.total_output_tokens_used.saturating_add(output_tokens);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::test_server::{self, Reply};

    pub(crate) fn test_model() -> ModelInfo {
        ModelInfo {
            id: String::from("claude-test"),
            display_name: String::from("Claude Test"),
            max_tokens: 1024,
        }
    }

    pub(crate) fn chat_against(base_url: &str) -> Chat {
        let client = Client::builder()
            .api_key("test")
            .base_url(base_url)
            .build()
            .unwrap();
        let tavily_client = Arc::new(TavilyClient::new("test"));
        let model = test_model();
        Chat {
            id: 0,
            agent: agent(&client, &model, &tavily_client),
            client,
            model_options: vec![model],
            history: Vec::new(),
            input: ChatInput::new(),
            total_input_tokens_used: 0,
            total_output_tokens_used: 0,
            tavily_client,
        }
    }

    const REJECTED: &str =
        r#"{"type":"error","error":{"type":"invalid_request_error","message":"test"}}"#;

    fn sent_messages(request: &str) -> Vec<serde_json::Value> {
        let body: serde_json::Value = serde_json::from_str(request).unwrap();
        body["messages"].as_array().unwrap().clone()
    }

    const STREAMED_TEXT: &str = r#"event: message_start
data: {"type":"message_start","message":{"id":"msg_1","type":"message","role":"assistant","content":[],"model":"claude-test","stop_reason":null,"stop_sequence":null,"usage":{"input_tokens":12,"output_tokens":1}}}

event: content_block_start
data: {"type":"content_block_start","index":0,"content_block":{"type":"text","text":""}}

event: content_block_delta
data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hi there"}}

event: content_block_stop
data: {"type":"content_block_stop","index":0}

event: message_delta
data: {"type":"message_delta","delta":{"stop_reason":"end_turn","stop_sequence":null},"usage":{"output_tokens":5}}

event: message_stop
data: {"type":"message_stop"}

"#;

    fn text_of(message: &Message) -> String {
        serde_json::to_value(message).unwrap().to_string()
    }

    #[tokio::test]
    async fn a_streamed_reply_joins_the_history_and_counts_its_tokens() {
        let (base_url, requests) = test_server::serve(vec![Reply::events(STREAMED_TEXT)]).await;
        let mut chat = chat_against(&base_url);
        chat.stream(Message::user("hello")).await;
        test_server::served(requests).await;
        let history = chat.chat_history();
        assert_eq!(history.len(), 2, "{history:#?}");
        assert!(text_of(&history[0]).contains("hello"));
        assert!(matches!(history[1], Message::Assistant { .. }));
        assert!(text_of(&history[1]).contains("Hi there"), "{history:#?}");
        assert_eq!(chat.total_output_tokens_used(), 5);
        assert_eq!(chat.total_input_tokens_used(), 12);
    }

    const CALLS_ADD: &str = r#"event: message_start
data: {"type":"message_start","message":{"id":"msg_a","type":"message","role":"assistant","content":[],"model":"claude-test","stop_reason":null,"stop_sequence":null,"usage":{"input_tokens":10,"output_tokens":1}}}

event: content_block_start
data: {"type":"content_block_start","index":0,"content_block":{"type":"tool_use","id":"toolu_a","name":"add","input":{}}}

event: content_block_delta
data: {"type":"content_block_delta","index":0,"delta":{"type":"input_json_delta","partial_json":"{\"lhs\":2,\"rhs\":3}"}}

event: content_block_stop
data: {"type":"content_block_stop","index":0}

event: message_delta
data: {"type":"message_delta","delta":{"stop_reason":"tool_use","stop_sequence":null},"usage":{"output_tokens":3}}

event: message_stop
data: {"type":"message_stop"}

"#;

    const CALLS_MULTIPLY: &str = r#"event: message_start
data: {"type":"message_start","message":{"id":"msg_m","type":"message","role":"assistant","content":[],"model":"claude-test","stop_reason":null,"stop_sequence":null,"usage":{"input_tokens":10,"output_tokens":1}}}

event: content_block_start
data: {"type":"content_block_start","index":0,"content_block":{"type":"tool_use","id":"toolu_m","name":"multiply","input":{}}}

event: content_block_delta
data: {"type":"content_block_delta","index":0,"delta":{"type":"input_json_delta","partial_json":"{\"lhs\":5,\"rhs\":4}"}}

event: content_block_stop
data: {"type":"content_block_stop","index":0}

event: message_delta
data: {"type":"message_delta","delta":{"stop_reason":"tool_use","stop_sequence":null},"usage":{"output_tokens":3}}

event: message_stop
data: {"type":"message_stop"}

"#;

    #[tokio::test]
    async fn tool_calls_run_and_their_results_go_back_to_the_model() {
        let (base_url, requests) = test_server::serve(vec![
            Reply::events(CALLS_ADD),
            Reply::events(CALLS_MULTIPLY),
            Reply::events(STREAMED_TEXT),
        ])
        .await;
        let mut chat = chat_against(&base_url);
        chat.stream(Message::user("what is (2 + 3) * 4")).await;
        let requests = test_server::served(requests).await;
        assert_eq!(requests.len(), 3);
        let after_add = sent_messages(&requests[1].body);
        let after_multiply = sent_messages(&requests[2].body);
        let last = |messages: &[serde_json::Value]| messages.last().unwrap().to_string();
        assert!(
            last(&after_add).contains("tool_result") && last(&after_add).contains("toolu_a"),
            "{after_add:#?}"
        );
        assert!(last(&after_add).contains('5'), "{after_add:#?}");
        assert!(
            last(&after_multiply).contains("toolu_m") && last(&after_multiply).contains("20"),
            "{after_multiply:#?}"
        );
        let history = chat.chat_history();
        assert!(
            text_of(history.last().unwrap()).contains("Hi there"),
            "{history:#?}"
        );
    }

    const SUMMARY: &str = r#"{"id":"msg_s","type":"message","role":"assistant","model":"claude-test","content":[{"type":"text","text":"We talked about Rust."}],"stop_reason":"end_turn","stop_sequence":null,"usage":{"input_tokens":20,"output_tokens":6}}"#;

    #[tokio::test]
    async fn compacting_replaces_the_history_with_the_summary() {
        let (base_url, requests) = test_server::serve(vec![Reply::json(200, SUMMARY)]).await;
        let mut chat = chat_against(&base_url);
        chat.add_to_chat_history(Message::user("tell me about Rust"));
        chat.add_to_chat_history(Message::assistant("Rust is a language."));
        chat.compact_context().await.unwrap();
        let requests = test_server::served(requests).await;
        let sent = sent_messages(&requests[0].body);
        assert_eq!(
            sent.len(),
            3,
            "the history and the summarize prompt: {sent:#?}"
        );
        let history = chat.chat_history();
        assert_eq!(history.len(), 1, "{history:#?}");
        assert!(matches!(history[0], Message::Assistant { .. }));
        assert!(text_of(&history[0]).contains("We talked about Rust."));
    }

    #[tokio::test]
    async fn a_saved_chat_imports_into_another() {
        let mut saved = chat_against("http://127.0.0.1:9");
        saved.id = u16::MAX - 1;
        saved.add_to_chat_history(Message::user("remember this"));
        saved.add_to_chat_history(Message::assistant("remembered"));
        std::fs::create_dir_all(CHATS_DIR_NAME).unwrap();
        saved.save_chat_history_to_file().unwrap();
        let mut imported = chat_against("http://127.0.0.1:9");
        imported.add_to_chat_history(Message::user("already here"));
        let result = imported.import_chat_history(saved.id);
        let _ = std::fs::remove_file(format!("{CHATS_DIR_NAME}/{}.json", saved.id));
        result.unwrap();
        let texts: Vec<String> = imported.chat_history().iter().map(text_of).collect();
        assert_eq!(texts.len(), 3, "{texts:#?}");
        assert!(texts[0].contains("already here"));
        assert_eq!(
            texts[1..],
            saved.chat_history().iter().map(text_of).collect::<Vec<_>>()[..]
        );
    }

    #[tokio::test]
    async fn importing_a_missing_chat_is_an_error_naming_it() {
        let mut chat = chat_against("http://127.0.0.1:9");
        let err = chat.import_chat_history(u16::MAX - 2).unwrap_err();
        assert!(
            format!("{err:#}").contains(&format!("ID: {}", u16::MAX - 2)),
            "{err:#}"
        );
        assert_eq!(chat.chat_history().len(), 0);
    }

    #[tokio::test]
    async fn a_streamed_message_reaches_the_model_once() {
        let (base_url, request) = test_server::serve_once(400, REJECTED).await;
        let mut chat = chat_against(&base_url);
        chat.stream(Message::user("hello")).await;
        let messages = sent_messages(&test_server::served(request).await.body);
        assert_eq!(messages.len(), 1, "{messages:#?}");
        assert_eq!(chat.chat_history().len(), 1, "the message is still kept");
    }

    #[tokio::test]
    async fn a_sent_message_reaches_the_model_once() {
        let (base_url, request) = test_server::serve_once(400, REJECTED).await;
        let mut chat = chat_against(&base_url);
        assert!(chat.send(Message::user("hello")).await.is_err());
        let messages = sent_messages(&test_server::served(request).await.body);
        assert_eq!(messages.len(), 1, "{messages:#?}");
    }

    #[tokio::test]
    async fn exiting_an_empty_chat_saves_no_file() {
        let mut chat = chat_against("http://127.0.0.1:9");
        chat.id = u16::MAX;
        let path = format!("{CHATS_DIR_NAME}/{}.json", chat.id);
        std::fs::create_dir_all(CHATS_DIR_NAME).unwrap();
        chat.exit_process().unwrap();
        let saved = std::path::Path::new(&path).exists();
        let _ = std::fs::remove_file(&path);
        assert!(!saved, "{path} was written for an empty chat");
    }

    #[tokio::test]
    async fn a_failed_send_leaves_the_history_as_it_was() {
        let (base_url, _) = test_server::serve_once(400, REJECTED).await;
        let mut chat = chat_against(&base_url);
        chat.add_to_chat_history(Message::user("earlier"));
        assert!(chat.send(Message::user("summarize")).await.is_err());
        assert_eq!(chat.chat_history().len(), 1);
    }

    #[test]
    fn a_model_is_picked_by_its_number_after_bad_tries() {
        let options = [
            test_model(),
            ModelInfo {
                id: String::from("second"),
                ..test_model()
            },
        ];
        let mut lines = "0\nfoo\n3\n2\n".as_bytes();
        let picked = select_model(&mut lines, &options).unwrap().unwrap();
        assert_eq!(picked.id, "second");
    }

    #[test]
    fn exit_or_the_end_of_input_picks_no_model() {
        let options = [test_model()];
        assert!(
            select_model(&mut "/exit\n".as_bytes(), &options)
                .unwrap()
                .is_none()
        );
        assert!(
            select_model(&mut "".as_bytes(), &options)
                .unwrap()
                .is_none()
        );
        assert!(
            select_model(&mut "9\n".as_bytes(), &options)
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn the_agent_is_marvin_with_room_for_long_tool_chains() {
        let model = test_model();
        let tavily = Arc::new(TavilyClient::new("test"));
        let client = Client::new("test").unwrap();
        let agent = agent(&client, &model, &tavily);
        assert_eq!(agent.name.as_deref(), Some("Marvin"));
        assert_eq!(agent.default_max_turns, Some(100));
        assert_eq!(agent.model.model, "claude-test");
    }
}
