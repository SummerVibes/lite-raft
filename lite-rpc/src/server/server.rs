use crate::{Error, Id, Result};
use crate::server::Server;
use super::*;
use async_std::net::{TcpListener, TcpStream};
use std::net::ToSocketAddrs;
use std::option::Option::Some;
use async_std::task;
use futures::{StreamExt, AsyncReadExt};

/// server definition
impl Server<NotBound> {
    //TODO: implement
    // how to implement #[export_service]
    pub async fn create(addr: String) -> Result<Server<Bound>> {
        let listener = TcpListener::bind(addr).await?;
        let server = Server { listener, service_map: Arc::new(HashMap::new()), state: PhantomData};
        Ok(server)
    }

}
impl Server<Bound> {
    //TODO: implement
    ///accept a connection and create a new thread to serve client requests
    pub async fn accept(&self) -> Result<()>{
        let mut incoming = self.listener.incoming();
        // block to wait for incoming requests
        while let Some(conn) = incoming.next().await {
            let mut stream = conn?;
            task::spawn(Self::serve(stream));
        }
        Ok(())
    }

    //TODO: implement
    ///serve client, parse request, invoke corresponding method and return response
    pub async fn serve(mut stream: TcpStream) -> Result<()>{
        let mut buf = [0; 1500];
        let n = stream.read(&mut buf).await.unwrap();
        let str = String::from_utf8_lossy(&buf[..n]);
        println!("{}",str);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn create_test() {
        block_on(async{
            let server = Server::create("127.0.0.1:8080".to_string()).await.unwrap();
            server.accept();
        });

    }
}


