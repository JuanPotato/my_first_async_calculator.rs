/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::io;

use tokio::net::TcpListener;
use tokio::prelude::*;

use crate::calculator::process_client;

mod calculator;

#[tokio::main]
async fn main() -> io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:7878").await?;
    let mut incoming_connections = tcp_listener.incoming();

    println!("Listening on 127.0.0.1:7878");

    while let Some(tcp_stream) = incoming_connections.next().await {
        let stream = tcp_stream.unwrap();
        let addr = stream.peer_addr().unwrap();

        tokio::spawn(async move {
            println!("Accepting stream from: {}", addr);

            process_client(stream).await.unwrap();

            println!("Closing stream from: {}", addr);
        });
    }

    Ok(())
}
