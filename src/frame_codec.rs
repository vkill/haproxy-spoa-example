use bytes::{Buf, BufMut, Bytes, BytesMut};
use futures_codec::{Decoder, Encoder};

const U32_LENGTH: usize = std::mem::size_of::<u32>();

pub struct FrameCodec();

impl Encoder for FrameCodec {
    type Item = Bytes;
    type Error = anyhow::Error;

    fn encode(&mut self, src: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(U32_LENGTH + src.len());
        dst.put_u32(src.len() as u32);
        dst.extend_from_slice(&src);
        Ok(())
    }
}

impl Decoder for FrameCodec {
    type Item = Bytes;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < U32_LENGTH {
            return Ok(None);
        }

        let mut len_bytes = [0u8; U32_LENGTH];
        len_bytes.copy_from_slice(&src[..U32_LENGTH]);
        let len = u32::from_be_bytes(len_bytes) as usize;

        if src.len() - U32_LENGTH >= len {
            // Skip the length header we already read.
            src.advance(U32_LENGTH);
            Ok(Some(src.split_to(len).freeze()))
        } else {
            Ok(None)
        }
    }
}
