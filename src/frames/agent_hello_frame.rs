use super::{HAProxyHelloFrame, HAProxyHelloFrameCapability};
use crate::{
    FrameFlags, FramePayload, FrameStorage, FrameType, SupportVersion, TypedData, VarintString,
};
use std::collections::HashMap;
use std::string::ToString;

#[derive(Debug)]
pub struct AgentHelloFrame {
    pub version: SupportVersion,
    pub max_frame_size: u32,
    pub capabilities: Vec<HAProxyHelloFrameCapability>,
}

impl AgentHelloFrame {
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

    pub fn from_haproxy_hello_frame(haproxy_hello_frame: HAProxyHelloFrame) -> Self {
        Self {
            version: haproxy_hello_frame
                .supported_versions
                .first()
                .unwrap()
                .to_owned(),
            max_frame_size: haproxy_hello_frame.max_frame_size,
            capabilities: haproxy_hello_frame.capabilities,
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
            "version".to_owned(),
            TypedData::STRING(VarintString::new(frame.version.to_string().as_str())),
        );
        h.insert(
            "max-frame-size".to_owned(),
            TypedData::UINT32(frame.max_frame_size),
        );
        h.insert(
            "capabilities".to_owned(),
            TypedData::STRING(VarintString::new(
                frame
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
