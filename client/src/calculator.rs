/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;

use futures::{SinkExt, StreamExt};
use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures::channel::oneshot;
use futures::executor::ThreadPool;
use futures::task::SpawnExt;
use futures_util::io::AsyncReadExt;
use futures_util::io::AsyncWriteExt;
use romio::TcpStream;

use calc_utils::{Deserializer, MathRequest, MathResult, SerealStreamer, Serializer};

#[derive(Debug)]
pub struct Calculator {
    threadpool: ThreadPool,
    message_sender: MsgSender,
}

#[derive(Debug)]
pub enum Input {
    Result(MathResult),
    Request(Msg),
}

type Msg = (MathRequest, oneshot::Sender<MathResult>);
type MsgSender = UnboundedSender<Msg>;
type MsgReceiver = UnboundedReceiver<Msg>;


impl Calculator {
    pub fn new() -> Calculator {
        let mut threadpool = ThreadPool::new().unwrap();
        let (tx, rx) = mpsc::unbounded::<Msg>();

        threadpool.spawn(process_responses(rx)).unwrap();

        Calculator {
            threadpool,
            message_sender: tx,
        }
    }

    pub async fn send(&mut self, req: MathRequest) -> Result<MathResult, oneshot::Canceled> {
        let (one_tx, one_rx) = oneshot::channel();

        await!(self.message_sender.send((req, one_tx))).unwrap();

        await!(one_rx)
    }

    pub async fn add(&mut self, a: f64, b: f64) -> Result<MathResult, oneshot::Canceled> {
        await!(self.send(MathRequest::add(a, b)))
    }

    pub async fn subtract(&mut self, a: f64, b: f64) -> Result<MathResult, oneshot::Canceled> {
        await!(self.send(MathRequest::subtract(a, b)))
    }

    pub async fn multiply(&mut self, a: f64, b: f64) -> Result<MathResult, oneshot::Canceled> {
        await!(self.send(MathRequest::multiply(a, b)))
    }

    pub async fn divide(&mut self, a: f64, b: f64) -> Result<MathResult, oneshot::Canceled> {
        await!(self.send(MathRequest::divide(a, b)))
    }
}


async fn process_responses(incoming_requests: MsgReceiver) {
    let stream = await!(TcpStream::connect(&"127.0.0.1:7878".parse().unwrap())).unwrap();
    let (read_stream, mut write_stream) = stream.split();

    let results_stream = SerealStreamer::new(read_stream).map(Input::Result);

    let requests_stream = incoming_requests.map(Input::Request);

    let mut combined_stream = futures::stream::select(results_stream, requests_stream);

    let mut request_map: HashMap<u32, oneshot::Sender<MathResult>> = HashMap::new();

    while let Some(input) = await!(combined_stream.next()) {
        match input {
            Input::Result(result) => {
                println!("{:?}", result);
                let tx = request_map.remove(&result.id).unwrap();
                tx.send(result).unwrap();
            }

            Input::Request((req, tx)) => {
                println!("{:?}", req);
                let mut buf = Vec::<u8>::new();

                buf.serialize(&(4 + 4 + 8 + 8 as u32)).unwrap();
                buf.serialize(&req).unwrap();

                await!(write_stream.write_all(&buf)).unwrap();

                request_map.insert(req.id, tx);
            }
        }
    }
}
