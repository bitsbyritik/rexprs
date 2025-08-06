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

            // let promise = match tsfn.call_async((js_req, res)).await {
            //     Ok(promise) => promise,
            //     Err(e) => {
            //         eprintln!("Failed to call JS function: {:?}", e);
            //         return internal_server_error();
            //     }
            // };
            tsfn.call((js_req, res), ThreadsafeFunctionCallMode::NonBlocking);
            println!("After tsfn.call_async");

            match rx.await {
                Ok(js_res) => convert_to_hyper_response(js_res),
                Err(_) => internal_server_error(),
            }
            // match rx.await {
            //     Ok(js_response) => {
            //         // Then await the promise to ensure JS side completed
            //         let _ = promise.await;
            //         convert_to_hyper_response(js_response)
            //     }
            //     Err(_) => {
            //         // If channel fails, still wait for promise to avoid unhandled rejection
            //         let _ = promise.await;
            //         internal_server_error()
            //     }
            // }
        })
    })
}
