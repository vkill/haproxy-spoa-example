use crate::{FrameFlags, FrameFlagsFromError, FrameType, FrameTypeFromError};
use bytes::Bytes;
use std::convert::{TryFrom, TryInto};
use thiserror::Error;

#[derive(Debug)]
pub struct FrameStorage {
    pub frame_type: FrameType,
    pub frame_flags: FrameFlags,
}

#[derive(Error, Debug)]
pub enum FrameStorageFromError {
    #[error("invalid frame type")]
    InvalidFrameType(#[from] FrameTypeFromError),
    #[error("invalid frame flags")]
    InvalidFrameFlags(#[from] FrameFlagsFromError),
}

impl TryFrom<&mut Bytes> for FrameStorage {
    type Error = FrameStorageFromError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, FrameStorageFromError> {
        let frame_type: FrameType = bytes.try_into()?;
        let frame_flags: FrameFlags = bytes.try_into()?;

        let frame = Self {
            frame_type,
            frame_flags,
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

        assert_eq!(frame_storage.frame_type, FrameType::HAPROXY_HELLO);
        assert_eq!(frame_storage.frame_flags.is_fin(), true);
        assert_eq!(frame_storage.frame_flags.is_abort(), false);

        Ok(())
    }
}
