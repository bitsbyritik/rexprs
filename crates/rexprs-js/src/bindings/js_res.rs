use crate::bindings::types::JsResponse;
use hyper::body::Bytes;
use hyper::header::HeaderName;
use hyper::{Response, StatusCode};

pub fn convert_to_hyper_response(js_response: JsResponse) -> Response<Bytes> {
    let status = js_response
        .status_code
        .and_then(|code| StatusCode::from_u16(code).ok())
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let mut builder = Response::builder().status(status);

    for (key, value) in js_response.headers {
        if let Ok(header_name) = key.parse::<HeaderName>() {
            builder = builder.header(header_name, value);
        }
    }

    let body = Bytes::from(js_response.body.unwrap_or_default());
    builder.body(body).unwrap_or_else(|_| {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Bytes::from("Failed to build response"))
            .unwrap()
    })
}
