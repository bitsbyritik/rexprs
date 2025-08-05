use http_body_util::BodyExt;
use hyper::{Request, body::Incoming};
use napi_derive::napi;
use std::collections::HashMap;

#[napi(object, js_name = "Request")]
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

impl JsRequest {
    pub async fn from_parts(
        req: Request<Incoming>,
        params: HashMap<String, String>,
    ) -> Result<Self, hyper::Error> {
        let (parts, body) = req.into_parts();
        let collected_body = body.collect().await?;
        let bytes = collected_body.to_bytes();

        let body_str = if !bytes.is_empty() {
            Some(String::from_utf8_lossy(&bytes).to_string())
        } else {
            None
        };

        let query_map = parts.uri.query().map_or(HashMap::new(), |q| {
            url::form_urlencoded::parse(q.as_bytes())
                .into_owned()
                .collect()
        });

        Ok(JsRequest {
            method: parts.method.to_string(),
            path: parts.uri.path().to_string(),
            headers: parts
                .headers
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or_default().to_string()))
                .collect(),
            body: body_str,
            params,
            query: query_map,
        })
    }
}
