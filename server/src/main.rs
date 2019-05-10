/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![feature(async_await, await_macro, generators)]

use std::io;

use futures::executor::{self, ThreadPool};
use futures::task::SpawnExt;
use futures_util::stream::StreamExt;
use romio::TcpListener;

use crate::calculator::process_client;

mod calculator;

fn main() -> io::Result<()> {
    executor::block_on(async {
        let mut threadpool = ThreadPool::new()?;

        let mut tcp_listener = TcpListener::bind(&"127.0.0.1:7878".parse().unwrap())?;
        let mut incoming_connections = tcp_listener.incoming();

        println!("Listening on 127.0.0.1:7878");

        while let Some(tcp_stream) = await!(incoming_connections.next()) {
            let stream = tcp_stream?;
            let addr = stream.peer_addr()?;

            threadpool.spawn(async move {
                println!("Accepting stream from: {}", addr);

                await!(process_client(stream)).unwrap();

                println!("Closing stream from: {}", addr);
            }).unwrap();
        }

        Ok(())
    })
}
