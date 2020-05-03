use crate::{FrameHeader, FramePayload};
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug)]
pub struct HAProxyDisconnectFrame {
    pub payload: HAProxyDisconnectFramePayload,
}

make_frame_kv_list_payload! {
#[derive(Debug)]
pub struct HAProxyDisconnectFramePayload {
    pub status_code: u32,
    pub message: String,
}
}

#[derive(Error, Debug)]
#[allow(non_camel_case_types)]
pub enum HAProxyDisconnectFrameParseError {
    #[error("STREAM-ID and FRAME-ID are must be set 0")]
    Invalid_STREAM_ID,
    #[error("STREAM-ID and FRAME-ID are must be set 0")]
    Invalid_FRAME_ID,
    #[error("field {0} not found")]
    FieldNotFound(String),
    #[error("field {0} value invalid")]
    FieldValueInvalid(String),
}

impl TryFrom<(FrameHeader, FramePayload)> for HAProxyDisconnectFrame {
    type Error = HAProxyDisconnectFrameParseError;
    fn try_from(t: (FrameHeader, FramePayload)) -> Result<Self, HAProxyDisconnectFrameParseError> {
        let (frame_header, frame_payload) = t;

        if frame_header.stream_id.u64_val() != 0 {
            return Err(HAProxyDisconnectFrameParseError::Invalid_STREAM_ID);
        }
        if frame_header.frame_id.u64_val() != 0 {
            return Err(HAProxyDisconnectFrameParseError::Invalid_FRAME_ID);
        }

        let status_code_name = &HAProxyDisconnectFramePayload::status_code_name();
        let status_code = frame_payload
            .get_kv_list_value(status_code_name)
            .ok_or(HAProxyDisconnectFrameParseError::FieldNotFound(
                status_code_name.to_owned(),
            ))?
            .get_u32()
            .ok_or(HAProxyDisconnectFrameParseError::FieldValueInvalid(
                status_code_name.to_owned(),
            ))?;

        let message_name = &HAProxyDisconnectFramePayload::message_name();
        let message = frame_payload
            .get_kv_list_value(message_name)
            .ok_or(HAProxyDisconnectFrameParseError::FieldNotFound(
                message_name.to_owned(),
            ))?
            .get_string()
            .ok_or(HAProxyDisconnectFrameParseError::FieldValueInvalid(
                message_name.to_owned(),
            ))?
            .val();

        let payload = HAProxyDisconnectFramePayload {
            status_code: status_code.to_owned(),
            message: message.to_owned(),
        };

        let frame = Self { payload };

        Ok(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FrameType;
    use bytes::Bytes;
    use std::convert::TryInto;

    /*
    b"\x02\0\0\0\x01\0\0\x0bstatus-code\x03\n\x07message\x08\x1bfragmentation not supported"
    b"\x02\0\0\0\x01\0\0\x0bstatus-code\x03\x04\x07message\x08\x16invalid frame received"
    */

    #[test]
    fn test_from() -> anyhow::Result<()> {
        let bytes = b"\x02\0\0\0\x01\0\0\x0bstatus-code\x03\n\x07message\x08\x1bfragmentation not supported";
        let mut bytes = Bytes::from_static(bytes);
        let bytes = &mut bytes;

        let frame_header: FrameHeader = bytes.try_into()?;
        println!("{:?}", frame_header);

        let frame_payload: FramePayload = (bytes, &frame_header.r#type).try_into()?;
        println!("{:?}", frame_payload);

        assert_eq!(frame_header.r#type, FrameType::HAPROXY_DISCONNECT);
        assert_eq!(frame_header.flags.is_fin(), true);
        assert_eq!(frame_header.flags.is_abort(), false);
        assert_eq!(frame_header.stream_id.u64_val(), 0);
        assert_eq!(frame_header.frame_id.u64_val(), 0);

        let frame = HAProxyDisconnectFrame::try_from((frame_header, frame_payload))?;
        println!("{:?}", frame);

        assert_eq!(frame.payload.status_code, 10);
        assert_eq!(frame.payload.message, "fragmentation not supported");

        Ok(())
    }
}
