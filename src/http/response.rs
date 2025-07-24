use serde::Serialize;
use crate::http::status::StatusCode;

pub struct Response {
    status_line: String,
    headers: Vec<String>,
    body: String,
}

impl Response {
    pub fn new(status: StatusCode, body: &str, content_type: &str) -> Self {
        let status_line = status.to_http_string();
        let headers = vec![
            format!("Content-Length: {}", body.len()),
            format!("Content-Type: {}", content_type),
        ];
        Self {
            status_line,
            headers,
            body: body.to_string(),
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.push(format!("{}: {}", key, value));
    }

    pub fn to_string(&self) -> String {
        let headers = self.headers.join("\r\n");
        format!("{}\r\n{}\r\n\r\n{}", self.status_line, headers, self.body)
    }

    pub fn json<T: Serialize>(status: StatusCode, data: &T) -> Self {
        match serde_json::to_string(data) {
            Ok(json_body) => {
                let headers = vec![
                    format!("Content-Length: {}", json_body.len()),
                    "Content-Type: application/json".to_string(),
                ];
                Self {
                    status_line: status.to_http_string(),
                    headers,
                    body: json_body,
                }
            }
            Err(_) => Self::new(
                StatusCode::InternalServerError,
                "Failed to serialize JSON",
                "text/plain",
            ),
        }
    }
}
