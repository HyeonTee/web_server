pub mod logger;
pub use logger::Logger;

use crate::http::{Request, Response};

pub trait Middleware: Send + Sync {
    fn call<'a>(&'a self, req: &'a Request, next: Next<'a>) -> Response;
}

pub struct Next<'a> {
    chain: &'a [Box<dyn Middleware>],
    final_handler: &'a dyn Fn(&Request) -> Response,
}

impl<'a> Next<'a> {
    pub fn new(
        chain: &'a [Box<dyn Middleware>],
        final_handler: &'a dyn Fn(&Request) -> Response,
    ) -> Self {
        Self {
            chain,
            final_handler,
        }
    }

    pub fn run(self, req: &Request) -> Response {
        match self.chain.split_first() {
            Some((first, rest)) => first.call(
                req,
                Next {
                    chain: rest,
                    final_handler: self.final_handler,
                },
            ),
            None => (self.final_handler)(req),
        }
    }
}
