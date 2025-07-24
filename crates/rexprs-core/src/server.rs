use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::{convert::Infallible, error::Error, net::SocketAddr};
use tokio::net::TcpListener;

pub struct RexprsServer {
    port: u16,
}

impl RexprsServer {
    pub fn new() -> Self {
        RexprsServer { port: 3000 }
    }

    pub async fn listen(&mut self, port: u16) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.port = port;
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

        println!("Rexprs server listening on {}", addr);

        let listner = TcpListener::bind(addr).await?;

        loop {
            let (stream, _) = listner.accept().await?;
            let io = TokioIo::new(stream);

            tokio::spawn(async move {
                let service = service_fn(handle_request);

                if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

async fn handle_request(_req: Request<Incoming>) -> Result<Response<String>, Infallible> {
    let response = Response::new("Hello from Rexprs!".to_string());
    Ok(response)
}
