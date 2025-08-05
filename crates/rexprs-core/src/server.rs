use crate::http::error::create_error_response;
use crate::router::{Handler, Router};
use http_body_util::Full;
use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::sync::Arc;
use std::{convert::Infallible, error::Error, net::SocketAddr};
use tokio::net::TcpListener;

pub struct RexprsServer {
    port: u16,
    router: Option<Router>,
}

impl RexprsServer {
    pub fn new() -> Self {
        RexprsServer {
            port: 3000,
            router: None,
        }
    }

    pub fn add_route(&mut self, method: Method, path: &str, handler: Handler) {
        if self.router.is_none() {
            self.router = Some(Router::new());
        }

        if let Some(router) = &mut self.router {
            router.add_route(method, path, handler);
        }
    }

    pub async fn listen(&mut self, port: u16) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.port = port;
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

        println!("Rexprs server listening on {}", addr);

        let listener = TcpListener::bind(addr).await?;

        let router = Arc::new(self.router.take().unwrap_or_else(Router::new));

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let router = Arc::clone(&router);

            tokio::spawn(async move {
                let service = service_fn(move |req| {
                    let router = Arc::clone(&router);
                    handle_request(req, router)
                });

                if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

async fn handle_request(
    req: Request<Incoming>,
    router: Arc<Router>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, Infallible> {
    let method = req.method();
    let path = req.uri().path();

    if let Some((handler, params)) = router.find_handler(method, path) {
        match tokio::time::timeout(std::time::Duration::from_secs(30), handler(req, params)).await {
            Ok(response) => {
                let (parts, body) = response.into_parts();
                let boxed_body = BoxBody::new(Full::new(body));
                Ok(Response::from_parts(parts, boxed_body))
            }
            Err(_) => Ok(create_error_response(
                StatusCode::REQUEST_TIMEOUT,
                "Request timeout",
            )),
        }
    } else {
        Ok(create_error_response(
            StatusCode::NOT_FOUND,
            &format!("Cannot {} {}", method, path),
        ))
    }
}
