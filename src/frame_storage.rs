use crate::{
    FrameFlags, FrameFlagsFromError, FrameType, FrameTypeFromError, Varint, VarintFromError,
};
use bytes::Bytes;
use std::convert::{TryFrom, TryInto};
use std::str;
use thiserror::Error;

#[derive(Debug)]
pub struct FrameStorage {
    pub r#type: FrameType,
    pub flags: FrameFlags,
    pub stream_id: Option<String>,
    pub frame_id: Option<String>,
}

#[derive(Error, Debug)]
pub enum FrameStorageFromError {
    #[error("invalid type")]
    InvalidType(#[from] FrameTypeFromError),
    #[error("invalid flags")]
    InvalidFlags(#[from] FrameFlagsFromError),
    #[error("invalid stream_id")]
    InvalidStreamID,
    #[error("invalid frame_id")]
    InvalidFrameID,
}

impl TryFrom<&mut Bytes> for FrameStorage {
    type Error = FrameStorageFromError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, FrameStorageFromError> {
        let r#type: FrameType = bytes.try_into()?;
        let flags: FrameFlags = bytes.try_into()?;

        let stream_id_len: Varint = bytes
            .try_into()
            .map_err(|_| FrameStorageFromError::InvalidStreamID)?;
        let stream_id_len = stream_id_len.u64_val() as usize;
        let stream_id: Option<String> = if stream_id_len == 0 {
            None
        } else {
            if bytes.len() < stream_id_len {
                return Err(FrameStorageFromError::InvalidStreamID);
            }
            let b = bytes.split_to(stream_id_len);
            let s = str::from_utf8(&b[..]).map_err(|_| FrameStorageFromError::InvalidStreamID)?;
            Some(s.to_owned())
        };

        let frame_id_len: Varint = bytes
            .try_into()
            .map_err(|_| FrameStorageFromError::InvalidFrameID)?;
        let frame_id_len = frame_id_len.u64_val() as usize;
        let frame_id: Option<String> = if frame_id_len == 0 {
            None
        } else {
            if bytes.len() < frame_id_len {
                return Err(FrameStorageFromError::InvalidFrameID);
            }
            let b = bytes.split_to(frame_id_len);
            let s = str::from_utf8(&b[..]).map_err(|_| FrameStorageFromError::InvalidFrameID)?;
            Some(s.to_owned())
        };

        let frame = Self {
            r#type,
            flags,
            stream_id,
            frame_id,
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

        assert_eq!(frame_storage.r#type, FrameType::HAPROXY_HELLO);
        assert_eq!(frame_storage.flags.is_fin(), true);
        assert_eq!(frame_storage.flags.is_abort(), false);
        assert_eq!(frame_storage.stream_id, None);
        assert_eq!(frame_storage.frame_id, None);

        Ok(())
    }
}
