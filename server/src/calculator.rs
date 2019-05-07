/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::io::{self, Cursor};

use futures::{async_stream, StreamExt};
use futures_util::io::{AsyncReadExt, AsyncWriteExt};
use romio::TcpStream;

use calc_types::{Deserializer, MathRequest, MathResult, Operation, Serializer};

pub async fn process_client(stream: TcpStream) -> io::Result<()> {
    let (mut read_stream, mut write_stream) = stream.split();

    let mut request_stream = Box::pin(get_requests(&mut read_stream));

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

#[derive(Debug)]
enum ResponseStatus {
    Length,
    Data(usize),
}

#[async_stream]
async fn get_requests<T: AsyncReadExt + Unpin>(stream: &mut T) -> MathRequest {
    let mut status = ResponseStatus::Length;
    let mut length_bytes = [0u8; 4];

    loop {
        match status {
            ResponseStatus::Length => {
                if let Err(e) = await!(stream.read_exact(&mut length_bytes)) {
                    return;
                }

                let len = length_bytes.as_ref().deserialize::<u32>().unwrap() as usize;

                status = ResponseStatus::Data(len);
            }

            ResponseStatus::Data(length) => {
                let mut data_bytes = vec![0u8; length];
                await!(stream.read_exact(&mut data_bytes)).unwrap();

                let mut cursor_bytes = Cursor::new(data_bytes);
                yield cursor_bytes.deserialize::<MathRequest>().unwrap();

                status = ResponseStatus::Length;
            }
        }
    }
}
