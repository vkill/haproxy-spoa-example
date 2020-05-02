use bytes::{BufMut, Bytes, BytesMut};
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug)]
pub struct NBArgs(u8);

impl NBArgs {
    pub fn new(val: u8) -> Self {
        Self(val)
    }

    pub fn val(&self) -> u8 {
        self.0
    }
}

#[derive(Error, PartialEq, Debug)]
pub enum NBArgsParseError {
    #[error("Insufficient bytes")]
    InsufficientBytes,
}

impl TryFrom<&mut Bytes> for NBArgs {
    type Error = NBArgsParseError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, NBArgsParseError> {
        if bytes.len() < 1 {
            return Err(NBArgsParseError::InsufficientBytes);
        }
        let b = bytes.split_to(1);
        Ok(Self::new(b[0]))
    }
}

impl NBArgs {
    pub fn write_to(self, buf: &mut BytesMut) {
        buf.put_u8(self.0);
        ()
    }
}
