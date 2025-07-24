use std::fs;
use crate::http::{response::Response, status::StatusCode};
use crate::utils::get_content_type;

pub fn serve(file_path: &str) -> Response {
    match fs::read_to_string(file_path) {
        Ok(content) => Response::new(
            StatusCode::Ok,
            &content,
            get_content_type(file_path),
        ),
        Err(_) => Response::new(
            StatusCode::InternalServerError,
            "File read error",
            "text/plain",
        ),
    }
}
