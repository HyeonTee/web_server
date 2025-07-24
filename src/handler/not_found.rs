use crate::http::{response::Response, status::StatusCode};
use std::fs;

pub fn serve() -> Response {
    match fs::read_to_string("static/404.html") {
        Ok(content) => Response::new(
            StatusCode::NotFound,
            &content,
            "text/html",
        ),
        Err(_) => Response::new(
            StatusCode::NotFound,
            "404 Not Found",
            "text/plain",
        ),
    }
}
