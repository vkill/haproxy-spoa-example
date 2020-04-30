use crate::{AgentHelloFrame, FrameStorage, FrameStorageParseError, FrameType, HAProxyHelloFrame};
use bytes::Bytes;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug)]
pub struct Frame {}
impl Frame {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Error, Debug)]
pub enum FrameHandleError {
    #[error("to FrameStorage failed")]
    ToFrameStorageFailed(#[from] FrameStorageParseError),
}

impl Frame {
    pub fn handle(&self, bytes: &mut Bytes) -> Result<Option<Bytes>, FrameHandleError> {
        let frame_storage = FrameStorage::try_from(bytes)?;

        let frame_storage_out = match frame_storage.r#type {
            FrameType::HAPROXY_HELLO => {
                if let Ok(haproxy_hello_frame) = HAProxyHelloFrame::try_from(frame_storage) {
                    let agent_hello_frame =
                        AgentHelloFrame::from_haproxy_hello_frame(haproxy_hello_frame);
                    Some(FrameStorage::from(agent_hello_frame))
                } else {
                    unimplemented!()
                }
            }
            _ => unimplemented!(),
        };

        Ok(frame_storage_out.map(|x| x.into()))
    }
}
