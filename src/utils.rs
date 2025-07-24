use crate::http::response::Response;
use crate::http::status::StatusCode;

pub fn get_content_type(file_path: &str) -> &str {
    if file_path.ends_with(".html") {
        "text/html"
    } else if file_path.ends_with(".css") {
        "text/css"
    } else if file_path.ends_with(".js") {
        "application/javascript"
    } else if file_path.ends_with(".json") {
        "application/json"
    } else if file_path.ends_with(".png") {
        "image/png"
    } else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
        "image/jpeg"
    } else if file_path.ends_with(".gif") {
        "image/gif"
    } else {
        "text/plain"
    }
}

pub fn json_error_response(status: StatusCode, message: &str) -> Response {
    let body = format!(
        r#"{{"error":"{}","message":"{}"}}"#,
        status.reason(),
        message
    );
    Response::new(status, &body, "application/json")
}