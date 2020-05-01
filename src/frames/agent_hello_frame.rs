use super::{HAProxyHelloFrameCapability, HAProxyHelloFramePayload};
use crate::{
    FrameFlags, FramePayload, FrameStorage, FrameType, SupportVersion, TypedData, VarintString,
};
use std::collections::HashMap;
use std::string::ToString;

#[derive(Debug)]
pub struct AgentHelloFrame {
    pub payload: AgentHelloFramePayload,
}

impl AgentHelloFrame {
    pub fn new(payload: AgentHelloFramePayload) -> Self {
        Self { payload }
    }
}

make_frame_kv_list_payload! {
#[derive(Debug)]
pub struct AgentHelloFramePayload {
    pub version: SupportVersion,
    pub max_frame_size: u32,
    pub capabilities: Vec<HAProxyHelloFrameCapability>,
}
}

impl AgentHelloFramePayload {
    pub fn new(
        version: SupportVersion,
        max_frame_size: u32,
        capabilities: Vec<HAProxyHelloFrameCapability>,
    ) -> Self {
        Self {
            version,
            max_frame_size,
            capabilities,
        }
    }

    pub fn from_haproxy_hello_frame_payload(
        haproxy_hello_frame_payload: HAProxyHelloFramePayload,
    ) -> Self {
        Self {
            version: haproxy_hello_frame_payload
                .supported_versions
                .first()
                .unwrap()
                .to_owned(),
            max_frame_size: haproxy_hello_frame_payload.max_frame_size,
            capabilities: haproxy_hello_frame_payload.capabilities,
        }
    }
}

impl From<AgentHelloFrame> for FrameStorage {
    fn from(frame: AgentHelloFrame) -> Self {
        let r#type = FrameType::AGENT_HELLO;
        let flags = FrameFlags::new(true, false);

        let stream_id = VarintString::new("");
        let frame_id = VarintString::new("");

        let mut h = HashMap::<String, TypedData>::new();
        h.insert(
            AgentHelloFramePayload::version_name(),
            TypedData::STRING(VarintString::new(
                frame.payload.version.to_string().as_str(),
            )),
        );
        h.insert(
            AgentHelloFramePayload::max_frame_size_name(),
            TypedData::UINT32(frame.payload.max_frame_size),
        );
        h.insert(
            AgentHelloFramePayload::capabilities_name(),
            TypedData::STRING(VarintString::new(
                frame
                    .payload
                    .capabilities
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .as_str(),
            )),
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
