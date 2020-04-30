use crate::{
    FrameFlags, FrameFlagsParseError, FramePayload, FramePayloadParseError, FramePayloadType,
    FrameType, FrameTypeParseError, VarintString,
};
use bytes::Bytes;
use std::convert::{TryFrom, TryInto};
use thiserror::Error;

#[derive(Debug)]
pub struct FrameStorage {
    pub r#type: FrameType,
    pub flags: FrameFlags,
    pub stream_id: VarintString,
    pub frame_id: VarintString,
    pub payload: FramePayload,
}

#[derive(Error, Debug)]
pub enum FrameStorageParseError {
    #[error("invalid type")]
    InvalidType(#[from] FrameTypeParseError),
    #[error("invalid flags")]
    InvalidFlags(#[from] FrameFlagsParseError),
    #[error("invalid stream_id")]
    InvalidStreamID,
    #[error("invalid frame_id")]
    InvalidFrameID,
    #[error("invalid payload")]
    InvalidPayload(#[from] FramePayloadParseError),
}

impl TryFrom<&mut Bytes> for FrameStorage {
    type Error = FrameStorageParseError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, FrameStorageParseError> {
        let r#type: FrameType = bytes.try_into()?;
        let flags: FrameFlags = bytes.try_into()?;

        let stream_id: VarintString = bytes
            .try_into()
            .map_err(|_| FrameStorageParseError::InvalidStreamID)?;

        let frame_id: VarintString = bytes
            .try_into()
            .map_err(|_| FrameStorageParseError::InvalidFrameID)?;

        let payload: FramePayload = match r#type {
            FrameType::HAPROXY_HELLO => (bytes, FramePayloadType::KV_LIST).try_into()?,
            _ => unimplemented!(),
        };

        let frame_storage = Self {
            r#type,
            flags,
            stream_id,
            frame_id,
            payload,
        };

        Ok(frame_storage)
    }
}

impl From<FrameStorage> for Bytes {
    fn from(storage: FrameStorage) -> Self {
        unimplemented!()
    }
}
