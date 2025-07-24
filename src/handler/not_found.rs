use crate::http::response::Response;
use std::fs;

pub fn serve() -> Response {
    match fs::read_to_string("static/404.html") {
        Ok(content) => Response::new("HTTP/1.1 404 Not Found", &content, "text/html"),
        Err(_) => Response::new("HTTP/1.1 404 Not Found", "404 Not Found", "text/plain"),
    }
}
