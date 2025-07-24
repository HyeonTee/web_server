use crate::http::{request::Request, response::Response};
use super::{static_handler, not_found};

pub fn route(request: &Request) -> Response {
    match request.path.as_str() {
        "/" => static_handler::serve("static/index.html"),
        "/about" => static_handler::serve("static/about.html"),
        _ => not_found::serve(),
    }
}
