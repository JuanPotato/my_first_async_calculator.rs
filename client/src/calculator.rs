/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;

use futures::{SinkExt, StreamExt};
use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures::channel::oneshot;
use tokio::net::TcpStream;

use calc_utils::{MathRequest, MathResult, SerealSink, SerealStreamer};

#[derive(Debug)]
pub struct Calculator {
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
        let (tx, rx) = mpsc::unbounded::<Msg>();

        tokio::spawn(process_responses(rx));

        Calculator {
            message_sender: tx,
        }
    }

    pub async fn send(&mut self, req: MathRequest) -> Result<MathResult, oneshot::Canceled> {
        let (one_tx, one_rx) = oneshot::channel();

        self.message_sender.send((req, one_tx)).await.unwrap();

        one_rx.await
    }

    pub async fn add(&mut self, a: f64, b: f64) -> Result<MathResult, oneshot::Canceled> {
        self.send(MathRequest::add(a, b)).await
    }

    pub async fn subtract(&mut self, a: f64, b: f64) -> Result<MathResult, oneshot::Canceled> {
        self.send(MathRequest::subtract(a, b)).await
    }

    pub async fn multiply(&mut self, a: f64, b: f64) -> Result<MathResult, oneshot::Canceled> {
        self.send(MathRequest::multiply(a, b)).await
    }

    pub async fn divide(&mut self, a: f64, b: f64) -> Result<MathResult, oneshot::Canceled> {
        self.send(MathRequest::divide(a, b)).await
    }
}


async fn process_responses(incoming_requests: MsgReceiver) {
    // First lets connect to the server and split our stream into read and write
    let mut stream = TcpStream::connect("127.0.0.1:7878").await.unwrap();
    let (read_stream, write_stream) = stream.split();

    // Lets take that write stream and pass it to a SerealSink which will take in Messages
    // and serialize them to send them down the tcp sink
    let mut server_sink = SerealSink::new(write_stream);

    // Now lets take that read stream, and pass it to a SerealStreamer which will read input
    // from the stream and deserialize it into Messages.
    // We map these messages to the Input enum
    let results_stream = SerealStreamer::new(read_stream).map(Input::Result);

    // Now lets take the incoming requests stream and wrap them in the Input enum too.
    let requests_stream = incoming_requests.map(Input::Request);

    // This finally allows us to merge the two streams so that we're awaiting a message from either.
    // This way, we can receive a request from the client, or a result from the server immediately
    // as either happen.
    let mut combined_stream = futures::stream::select(results_stream, requests_stream);

    // We also need a way to route each incoming result back to the request it came from. Luckily
    // Each message has a u32 id associated with it. So we create a hashmap of the ids and oneshot
    // senders that we will use to send back the result in.
    let mut request_map: HashMap<u32, oneshot::Sender<MathResult>> = HashMap::new();

    // Now we're ready to receive results or requests from our stream.
    while let Some(input) = combined_stream.next().await {
        match input {
            // We've received a request from the client
            Input::Request((req, tx)) => {
                println!("{:?}", req);
                // Let's send the request to the server through the SerealSink
                server_sink.send(&req).await.unwrap();
                // And lets put that request id into the map so we can send the result back
                request_map.insert(req.id, tx);
            }

            // We've received a result from the server
            Input::Result(result) => {
                println!("{:?}", result);
                // Get the oneshot sender from the map that matches with the id
                let tx = request_map.remove(&result.id).unwrap();
                // Send the result back to the client
                tx.send(result).unwrap();
            }
        }
    }
}
