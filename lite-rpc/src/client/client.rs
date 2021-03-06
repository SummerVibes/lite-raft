use async_std::net::{ToSocketAddrs, TcpStream};
use crate::Result;
use super::*;

/// the client have not yet connected to server.
impl Client<DisConnected>{
    pub fn connect(addr: impl ToSocketAddrs) -> Result<Client<Connected>>{
        let stream = TcpStream::connect(addr).await?;
    }
}

impl Client<Connected>{
    pub fn call(){

    }
    pub async fn async_call(){

    }
    pub fn spawn_call(){}
}
