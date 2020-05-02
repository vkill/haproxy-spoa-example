use crate::{
    AgentDisconnectFrame, AgentDisconnectFramePayload, AgentHelloFrame, AgentHelloFramePayload,
    FrameKnownError, FrameStorage, FrameStorageParseError, FrameType, HAProxyHelloFrame,
};
use bytes::{Bytes, BytesMut};
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
    pub fn handle(&self, bytes: &mut Bytes) -> Result<Option<BytesMut>, FrameHandleError> {
        let frame_storage = FrameStorage::try_from(bytes)?;

        let frame_storage_out = match frame_storage.r#type {
            FrameType::HAPROXY_HELLO => {
                if let Ok(haproxy_hello_frame) = HAProxyHelloFrame::try_from(frame_storage) {
                    let frame = AgentHelloFrame::new(
                        AgentHelloFramePayload::from_haproxy_hello_frame_payload(
                            haproxy_hello_frame.payload,
                        ),
                    );
                    Some(FrameStorage::from(frame))
                } else {
                    let frame = AgentDisconnectFrame::new(
                        AgentDisconnectFramePayload::from_frame_known_error(
                            FrameKnownError::invalid_frame_received,
                        ),
                    );

                    Some(FrameStorage::from(frame))
                }
            }
            _ => unimplemented!(),
        };

        Ok(frame_storage_out.map(|x| x.into()))
    }
}
