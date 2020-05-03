use crate::{FrameFlags, FrameFlagsParseError, FrameType, FrameTypeParseError, Varint};
use bytes::{Bytes, BytesMut};
use std::convert::{TryFrom, TryInto};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct FrameHeader {
    pub r#type: FrameType,
    pub flags: FrameFlags,
    pub stream_id: Varint,
    pub frame_id: Varint,
}

#[derive(Error, Debug)]
pub enum FrameHeaderParseError {
    #[error("invalid type")]
    InvalidType(#[from] FrameTypeParseError),
    #[error("invalid flags")]
    InvalidFlags(#[from] FrameFlagsParseError),
    #[error("invalid stream_id")]
    InvalidStreamID,
    #[error("invalid frame_id")]
    InvalidFrameID,
}

impl TryFrom<&mut Bytes> for FrameHeader {
    type Error = FrameHeaderParseError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, FrameHeaderParseError> {
        let r#type: FrameType = bytes.try_into()?;
        let flags: FrameFlags = bytes.try_into()?;

        let stream_id: Varint = bytes
            .try_into()
            .map_err(|_| FrameHeaderParseError::InvalidStreamID)?;

        let frame_id: Varint = bytes
            .try_into()
            .map_err(|_| FrameHeaderParseError::InvalidFrameID)?;

        let frame_header = Self {
            r#type,
            flags,
            stream_id,
            frame_id,
        };

        Ok(frame_header)
    }
}

impl From<FrameHeader> for BytesMut {
    fn from(header: FrameHeader) -> Self {
        let mut buf = BytesMut::new();

        header.r#type.write_to(&mut buf);
        header.flags.write_to(&mut buf);
        buf.extend_from_slice(BytesMut::from(header.stream_id).as_ref());
        buf.extend_from_slice(BytesMut::from(header.frame_id).as_ref());

        buf
    }
}
