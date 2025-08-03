use crate::bindings::types::JsResponse;
use napi::Result;
use napi_derive::napi;
use std::{collections::HashMap, mem::take};
use tokio::sync::oneshot;

#[napi]
pub struct Res {
    status_code: Option<u16>,
    headers: HashMap<String, String>,
    sender: Option<oneshot::Sender<JsResponse>>,
}

#[napi]
impl Res {
    #[napi(constructor)]
    pub fn new() -> Self {
        unreachable!()
    }

    #[napi]
    pub fn status(&mut self, code: u16) -> &Self {
        self.status_code = Some(code);
        self
    }

    #[napi]
    pub fn header(&mut self, key: String, value: String) -> &Self {
        self.headers.insert(key, value);
        self
    }

    #[napi]
    pub fn send(&mut self, body: String) -> Result<()> {
        self.try_send(Some(body))
    }

    fn try_send(&mut self, body: Option<String>) -> Result<()> {
        if let Some(sender) = self.sender.take() {
            let response = JsResponse {
                status_code: Some(self.status_code.unwrap_or(200)),
                headers: take(&mut self.headers),
                body,
            };
            sender
                .send(response)
                .map_err(|_| napi::Error::from_reason("Failed to send response"))?;
            Ok(())
        } else {
            Err(napi::Error::from_reason("Response already sent"))
        }
    }
}
