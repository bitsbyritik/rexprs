use napi_derive::napi;
use std::collections::HashMap;

#[napi(object)]
pub struct JsRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

#[napi(object)]
pub struct JsResponse {
    pub status_code: Option<u16>,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Default for JsResponse {
    fn default() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        JsResponse {
            status_code: Some(200),
            headers,
            body: None,
        }
    }
}
