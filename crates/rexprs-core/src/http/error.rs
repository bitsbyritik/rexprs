use http_body_util::Full;
use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};
use serde_json::json;
use std::convert::Infallible;

pub fn create_error_response(
    status: StatusCode,
    message: &str,
) -> Response<BoxBody<Bytes, Infallible>> {
    let json_body = json!({ "error": message }).to_string();
    let bytes = Bytes::from(json_body);

    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(BoxBody::new(Full::new(bytes)))
        .unwrap_or_else(|_| {
            let fallback = Bytes::from("{\"error\": \"Internal server error\"}");
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(BoxBody::new(Full::new(fallback)))
                .unwrap()
        })
}
