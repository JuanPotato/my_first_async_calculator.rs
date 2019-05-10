/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::io;
use std::pin::Pin;

use futures::sink::Sink;
use futures::task::{Context, Poll};
use futures_util::io::AsyncWrite;

#[derive(Debug)]
pub struct PacketSink<A: AsyncWrite + Unpin> {
    async_writer: A,
    buffer: Vec<u8>,
    pos: usize,
}

impl<A: AsyncWrite + Unpin> PacketSink<A> {
    pub fn new(writer: A) -> PacketSink<A> {
        PacketSink {
            async_writer: writer,
            buffer: Vec::new(),
            pos: 0,
        }
    }
}

impl<A: AsyncWrite + Unpin> Sink<&[u8]> for PacketSink<A> {
    type SinkError = io::Error;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Result<(), io::Error>> {
        // Idk if I should be doing anything else :/
        Poll::Ready(Ok(()))
    }

    fn start_send(mut self: Pin<&mut Self>, item: &[u8]) -> Result<(), io::Error> {
        let len_bytes = (item.len() as u32).to_le_bytes();

        self.buffer.extend_from_slice(&len_bytes);
        self.buffer.extend_from_slice(item);

        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        let ps = self.get_mut();

        while ps.pos < ps.buffer.len() {
            let pin_writer = Pin::new(&mut ps.async_writer);
            let res = pin_writer.poll_write(cx, &ps.buffer[ps.pos..]);

            match res {
                Poll::Ready(Ok(num)) => {
                    // Should I do something if it's zero???
                    ps.pos += num;
                }

                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),

                Poll::Pending => return Poll::Pending,
            }
        }

        ps.buffer.truncate(0);
        ps.pos = 0;

        let pin_writer = Pin::new(&mut ps.async_writer);

        pin_writer.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::SinkError>> {
        let PacketSink { async_writer, .. } = self.get_mut();

        let pin_writer = Pin::new(async_writer);

        pin_writer.poll_flush(cx)
    }
}
