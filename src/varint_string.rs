use crate::Varint;
use bytes::Bytes;
use std::convert::{TryFrom, TryInto};
use std::str;
use thiserror::Error;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct VarintString(String);

impl VarintString {
    pub fn new(val: &str) -> Self {
        Self(val.to_owned())
    }

    pub fn val(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Error, PartialEq, Debug)]
pub enum VarintStringParseError {
    #[error("Insufficient bytes")]
    InsufficientBytes,

    #[error("Invalid bytes")]
    Invalid,
}

impl TryFrom<&mut Bytes> for VarintString {
    type Error = VarintStringParseError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, VarintStringParseError> {
        let len: Varint = bytes
            .try_into()
            .map_err(|_| VarintStringParseError::InsufficientBytes)?;
        let len = len.u64_val() as usize;
        let s = if len == 0 {
            "".to_owned()
        } else {
            if bytes.len() < len {
                return Err(VarintStringParseError::InsufficientBytes);
            }
            let b = bytes.split_to(len);
            let s = str::from_utf8(&b[..]).map_err(|_| VarintStringParseError::Invalid)?;
            s.to_owned()
        };

        Ok(Self(s))
    }
}
