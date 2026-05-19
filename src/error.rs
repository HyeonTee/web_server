use crate::http::{Response, StatusCode};

pub fn json_error_response(status: StatusCode, message: &str) -> Response {
    let body = format!(
        r#"{{"error":"{}","message":"{}"}}"#,
        status.reason(),
        message
    );
    Response::new(status, &body, "application/json")
}
