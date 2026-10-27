//! A one-shot HTTP server for tests: answers the first request with a canned response and hands back what was asked.

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

/// What the server was asked.
#[derive(Debug)]
pub(crate) struct Request {
    /// The request line, such as `GET /v1/models HTTP/1.1`.
    pub(crate) line: String,
    /// The request body.
    pub(crate) body: String,
}

/// Serves one request on a free local port. Returns the base URL and a handle that resolves to the request.
pub(crate) async fn serve_once(status: u16, body: &'static str) -> (String, JoinHandle<Request>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let handle = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = Vec::new();
        let mut buf = [0; 4096];
        let body_start = loop {
            let n = socket.read(&mut buf).await.unwrap();
            request.extend_from_slice(&buf[..n]);
            if let Some(i) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                break i + 4;
            }
        };
        let head = String::from_utf8_lossy(&request[..body_start]).into_owned();
        let content_length: usize = head
            .to_lowercase()
            .lines()
            .find_map(|line| line.strip_prefix("content-length:"))
            .map_or(0, |len| len.trim().parse().unwrap());
        while request.len() < body_start + content_length {
            let n = socket.read(&mut buf).await.unwrap();
            request.extend_from_slice(&buf[..n]);
        }
        let response = format!(
            "HTTP/1.1 {status} Test\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
            body.len()
        );
        socket.write_all(response.as_bytes()).await.unwrap();
        Request {
            line: head.lines().next().unwrap_or_default().to_owned(),
            body: String::from_utf8_lossy(&request[body_start..]).into_owned(),
        }
    });
    (base_url, handle)
}
