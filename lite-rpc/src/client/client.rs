use async_std::net::{ToSocketAddrs, TcpStream};
use crate::{Result};
use super::*;
use crate::server::Response;
use futures::{AsyncWriteExt, AsyncReadExt};


/// the client have not yet connected to server.
impl Client<DisConnected> {
    ///connect to a server
    pub async fn connect(addr: impl ToSocketAddrs) -> Result<Client<Connected>> {
        let stream = TcpStream::connect(addr).await?;
        return Ok(Client::<Connected> { request_map: Default::default(), state: PhantomData });
    }
}

/// connected to server
impl Client<Connected> {
    //TODO: test
    ///commonly call
    pub fn call(&self, req: Request) -> Result<Box<Response>> {
        let data = serde_json::to_string(&req)?;
        let mut stream = &self.stream;
        let s = stream.write_all(data.as_bytes());
        let mut buf = vec![0u8; 1024];
        let n = stream.read(&mut buf).await?;
        let str = String::from_utf8_lossy(&buf[0..n]);
        let resp: Box<Response> = serde_json::from_str(str.as_ref())?;
        Ok(resp)
    }
    fn read(stream: TcpStream) -> Result<()>{

        Ok(())
    }
    fn write(stream: TcpStream, data: String) -> Result<()> {
        Ok(())
    }

    ///asynchronously call
    pub async fn async_call() {}

    ///create a new thread to call
    pub fn spawn_call() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddrV4;
    use async_std::net::Ipv4Addr;
    use futures::executor::block_on;

    #[test]
    fn connect_test() {
        let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
        async_std::task::block_on(async{Client::connect(addr).await.unwrap()});
    }
    #[test]
    fn call_test() {
        block_on(async{
            let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
            let client = Client::connect(addr).await.unwrap();
            client.
            client.call()
        });

    }
}
