use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};

pub fn create_error_response(status: StatusCode, message: &str) -> Response<BoxBody<Bytes, Error>> {
    Response::builder()
        .status(status)
        .header("content-type", "text/plain")
        .body(Bytes::from(message.to_string()))
        .unwrap_or_else(|_| {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Bytes::from("Internal server error"))
                .unwrap()
        })
}
