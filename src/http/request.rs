pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
}

impl Request {
    pub fn from(buf: &[u8]) -> Option<Self> {
        let request_line = std::str::from_utf8(buf).ok()?.lines().next()?;
        let mut parts = request_line.split_whitespace();
        Some(Self {
            method: parts.next()?.to_string(),
            path: parts.next()?.to_string(),
            version: parts.next()?.to_string(),
        })
    }
}
