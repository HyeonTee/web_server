use crate::http::{Method, Request, Response};

pub type Handler = fn(&Request) -> Response;

pub struct Route {
    pub method: Method,
    pub pattern: Pattern,
    pub handler: Handler,
}

pub enum Pattern {
    Exact(String),
    Prefix(String),
}

impl Pattern {
    pub fn matches(&self, path: &str) -> bool {
        match self {
            Pattern::Exact(p) => p == path,
            Pattern::Prefix(p) => path.starts_with(p.as_str()),
        }
    }
}
