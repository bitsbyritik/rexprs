use crate::bindings::types::{JsRequest, JsResponse};
use hyper::{
    Request, Response,
    body::{Bytes, Incoming},
};
use napi::threadsafe_function::ThreadsafeFunction;
use rexprs_core::http::types::RequestHandlerFn;
use std::{collections::HashMap, sync::Arc};

pub fn create_handler(tsfn: ThreadsafeFunction<JsRequest, JsResponse>) -> RequestHandlerFn {
    let tsfn = Arc::new(tsfn);
    Arc::new(move |req: Request<Incoming>, params| {
        let tsfn = tsfn.clone();
        Box::pin(async move {
            let headers = req
                .headers()
                .iter()
                .filter_map(|(k, v)| Some((k.as_str().to_string(), v.to_str().ok()?.to_string())))
                .collect::<HashMap<_, _>>();

            let method = req.method().to_string();
            let path = req.uri().path().to_string();
            let query = req
                .uri()
                .query()
                .map(|q| {
                    url::form_urlencoded::parse(q.as_bytes())
                        .into_owned()
                        .collect::<HashMap<String, String>>()
                })
                .unwrap_or_default();

            let js_req = JsRequest {
                method,
                path,
                body: None,
                headers: headers.clone(),
                params,
                query,
            };

            println!("Before tsfn.call_async");
            let js_res = JsResponse {
                status_code: Some(200),
                body: Some("pong from rust".to_string()),
                headers: HashMap::new(),
            };
            println!("✅ Skipping JS, returning test response");
            println!("After tsfn.call_async");

            let status = js_res.status_code.unwrap_or(200);
            let body = js_res.body.unwrap_or_default();

            let mut builder = Response::builder().status(status);
            for (key, value) in js_res.headers {
                builder = builder.header(&key, &value);
            }

            builder
                .body(Bytes::from(body))
                .unwrap_or_else(|_| Response::new(Bytes::from("Failed to build response")))
        })
    })
}
