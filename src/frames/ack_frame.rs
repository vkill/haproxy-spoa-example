use crate::{Action, FrameFlags, FramePayload, FrameStorage, FrameType, Varint};

#[derive(Debug)]
pub struct AckFrame {
    pub stream_id: Varint,
    pub frame_id: Varint,
    pub payload: AckFramePayload,
}

impl AckFrame {
    pub fn new(stream_id: Varint, frame_id: Varint, payload: AckFramePayload) -> Self {
        Self {
            stream_id,
            frame_id,
            payload,
        }
    }
}

#[derive(Debug)]
pub struct AckFramePayload {
    pub actions: Vec<Action>,
}

impl AckFramePayload {
    pub fn new(actions: Vec<Action>) -> Self {
        Self { actions }
    }
}

impl From<AckFrame> for FrameStorage {
    fn from(frame: AckFrame) -> Self {
        let r#type = FrameType::ACK;
        let flags = FrameFlags::new(true, false);

        let stream_id = frame.stream_id;
        let frame_id = frame.frame_id;

        let payload = FramePayload::LIST_OF_ACTIONS(frame.payload.actions);

        let frame_storage = Self {
            r#type,
            flags,
            stream_id,
            frame_id,
            payload,
        };

        frame_storage
    }
}
