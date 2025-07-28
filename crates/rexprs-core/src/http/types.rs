use hyper::{
    Request, Response,
    body::{Bytes, Incoming},
};
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

pub type RequestHandlerFn =
    Arc<dyn Fn(Request<Incoming>, HashMap<String, String>) -> HandleFuture + Send + Sync>;

pub type HandleFuture = Pin<Box<dyn Future<Output = Response<Bytes>> + Send>>;
