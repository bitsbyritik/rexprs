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

pub fn create_error(status: StatusCode, message: &str) -> Response<Bytes> {
    let json_body = json!({ "error": message }).to_string();
    let bytes = Bytes::from(json_body);

    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(bytes)
        .unwrap_or_else(|_| {
            let fallback = Bytes::from("{\"error\": \"Internal Server Error\"}");
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(fallback)
                .unwrap()
        })
}

pub fn bad_request(message: &str) -> Response<Bytes> {
    create_error(StatusCode::BAD_REQUEST, message)
}

pub fn not_found(message: &str) -> Response<Bytes> {
    create_error(StatusCode::NOT_FOUND, message)
}

pub fn timeout_error() -> Response<Bytes> {
    create_error(StatusCode::REQUEST_TIMEOUT, "Request timed out")
}

pub fn internal_server_error() -> Response<Bytes> {
    create_error(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
}
