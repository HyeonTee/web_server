use super::{Middleware, Next};
use crate::http::{Request, Response};
use std::time::Instant;

pub struct Logger;

impl Middleware for Logger {
    fn call<'a>(&'a self, req: &'a Request, next: Next<'a>) -> Response {
        let start = Instant::now();
        let method = req.method.as_str().to_string();
        let path = req.path.clone();

        let response = next.run(req);

        eprintln!("{} {} ({:?})", method, path, start.elapsed());
        response
    }
}
