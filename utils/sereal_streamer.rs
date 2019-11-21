/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::io::Cursor;
use std::pin::Pin;
use std::marker::PhantomData;

use futures::stream::Stream;
use futures::task::{Context, Poll};
use tokio::io::AsyncRead;

use crate::deserialize::{Deserializable, Deserializer};
use crate::packet_streamer::PacketStreamer;

#[derive(Debug)]
pub struct SerealStreamer<D: Deserializable + Unpin, A: AsyncRead + Unpin>(PacketStreamer<A>, PhantomData<D>);

impl<D: Deserializable + Unpin, A: AsyncRead + Unpin> SerealStreamer<D, A> {
    pub fn new(reader: A) -> SerealStreamer<D, A> {
        SerealStreamer(PacketStreamer::new(reader), PhantomData)
    }
}

impl<D: Deserializable + Unpin, A: AsyncRead + Unpin> Stream for SerealStreamer<D, A> {
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
