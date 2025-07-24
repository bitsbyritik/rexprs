use napi::bindgen_prelude::*;
use napi_derive::napi;
use rexprs_core::server::RexprsServer;
use tokio::runtime::Runtime;

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
}
