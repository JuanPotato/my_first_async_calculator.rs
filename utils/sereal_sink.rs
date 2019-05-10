/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::io;
use std::marker::PhantomData;
use std::pin::Pin;

use futures::sink::Sink;
use futures::task::{Context, Poll};
use futures_util::io::AsyncWrite;

use crate::packet_sink::PacketSink;
use crate::serialize::{Serializable, Serializer};

#[derive(Debug)]
pub struct SerealSink<S: Serializable + Unpin, A: AsyncWrite + Unpin>(PacketSink<A>, PhantomData<S>);

impl<S: Serializable + Unpin, A: AsyncWrite + Unpin> SerealSink<S, A> {
    pub fn new(writer: A) -> SerealSink<S, A> {
        SerealSink(PacketSink::new(writer), PhantomData)
    }
}


impl<S: Serializable + Unpin, A: AsyncWrite + Unpin> Sink<&S> for SerealSink<S, A> {
    type SinkError = io::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        let SerealSink(ps, _) = self.get_mut();
        let packet_sink = Pin::new(ps);

        packet_sink.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: &S) -> Result<(), io::Error> {
        let SerealSink(ps, _) = self.get_mut();
        let packet_sink = Pin::new(ps);

        let mut buf = Vec::new();
        buf.serialize(item)?;

        packet_sink.start_send(&buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        let SerealSink(ps, _) = self.get_mut();
        let packet_sink = Pin::new(ps);

        packet_sink.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::SinkError>> {
        let SerealSink(ps, _) = self.get_mut();
        let packet_sink = Pin::new(ps);

        packet_sink.poll_flush(cx)
    }
}
