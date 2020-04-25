use bytes::Bytes;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use thiserror::Error;

#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum FrameType {
    UNSET = 0,
    HAPROXY_HELLO = 1,
    HAPROXY_DISCONNECT = 2,
    NOTIFY = 3,
    AGENT_HELLO = 101,
    AGENT_DISCONNECT = 102,
    ACK = 103,
}

#[derive(Error, PartialEq, Debug)]
pub enum FrameTypeFromError {
    #[error("Insufficient bytes")]
    InsufficientBytes,

    #[error("invalid")]
    Invalid,
}

impl TryFrom<&mut Bytes> for FrameType {
    type Error = FrameTypeFromError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, FrameTypeFromError> {
        if bytes.len() < 1 {
            return Err(FrameTypeFromError::InsufficientBytes);
        }
        let bytes = bytes.split_to(1);
        let r#u8 = u8::from_be_bytes([bytes[0]]);
        let r#type = Self::try_from(r#u8).map_err(|_| FrameTypeFromError::Invalid)?;
        Ok(r#type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_from() -> anyhow::Result<()> {
        let mut bytes = Bytes::from_static(b"\x01");
        let bytes = &mut bytes;
        let frame_type: FrameType = bytes.try_into()?;
        assert_eq!(frame_type, FrameType::HAPROXY_HELLO);

        let mut bytes = Bytes::from_static(b"\x00");
        let bytes = &mut bytes;
        let frame_type: FrameType = bytes.try_into()?;
        assert_eq!(frame_type, FrameType::UNSET);

        let mut bytes = Bytes::from_static(b"\x65");
        let bytes = &mut bytes;
        let frame_type: FrameType = bytes.try_into()?;
        assert_eq!(frame_type, FrameType::AGENT_HELLO);

        let mut bytes = Bytes::from_static(b"\xff");
        let bytes = &mut bytes;
        if let Err(e) = FrameType::try_from(bytes) {
            assert_eq!(e, FrameTypeFromError::Invalid);
        } else {
            assert!(false, "should err");
        }

        Ok(())
    }
}
