use crate::{TypedData, VarintString};
use bytes::Bytes;
use std::collections::HashMap;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum FramePayload {
    LIST_OF_MESSAGES,
    LIST_OF_ACTIONS,
    KV_LIST(Vec<HashMap<VarintString, TypedData>>),
}

#[allow(non_camel_case_types)]
pub enum FramePayloadType {
    LIST_OF_MESSAGES,
    LIST_OF_ACTIONS,
    KV_LIST,
}

#[derive(Error, PartialEq, Debug)]
pub enum FramePayloadParseError {
    #[error("Insufficient bytes")]
    InsufficientBytes,

    #[error("invalid")]
    Invalid,
}

impl TryFrom<(&mut Bytes, FramePayloadType)> for FramePayload {
    type Error = FramePayloadParseError;

    fn try_from(t: (&mut Bytes, FramePayloadType)) -> Result<Self, FramePayloadParseError> {
        let (bytes, r#type) = t;

        unimplemented!()
    }
}
