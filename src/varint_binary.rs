use crate::Varint;
use bytes::Bytes;
use std::convert::{TryFrom, TryInto};
use thiserror::Error;

#[derive(Debug)]
pub struct VarintBinary(Vec<u8>);

impl VarintBinary {
    pub fn val(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[derive(Error, PartialEq, Debug)]
pub enum VarintBinaryParseError {
    #[error("Insufficient bytes")]
    InsufficientBytes,
}

impl TryFrom<&mut Bytes> for VarintBinary {
    type Error = VarintBinaryParseError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, VarintBinaryParseError> {
        let len: Varint = bytes
            .try_into()
            .map_err(|_| VarintBinaryParseError::InsufficientBytes)?;
        let len = len.u64_val() as usize;
        let a: Vec<u8> = if len == 0 {
            vec![]
        } else {
            if bytes.len() < len {
                return Err(VarintBinaryParseError::InsufficientBytes);
            }
            let b = bytes.split_to(len);
            b.to_vec()
        };

        Ok(Self(a))
    }
}
