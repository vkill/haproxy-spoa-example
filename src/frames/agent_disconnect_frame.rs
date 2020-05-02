use crate::{
    FrameFlags, FrameKnownError, FramePayload, FrameStorage, FrameType, TypedData, VarintString,
};
use std::collections::HashMap;
use std::string::ToString;

#[derive(Debug)]
pub struct AgentDisconnectFrame {
    pub payload: AgentDisconnectFramePayload,
}

impl AgentDisconnectFrame {
    pub fn new(payload: AgentDisconnectFramePayload) -> Self {
        Self { payload }
    }
}

make_frame_kv_list_payload! {
#[derive(Debug)]
pub struct AgentDisconnectFramePayload {
    pub status_code: u32,
    pub message: String,
}
}

impl AgentDisconnectFramePayload {
    pub fn new(status_code: u32, message: String) -> Self {
        Self {
            status_code,
            message,
        }
    }

    pub fn from_frame_known_error(frame_known_error: FrameKnownError) -> Self {
        Self {
            status_code: frame_known_error.clone().into(),
            message: frame_known_error.to_string(),
        }
    }
}

impl From<AgentDisconnectFrame> for FrameStorage {
    fn from(frame: AgentDisconnectFrame) -> Self {
        let r#type = FrameType::AGENT_HELLO;
        let flags = FrameFlags::new(true, false);

        let stream_id = VarintString::new("");
        let frame_id = VarintString::new("");

        let mut h = HashMap::<String, TypedData>::new();
        h.insert(
            AgentDisconnectFramePayload::status_code_name(),
            TypedData::UINT32(frame.payload.status_code),
        );
        h.insert(
            AgentDisconnectFramePayload::message_name(),
            TypedData::STRING(VarintString::new(frame.payload.message.as_str())),
        );
        let payload = FramePayload::KV_LIST(h);

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
