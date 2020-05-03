use crate::{FrameFlags, FrameStorage, TypedData, Varint, VarintString};
use std::collections::HashMap;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug)]
pub struct NotifyFrame {
    pub flags: FrameFlags,

    /*
    first req
        msg-1: stream_id 0 frame_id 1
        msg-2: stream_id 0 frame_id 2
    second req
        msg-1: stream_id 3 frame_id 1
        msg-2: stream_id 3 frame_id 2
    third req
        msg-1: stream_id 6 frame_id 1
        msg-2: stream_id 6 frame_id 2
    forth req
        msg-1: stream_id 9 frame_id 1
        msg-2: stream_id 9 frame_id 2
    */
    pub stream_id: Varint,
    pub frame_id: Varint,
    pub payload: NotifyFramePayload,
}

#[derive(Debug)]
pub struct NotifyFramePayload {
    pub messages: HashMap<VarintString, HashMap<VarintString, TypedData>>,
}

#[derive(Error, Debug)]
#[allow(non_camel_case_types)]
pub enum NotifyFrameParseError {
    #[error("STREAM-ID and FRAME-ID must be set")]
    Invalid_STREAM_ID,
    #[error("STREAM-ID and FRAME-ID must be set")]
    Invalid_FRAME_ID,
    #[error("invalid payload")]
    Invalid_Payload,
}

impl TryFrom<FrameStorage> for NotifyFrame {
    type Error = NotifyFrameParseError;
    fn try_from(storage: FrameStorage) -> Result<Self, NotifyFrameParseError> {
        if storage.frame_id.u64_val() == 0 {
            return Err(NotifyFrameParseError::Invalid_FRAME_ID);
        }

        let messages = storage
            .payload
            .get_list_of_messages()
            .ok_or(NotifyFrameParseError::Invalid_Payload)?;

        let payload = NotifyFramePayload { messages: messages };

        let frame = Self {
            flags: storage.flags,
            stream_id: storage.stream_id,
            frame_id: storage.frame_id,
            payload,
        };

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
    b"\x03\0\0\0\x01\0\x01\x04demo\x02\narg_method\x08\x03GET\x08arg_path\x08\x01/"
    */

    #[test]
    fn test_from() -> anyhow::Result<()> {
        let bytes = b"\x03\0\0\0\x01\0\x01\x04demo\x02\narg_method\x08\x03GET\x08arg_path\x08\x01/";
        let mut bytes = Bytes::from_static(bytes);
        let bytes = &mut bytes;

        let frame_storage: FrameStorage = bytes.try_into()?;
        println!("{:?}", frame_storage);

        assert_eq!(frame_storage.r#type, FrameType::NOTIFY);
        assert_eq!(frame_storage.flags.is_fin(), true);
        assert_eq!(frame_storage.flags.is_abort(), false);
        assert_ne!(frame_storage.frame_id.u64_val(), 0);

        let frame = NotifyFrame::try_from(frame_storage)?;
        println!("{:?}", frame);

        assert_eq!(frame.payload.messages.len(), 1);

        let message = frame
            .payload
            .messages
            .get(&VarintString::new("demo"))
            .unwrap();

        assert_eq!(message.len(), 2);
        assert_eq!(
            message.get(&VarintString::new("arg_method")),
            Some(&TypedData::STRING(VarintString::new("GET")))
        );
        assert_eq!(
            message.get(&VarintString::new("arg_path")),
            Some(&TypedData::STRING(VarintString::new("/")))
        );

        Ok(())
    }
}
