use std::collections::HashMap;
use serde::de::DeserializeOwned;
use urlencoding::decode;

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub query_string: String,
    pub body: Vec<u8>,
}

impl Request {
    pub fn from(buf: &[u8]) -> Option<Self> {
        let request_text = std::str::from_utf8(buf).ok()?;

        // 분리: 헤더와 바디
        let (head, body) = request_text.split_once("\r\n\r\n")?;
        let mut lines = head.lines();

        // 요청라인 파싱
        let request_line = lines.next()?;
        let mut parts = request_line.split_whitespace();
        let method = parts.next()?.to_string();
        let full_path = parts.next()?.to_string();
        let version = parts.next()?.to_string();

        // path, query_string 분리
        let (path, query_string) = if let Some((p, q)) = full_path.split_once('?') {
            (p.to_string(), q.to_string())
        } else {
            (full_path, String::new())
        };

        // 헤더 파싱
        let mut headers = HashMap::new();
        for line in lines {
            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        // 바디는 헤더 다음부터 남은 부분
        let body_start = buf.windows(4).position(|w| w == b"\r\n\r\n")? + 4;
        let body = buf[body_start..].to_vec();

        Some(Self {
            method,
            path,
            version,
            headers,
            query_string,
            body,
        })
    }

    pub fn parse_query_string(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();

        for pair in self.query_string.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                let key = decode(key).unwrap_or_default().to_string();
                let value = decode(value).unwrap_or_default().to_string();
                result.insert(key, value);
            }
        }

        result
    }

    pub fn json<T: DeserializeOwned>(&self) -> Option<T> {
        serde_json::from_slice(&self.body).ok()
    }
}
