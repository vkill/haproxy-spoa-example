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

        let frame = Self {
            r#type,
            flags,
            stream_id,
            frame_id,
            payload,
        };

        Ok(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() -> anyhow::Result<()> {
        let bytes = b"\x01\0\0\0\x01\0\0\x12supported-versions\x08\x032.0\x0emax-frame-size\x03\xfc\xf0\x06\x0ccapabilities\x08\x10pipelining,async\tengine-id\x08$6bdec4ec-6b9a-4705-83f4-8817766c0c57";
        let mut bytes = Bytes::from_static(bytes);
        let bytes = &mut bytes;

        let frame_storage: FrameStorage = bytes.try_into()?;
        println!("{:?}", frame_storage);

        assert_eq!(frame_storage.r#type, FrameType::HAPROXY_HELLO);
        assert_eq!(frame_storage.flags.is_fin(), true);
        assert_eq!(frame_storage.flags.is_abort(), false);
        assert_eq!(frame_storage.stream_id.val(), "");
        assert_eq!(frame_storage.frame_id.val(), "");

        Ok(())
    }
}
