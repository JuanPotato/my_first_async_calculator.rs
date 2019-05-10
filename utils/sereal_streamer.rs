use std::io::Cursor;
use std::pin::Pin;
use std::marker::PhantomData;

use futures::stream::Stream;
use futures::task::{Context, Poll};
use futures_util::io::AsyncRead;

use crate::deserialize::{Deserializable, Deserializer};
use crate::packet_streamer::PacketStreamer;

#[derive(Debug)]
pub struct SerealStreamer<A: AsyncRead + Unpin, D: Deserializable + Unpin>(PacketStreamer<A>, PhantomData<D>);

impl<A: AsyncRead + Unpin,D: Deserializable + Unpin> SerealStreamer<A, D> {
    pub fn new(reader: A) -> SerealStreamer<A, D> {
        SerealStreamer(PacketStreamer::new(reader), PhantomData)
    }
}

impl<A: AsyncRead + Unpin, D: Deserializable + Unpin> Stream for SerealStreamer<A, D> {
    type Item = D;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<D>> {
        let SerealStreamer(packets, _) = self.get_mut();

        match Pin::new(packets).poll_next(cx) {
            Poll::Ready(Some(packet)) => {
                let mut cursor_bytes = Cursor::new(packet);
                Poll::Ready(Some(cursor_bytes.deserialize().unwrap()))
            }

            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

