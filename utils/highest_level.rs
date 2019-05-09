// use byteorder::{ByteOrder, LE};
/*
use futures::async_stream;
use futures_util::io::AsyncReadExt;

const LENGTH_BYTES: usize = 4;

#[async_stream]
async fn get_packets<A: AsyncReadExt + Unpin>(stream: &mut A) -> Vec<u8> {
    let mut length_bytes = [0u8; LENGTH_BYTES];

    loop {
        if let Err(e) = await!(stream.read_exact(&mut length_bytes)) {
            // break; // commented out because clion throws a fit saying that im returning ()
                      // when I should be returning Vec<u8>. But we're in generator land
        }

        let length = LE::read_u32(&length_bytes) as usize;

        let mut data_bytes = vec![0u8; length];

        if let Err(e) = await!(stream.read_exact(&mut data_bytes)) {
            // break;
        }

        yield data_bytes;
    }
}
*/
// the fork that has async_stream updated to .await so its broken until the update is out