use crate::bindings::res::Res;
use crate::bindings::types::JsRequest;
use crate::handler::create_handler;
use hyper::Method;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use rexprs_core::server::RexprsServer;
use tokio::runtime::Runtime;

mod bindings;
mod handler;

#[napi]
pub struct Rexprs {
    inner: RexprsServer,
    rt: Runtime,
}

#[napi]
impl Rexprs {
    #[napi(constructor)]
    pub fn new() -> Result<Self> {
        Ok(Rexprs {
            inner: RexprsServer::new(),
            rt: Runtime::new().map_err(|e| {
                Error::new(
                    Status::GenericFailure,
                    format!("Failed to create Tokio runtime: {}", e),
                )
            })?,
        })
    }

    #[napi]
    pub fn listen(&mut self, port: u16) -> Result<()> {
        println!("Starting server on localhost {}", port);
        let mut server = std::mem::replace(&mut self.inner, RexprsServer::new());
        self.rt.spawn(async move {
            if let Err(e) = server.listen(port).await {
                eprintln!(" Server error: {}", e);
            }
        });
        println!("Server started succesfully on port {}", port);

        Ok(())
    }

    #[napi]
    pub fn get(
        &mut self,
        path: String,
        callback: Function<(JsRequest, Res), Promise<()>>,
    ) -> Result<()> {
        self.register_route(Method::GET, &path, callback)
    }

    fn register_route(
        &mut self,
        method: Method,
        path: &str,
        callback: Function<(JsRequest, Res), Promise<()>>,
    ) -> Result<()> {
        let tsfn = callback
            .build_threadsafe_function::<(JsRequest, Res)>()
            .build()?;

        let handler = create_handler(tsfn);
        self.inner.add_route(method, path, handler);

        Ok(())
    }
}
