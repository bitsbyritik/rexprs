use crate::bindings::types::{JsRequest, JsResponse};
use crate::handler::create_handler;
use hyper::Method;
use napi::threadsafe_function::{ThreadSafeCallContext, ThreadsafeCallContext, ThreadsafeFunction};
use napi::{JsNumber, JsString, bindgen_prelude::*};
use napi_derive::napi;
use rexprs_core::server::RexprsServer;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;
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
        self.rt
            .block_on(self.inner.listen(port))
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(())
    }

    #[napi]
    pub fn get(&mut self, path: String, callback: Function<JsRequest, JsResponse>) -> Result<()> {
        self.register_route(Method::GET, &path, callback)
    }

    fn register_route(
        &mut self,
        method: Method,
        path: &str,
        callback: Function<JsRequest, JsResponse>,
    ) -> Result<()> {
        let tsfn = callback
            .build_threadsafe_function()
            .callee_handled::<true>()
            .build()?;

        let handler = create_handler(tsfn);
        self.inner.add_route(method, path, handler);

        Ok(())
    }
}
