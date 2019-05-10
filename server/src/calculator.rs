/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::io;

use futures::{SinkExt, StreamExt};
use futures_util::io::AsyncReadExt;
use romio::TcpStream;

use calc_utils::{MathRequest, MathResult, Operation, SerealSink, SerealStreamer};

pub async fn process_client(stream: TcpStream) -> io::Result<()> {
    let (read_stream, write_stream) = stream.split();

    let mut request_stream: SerealStreamer<MathRequest, _> = SerealStreamer::new(read_stream);
    let mut response_sink: SerealSink<MathResult, _> = SerealSink::new(write_stream);

    while let Some(request) = await!(request_stream.next()) {
        println!("Math request: {:?}", &request);

        let res = match &request.operation {
            Operation::Addition => request.a + request.b,
            Operation::Subtraction => request.a - request.b,
            Operation::Multiplication => request.a * request.b,
            Operation::Division => request.a / request.b,
        };

        println!("Result: {}", res);

        let math_res = MathResult {
            id: request.id,
            res,
        };

        await!(response_sink.send(&math_res)).unwrap();
    }

    Ok(())
}
