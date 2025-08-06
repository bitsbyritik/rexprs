use crate::bindings::types::JsResponse;
use napi::Result;
use napi_derive::napi;
use std::{collections::HashMap, mem::take};
use tokio::sync::oneshot;

#[napi(js_name = "Response")]
pub struct Res {
    status_code: Option<u16>,
    headers: HashMap<String, String>,
    sender: Option<oneshot::Sender<JsResponse>>,
}

#[napi]
impl Res {
    #[napi(constructor)]
    pub fn new() -> Self {
        panic!("Res can only be created by the framework");
    }

    pub fn new_with_sender(sender: oneshot::Sender<JsResponse>) -> Self {
        Self {
            status_code: None,
            headers: HashMap::new(),
            sender: Some(sender),
        }
    }

    #[napi]
    pub fn status(&mut self, code: u16) {
        self.status_code = Some(code);
    }

    #[napi]
    pub fn header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    #[napi]
    pub fn send(&mut self, body: String) -> Result<()> {
        println!("res.send() called with body: {}", body);
        let result = self.try_send(Some(body));
        println!("res.send() completed with result: {:?}", result);
        result
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
