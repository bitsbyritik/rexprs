use crate::bindings::{
    js_res::convert_to_hyper_response,
    res::Res,
    types::{JsRequest, JsResponse},
};
use hyper::{Request, body::Incoming};
use napi::{
    Status,
    bindgen_prelude::Promise,
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
};
use rexprs_core::http::{error::internal_server_error, types::RequestHandlerFn};
use std::sync::Arc;
use tokio::sync::oneshot;

pub fn create_handler(
    tsfn: ThreadsafeFunction<(JsRequest, Res), Promise<()>, (JsRequest, Res), Status, false>,
) -> RequestHandlerFn {
    let tsfn = Arc::new(tsfn);
    Arc::new(move |req: Request<Incoming>, params| {
        let tsfn = tsfn.clone();
        Box::pin(async move {
            let js_req = match JsRequest::from_parts(req, params).await {
                Ok(req) => req,
                Err(_) => return internal_server_error(),
            };

            let (tx, rx) = oneshot::channel::<JsResponse>();
            let res = Res::new_with_sender(tx);

            println!("Before tsfn.call_async");
            let promise_result = tsfn.call_async((js_req, res)).await;
            println!("After tsfn.call_async");

            match promise_result {
                Ok(promise) => {
                    println!("Successfully called JS function, got promise");

                    match promise.await {
                        Ok(_) => {
                            println!("Promise resolved successfully");
                        }
                        Err(e) => {
                            eprintln!("Promise rejected: {:?}", e);
                            return internal_server_error();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to call JS function: {:?}", e);
                    return internal_server_error();
                }
            }

            match rx.await {
                Ok(js_response) => {
                    println!("Received response");
                    convert_to_hyper_response(js_response)
                }
                Err(_) => internal_server_error(),
            }
        })
    })
}
