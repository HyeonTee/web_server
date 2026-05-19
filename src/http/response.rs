use crate::http::status::StatusCode;
use serde::Serialize;

pub struct Response {
    status_line: String,
    headers: Vec<String>,
    body: Vec<u8>,
}

impl Response {
    pub fn new(status: StatusCode, body: impl AsRef<[u8]>, content_type: &str) -> Self {
        let body = body.as_ref().to_vec();
        let headers = vec![
            format!("Content-Length: {}", body.len()),
            format!("Content-Type: {}", content_type),
            "Connection: close".to_string(),
        ];
        Self {
            status_line: status.to_http_string(),
            headers,
            body,
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.push(format!("{}: {}", key, value));
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let headers = self.headers.join("\r\n");
        let head = format!("{}\r\n{}\r\n\r\n", self.status_line, headers);
        let mut out = head.into_bytes();
        out.extend_from_slice(&self.body);
        out
    }

    pub fn json<T: Serialize>(status: StatusCode, data: &T) -> Self {
        match serde_json::to_string(data) {
            Ok(json_body) => Self::new(status, json_body, "application/json"),
            Err(_) => Self::new(
                StatusCode::InternalServerError,
                "Failed to serialize JSON",
                "text/plain",
            ),
        }
    }
}
