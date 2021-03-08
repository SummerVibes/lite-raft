use std::collections::HashMap;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;

use async_std::future::Future;
use async_std::net::TcpListener;
use erased_serde::{Serialize, Deserializer};


use crate::Id;
use crate::Result;

mod service;
mod server;

/// response struct, it contains a
pub struct Response {
    id: Id,
    result: MethodResult,
}

pub struct Server<State> {
    listener: TcpListener,
    pub service_map: ServiceMap,
    state: PhantomData<State>,
}

/// server not bind to a port
pub struct Bound {}
/// server bound to a port
pub struct NotBound {}

///
pub struct Service{
    pub method_map: HashMap<&'static str, ArcAsyncMethod>,
}

/// service map
pub type ServiceMap = Arc<HashMap<String,Service>>;
///method map
pub type MethodResult = Result<Box<dyn Serialize + Send + Sync + 'static>>;
pub type MethodResultFuture = Pin<Box<dyn Future<Output = MethodResult> + Send>>;
///S represents state
pub type AsyncMethod = dyn Fn(Box<dyn Deserializer<'static> + Send>) -> MethodResultFuture
+ Send
+ Sync
+ 'static;
///atomic reference counter AsyncMethod
pub type ArcAsyncMethod = Arc<AsyncMethod>;
/// what is this?
pub type AsyncServiceCall = dyn Fn(String, Box<dyn Deserializer<'static> + Send>) -> MethodResultFuture
+ Send
+ Sync
+ 'static;
pub type ArcAsyncServiceCall = Arc<AsyncServiceCall>;
pub type AsyncServiceMap = HashMap<&'static str, ArcAsyncServiceCall>;