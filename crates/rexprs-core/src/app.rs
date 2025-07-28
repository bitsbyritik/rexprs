use crate::router::{Handler, Router};
use hyper::Method;

pub struct App {
    router: Router,
}

impl App {
    pub fn new() -> Self {
        App {
            router: Router::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::GET, path, handler);
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::POST, path, handler);
    }

    pub fn put(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::PUT, path, handler);
    }

    pub fn delete(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::DELETE, path, handler);
    }

    pub fn patch(&mut self, path: &str, handler: Handler) {
        self.add_route(Method::PATCH, path, handler);
    }

    fn add_route(&mut self, method: Method, path: &str, handler: Handler) {
        self.router.add_route(method, path.to_string(), handler);
    }
}
