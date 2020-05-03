use crate::{
    AckFrame, AckFramePayload, Action, ActionVarScope, AgentDisconnectFrame,
    AgentDisconnectFramePayload, AgentHelloFrame, AgentHelloFramePayload, FrameKnownError,
    FrameStorage, FrameStorageParseError, FrameType, HAProxyDisconnectFrame, HAProxyHelloFrame,
    HAProxyHelloFrameCapability, NotifyFrame, SupportVersion, TypedData, VarintString,
};
use bytes::{Bytes, BytesMut};
use log::*;
use semver::Version;
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
    pub fn handle(&self, bytes: &mut Bytes) -> Result<(Option<BytesMut>, bool), FrameHandleError> {
        let frame_storage = FrameStorage::try_from(bytes)?;
        debug!("read frame_storage: {:?}", frame_storage);

        let mut do_close = false;

        let frame_storage_out = match frame_storage.r#type {
            FrameType::HAPROXY_HELLO => match HAProxyHelloFrame::try_from(frame_storage) {
                Ok(haproxy_hello_frame) => {
                    let frame = AgentHelloFrame::new(AgentHelloFramePayload::new(
                        SupportVersion::new(Version::new(2, 0, 0)),
                        haproxy_hello_frame.payload.max_frame_size,
                        vec![
                            HAProxyHelloFrameCapability::r#async,
                            HAProxyHelloFrameCapability::pipelining,
                            HAProxyHelloFrameCapability::fragmentation,
                        ],
                    ));
                    if haproxy_hello_frame.payload.healthcheck == Some(true) {
                        do_close = true
                    }
                    Some(FrameStorage::from(frame))
                }
                Err(e) => {
                    error!("make HAProxyHelloFrame failed, error: {}", e);

                    let frame = AgentDisconnectFrame::new(
                        AgentDisconnectFramePayload::from_frame_known_error(
                            FrameKnownError::invalid_frame_received,
                        ),
                    );

                    do_close = true;
                    Some(FrameStorage::from(frame))
                }
            },
            FrameType::HAPROXY_DISCONNECT => {
                match HAProxyDisconnectFrame::try_from(frame_storage) {
                    Ok(haproxy_disconnect_frame) => {
                        let frame = AgentDisconnectFrame::new(AgentDisconnectFramePayload::new(
                            haproxy_disconnect_frame.payload.status_code,
                            haproxy_disconnect_frame.payload.message,
                        ));
                        do_close = true;
                        Some(FrameStorage::from(frame))
                    }
                    Err(e) => {
                        error!("make HAProxyDisconnectFrame failed, error: {}", e);

                        let frame = AgentDisconnectFrame::new(
                            AgentDisconnectFramePayload::from_frame_known_error(
                                FrameKnownError::invalid_frame_received,
                            ),
                        );
                        do_close = true;
                        Some(FrameStorage::from(frame))
                    }
                }
            }
            FrameType::NOTIFY => match NotifyFrame::try_from(frame_storage) {
                Ok(notify_frame) => {
                    let mut actions: Vec<Action> = vec![];
                    if let Some(_) = notify_frame
                        .payload
                        .messages
                        .get(&VarintString::new("msg-1"))
                    {
                        actions.push(Action::set_val(
                            ActionVarScope::TRANSACTION,
                            VarintString::new("var_name_1"),
                            TypedData::STRING(VarintString::new("var-value-1")),
                        ))
                    }

                    let frame = AckFrame::new(
                        notify_frame.stream_id,
                        notify_frame.frame_id,
                        AckFramePayload::new(actions),
                    );

                    Some(FrameStorage::from(frame))
                }
                Err(e) => {
                    error!("make NotifyFrame failed, error: {}", e);

                    let frame = AgentDisconnectFrame::new(
                        AgentDisconnectFramePayload::from_frame_known_error(
                            FrameKnownError::invalid_frame_received,
                        ),
                    );

                    do_close = true;
                    Some(FrameStorage::from(frame))
                }
            },
            FrameType::UNSET => unimplemented!(),
            _ => panic!("not support"),
        };

        info!(
            "write frame_storage: {:?}, do_close: {}",
            frame_storage_out, do_close
        );

        Ok((frame_storage_out.map(|x| x.into()), do_close))
    }
}
