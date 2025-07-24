#[derive(Debug)]
pub enum StatusCode {
    Ok,
    NotFound,
    BadRequest,
    Unauthorized,
    MethodNotAllowed,
    InternalServerError,
}

impl StatusCode {
    pub fn as_tuple(&self) -> (&'static str, &'static str) {
        match self {
            StatusCode::Ok => ("200", "OK"),
            StatusCode::NotFound => ("404", "Not Found"),
            StatusCode::BadRequest => ("400", "Bad Request"),
            StatusCode::Unauthorized => ("401", "Unauthorized"),
            StatusCode::MethodNotAllowed => ("405", "Method Not Allowed"),
            StatusCode::InternalServerError => ("500", "Internal Server Error"),
        }
    }

    pub fn to_http_string(&self) -> String {
        let (code, reason) = self.as_tuple();
        format!("HTTP/1.1 {} {}", code, reason)
    }
}
