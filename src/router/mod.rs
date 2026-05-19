pub mod route;
pub use route::{Handler, Pattern, Route};

use crate::handlers::not_found;
use crate::http::{Method, Request, Response};
use crate::middleware::{Middleware, Next};

pub struct Router {
    routes: Vec<Route>,
    middleware: Vec<Box<dyn Middleware>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            middleware: Vec::new(),
        }
    }

    pub fn get(self, path: &str, handler: Handler) -> Self {
        self.add(Method::Get, path, handler)
    }

    pub fn post(self, path: &str, handler: Handler) -> Self {
        self.add(Method::Post, path, handler)
    }

    pub fn wrap<M: Middleware + 'static>(mut self, mw: M) -> Self {
        self.middleware.push(Box::new(mw));
        self
    }

    fn add(mut self, method: Method, path: &str, handler: Handler) -> Self {
        let pattern = if let Some(prefix) = path.strip_suffix("/*") {
            Pattern::Prefix(format!("{}/", prefix))
        } else if let Some(prefix) = path.strip_suffix('*') {
            Pattern::Prefix(prefix.to_string())
        } else {
            Pattern::Exact(path.to_string())
        };
        self.routes.push(Route {
            method,
            pattern,
            handler,
        });
        self
    }

    pub fn handle(&self, req: &Request) -> Response {
        let dispatch = |req: &Request| -> Response {
            for route in &self.routes {
                // HEAD is satisfied by the matching GET handler (RFC 9110 §9.3.2).
                let method_matches = route.method == req.method
                    || (route.method == Method::Get && req.method == Method::Head);
                if method_matches && route.pattern.matches(&req.path) {
                    return (route.handler)(req);
                }
            }
            not_found::serve()
        };

        Next::new(&self.middleware, &dispatch).run(req)
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
