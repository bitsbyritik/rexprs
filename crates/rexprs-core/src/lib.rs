use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, body::Incoming as IncomingBody};
use hyper_util::rt::TokioIo;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub struct RexprsServer {
    port: u16,
}

impl RexprsServer {
    pub fn new() -> Self {
        RexprsServer { port: 0 }
    }

    pub async fn listen(&mut self, port: u16) -> Result<(), Box<dyn Error>> {
        self.port = port;
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(addr).await?;

        println!("Server is running on http://localhost:{}", port);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);

            tokio::task::spawn(async move {
                let service = service_fn(handle_request);

                if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

async fn handle_request(
    req: Request<IncomingBody>,
) -> Result<Response<String>, Box<dyn Error + Send + Sync>> {
    Ok(Response::new("Hello from Rust!".to_string()))
}
