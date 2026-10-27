//! A local HTTP server for tests: answers requests with canned responses, in order, and hands back what was asked.

use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;

/// What the server was asked.
#[derive(Debug)]
pub(crate) struct Request {
    /// The request line, such as `GET /v1/models HTTP/1.1`.
    pub(crate) line: String,
    /// The request body.
    pub(crate) body: String,
}

/// One canned response.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Reply {
    /// HTTP status code.
    pub(crate) status: u16,
    /// `content-type` header.
    pub(crate) content_type: &'static str,
    /// Response body.
    pub(crate) body: &'static str,
}

impl Reply {
    /// A JSON response.
    pub(crate) fn json(status: u16, body: &'static str) -> Self {
        Self {
            status,
            content_type: "application/json",
            body,
        }
    }

    /// A 200 server-sent event stream.
    pub(crate) fn events(body: &'static str) -> Self {
        Self {
            status: 200,
            content_type: "text/event-stream",
            body,
        }
    }
}

/// Serves one JSON request on a free local port. Returns the base URL and a handle that resolves to the request.
pub(crate) async fn serve_once(status: u16, body: &'static str) -> (String, JoinHandle<Request>) {
    let (base_url, handle) = serve(vec![Reply::json(status, body)]).await;
    let handle = tokio::spawn(async move { handle.await.unwrap().pop().unwrap() });
    (base_url, handle)
}

/// Serves one request per reply, in order, on a free local port. Returns the base URL and a handle that resolves to the requests.
pub(crate) async fn serve(replies: Vec<Reply>) -> (String, JoinHandle<Vec<Request>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let handle = tokio::spawn(async move {
        let mut requests = Vec::new();
        for reply in replies {
            let (mut socket, _) = listener.accept().await.unwrap();
            requests.push(answer(&mut socket, reply).await);
        }
        requests
    });
    (base_url, handle)
}

/// How long a test waits for the server to see every request it expects.
const PATIENCE: Duration = Duration::from_secs(10);

/// Waits for the server behind `handle` to finish. Panics instead of hanging when a request it expects never comes.
pub(crate) async fn served<T>(handle: JoinHandle<T>) -> T {
    tokio::time::timeout(PATIENCE, handle)
        .await
        .expect("the server did not get every request it expected")
        .unwrap()
}

/// Reads one request from `socket` and writes `reply`.
async fn answer(socket: &mut TcpStream, reply: Reply) -> Request {
    let mut request = Vec::new();
    let mut buf = [0; 4096];
    let body_start = loop {
        let n = socket.read(&mut buf).await.unwrap();
        request.extend_from_slice(&buf[..n]);
        if let Some(i) = request.windows(4).position(|w| w == b"\r\n\r\n") {
            break i.saturating_add(4);
        }
    };
    let head = String::from_utf8_lossy(&request[..body_start]).into_owned();
    let content_length: usize = head
        .to_lowercase()
        .lines()
        .find_map(|line| line.strip_prefix("content-length:"))
        .map_or(0, |len| len.trim().parse().unwrap());
    while request.len() < body_start.saturating_add(content_length) {
        let n = socket.read(&mut buf).await.unwrap();
        request.extend_from_slice(&buf[..n]);
    }
    let Reply {
        status,
        content_type,
        body,
    } = reply;
    let response = format!(
        "HTTP/1.1 {status} Test\r\ncontent-type: {content_type}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
        body.len()
    );
    socket.write_all(response.as_bytes()).await.unwrap();
    Request {
        line: head.lines().next().unwrap_or_default().to_owned(),
        body: String::from_utf8_lossy(&request[body_start..]).into_owned(),
    }
}
