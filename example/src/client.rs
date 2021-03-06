// Copyright 2018 Google LLC
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use clap::{App, Arg};
use std::{io, net::SocketAddr};
use tarpc::{client, context, tokio_serde::formats::Json};

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let server_addr = "127.0.0.1";
    let name = "host1".to_string();

    //create a transport
    let mut transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    // WorldClient is generated by the service attribute. It has a constructor `new` that takes a
    // config and any Transport as input.
    let mut client =
        service::WorldClient::new(client::Config::default(), transport.await?).spawn()?;

    // The client has an RPC method for each RPC defined in the annotated trait. It takes the same
    // args as defined, with the addition of a Context, which is always the first arg. The Context
    // specifies a deadline and trace information which can be helpful in debugging requests.
    let hello = client.hello(context::current(), name).await?;

    println!("{}", hello);

    Ok(())
}