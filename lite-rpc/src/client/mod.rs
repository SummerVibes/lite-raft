pub mod client;

use async_std;
use std::marker::PhantomData;
use async_std::net::TcpStream;
use crate::Id;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// rpc client with state
pub struct Client<ClientState>{
    request_map: HashMap<Id,TcpStream>,
    state: PhantomData<ClientState>
}

/// data structure that represents a rpc request
#[derive(Debug,Deserialize,Serialize)]
pub struct Request{
    id: Id,
    service: String,
    method: String,
    args: Option<Vec<String>>,
}

/// represents client's disconnected state
pub struct DisConnected{}
/// represents client's connected state
pub struct Connected{}
