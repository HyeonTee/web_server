use crate::http::response::Response;
use std::fs;

pub fn serve(file_path: &str) -> Response {
    match fs::read_to_string(file_path) {
        Ok(content) => Response::new("HTTP/1.1 200 OK", &content, get_content_type(file_path)),
        Err(_) => Response::new("HTTP/1.1 500 Internal Server Error", "File read error", "text/plain"),
    }
}

fn get_content_type(file_path: &str) -> &str {
    if file_path.ends_with(".html") {
        "text/html"
    } else if file_path.ends_with(".css") {
        "text/css"
    } else if file_path.ends_with(".js") {
        "application/javascript"
    } else {
        "text/plain"
    }
}
