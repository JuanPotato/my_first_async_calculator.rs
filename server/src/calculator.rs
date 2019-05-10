/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::io::{self, Cursor};

use futures::StreamExt;
use futures_util::io::{AsyncReadExt, AsyncWriteExt};
use romio::TcpStream;

use calc_utils::{Deserializer, MathRequest, MathResult, Operation, SerealStreamer, Serializer};

pub async fn process_client(stream: TcpStream) -> io::Result<()> {
    let (read_stream, mut write_stream) = stream.split();

    let mut request_stream: SerealStreamer<_, MathRequest> = SerealStreamer::new(read_stream);

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

        let mut buf = Vec::<u8>::new();

        buf.serialize(&(4 + 8 as u32)).unwrap();
        buf.serialize(&math_res).unwrap();

        await!(write_stream.write_all(&buf)).unwrap();
    }

    Ok(())
}
