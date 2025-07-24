pub struct Response {
    pub status_line: String,
    pub headers: Vec<String>,
    pub body: String,
}

impl Response {
    pub fn new(status_line: &str, body: &str, content_type: &str) -> Self {
        let headers = vec![
            format!("Content-Length: {}", body.len()),
            format!("Content-Type: {}", content_type),
        ];
        Self {
            status_line: status_line.to_string(),
            headers,
            body: body.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        let headers = self.headers.join("\r\n");
        format!("{}\r\n{}\r\n\r\n{}", self.status_line, headers, self.body)
    }
}
