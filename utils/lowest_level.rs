use std::pin::Pin;

use byteorder::{ByteOrder, LE};
use futures::stream::Stream;
use futures::task::{Context, Poll};
use futures_util::io::AsyncRead;

const LENGTH_BYTES: usize = 4;

#[derive(Debug)]
enum PacketState {
    Length,
    Data(usize),
}

#[derive(Debug)]
/// Streams Packets that are structured with a u32 length then that many bytes following
pub struct PacketStreamer<A: AsyncRead + Unpin> {
    async_reader: A,
    state: PacketState,
    buffer: Vec<u8>,
    pos: usize,
}

impl<A: AsyncRead + Unpin> PacketStreamer<A> {
    pub fn new(reader: A) -> PacketStreamer<A> {
        PacketStreamer {
            async_reader: reader,
            state: PacketState::Length,
            buffer: vec![0u8; LENGTH_BYTES],
            pos: 0,
        }
    }
}

impl<A: AsyncRead + Unpin> Stream for PacketStreamer<A> {
    type Item = Vec<u8>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        // Get ourself (good pun) out of a pin
        // basically &mut self
        let s = Pin::get_mut(self);

        // Depending on what state we are in, we are going to try to read a different amount
        // of bytes.
        let mut target_len = match s.state {
            PacketState::Length => LENGTH_BYTES,
            PacketState::Data(len) => len,
        };


        // Outside loop to repeat this in case we finish the length state and can move on to data
        loop {

            // We're going to read as much as we can up till our target
            while s.pos < target_len {
                let reader = Pin::new(&mut s.async_reader);
                // Start reading after pos to only read remaining empty bytes
                let res = reader.poll_read(cx, &mut s.buffer[s.pos..]);

                match res {
                    Poll::Ready(Ok(num)) => {
                        // If we didn't read anything, they've disconnected
                        if num == 0 {
                            return Poll::Ready(None);
                        }

                        // Otherwise, we read some amount and we'll increment the position and
                        // loop back to try to read some more
                        s.pos += num;
                    }

                    // If we received an error, we're definitely not going to be able to continue
                    // TODO: use e?
                    Poll::Ready(Err(_e)) => return Poll::Ready(None),

                    // Return pending if we got pending
                    Poll::Pending => return Poll::Pending,
                }
            }

            // Once we've reached this part, that means the position is equal to the target length
            // and we've read all we need for our state
            match s.state {
                PacketState::Length => {
                    // If we were in the length state, read the length from the bytes
                    let length = LE::read_u32(&s.buffer) as usize;

                    // transition to the data state and supply the length of the data we need
                    s.state = PacketState::Data(length);

                    // Allocate the needed space all at once with reserve
                    // then resize the buffer to be at the length (we already got the capacity)
                    // we need so that poll_read actually fills it
                    s.buffer.reserve(length);
                    s.buffer.resize(length, 0);

                    // Reset the position since we're reading new data
                    // We are able to overwrite the data received from the length bytes because
                    // we resize the array to a length that we know we will receive. The bytes
                    // from before will all either be overwritten if our new data is larger than
                    // the length bytes, or be chopped off due to resize
                    s.pos = 0;

                    // set the new target length for when we loop back and try to poll_read
                    target_len = length;
                }

                PacketState::Data(_len) => {
                    // If we were in the data state, we finished reading a packet and can return it

                    // As before, we're going to change the state back to Length so we can read new
                    // packets
                    s.state = PacketState::Length;

                    // Reset the position so we can read from the beginning for length
                    s.pos = 0;

                    // And finally, replace the buffer in the struct with a newly created buffer
                    // with the appropriate size for reading the length.
                    let packet = std::mem::replace(&mut s.buffer, vec![0u8; LENGTH_BYTES]);

                    // Return the buffer that was in the struct
                    return Poll::Ready(Some(packet));
                }
            }
        }
    }
}

