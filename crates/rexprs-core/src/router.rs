use crate::http::types::RequestHandlerFn;
use hyper::Method;
use std::collections::HashMap;

pub type Handler = RequestHandlerFn;

#[derive(Clone)]
pub struct Route {
    pub method: Method,
    pub path: String,
    pub handler: Handler,
}

#[derive(Default)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Router { routes: Vec::new() }
    }

    pub fn add_route(&mut self, method: Method, path: &str, handler: Handler) {
        self.routes.push(Route {
            method,
            path: path.to_string(),
            handler,
        });
    }

    pub fn find_handler(
        &self,
        method: &Method,
        req_path: &str,
    ) -> Option<(&Handler, HashMap<String, String>)> {
        for route in &self.routes {
            if &route.method != method {
                continue;
            }

            if let Some(params) = match_route(&route.path, req_path) {
                return Some((&route.handler, params));
            }
        }
        None
    }
}

pub fn match_route(route_path: &str, req_path: &str) -> Option<HashMap<String, String>> {
    let route_parts: Vec<_> = route_path.trim_matches('/').split('/').collect();
    let req_parts: Vec<_> = req_path.trim_matches('/').split('/').collect();

    if route_parts.len() != req_parts.len() {
        return None;
    }

    let mut params = HashMap::new();
    for (route_part, req_part) in route_parts.iter().zip(req_parts.iter()) {
        if route_part.starts_with(':') {
            params.insert(route_part[1..].to_string(), req_part.to_string());
        } else if route_part != req_part {
            return None;
        }
    }
    Some(params)
}
