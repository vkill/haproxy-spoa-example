use crate::{Action, FrameHeader, FrameType, NBArgs, TypedData, VarintString};
use bytes::{Bytes, BytesMut};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use thiserror::Error;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum FramePayload {
    LIST_OF_MESSAGES(HashMap<VarintString, HashMap<VarintString, TypedData>>),
    LIST_OF_ACTIONS(Vec<Action>),
    KV_LIST(HashMap<VarintString, TypedData>),
}

impl FramePayload {
    pub fn get_list_of_messages(
        &self,
    ) -> Option<HashMap<VarintString, HashMap<VarintString, TypedData>>> {
        match self {
            Self::LIST_OF_MESSAGES(hash) => Some(hash.to_owned()),
            _ => None,
        }
    }

    pub fn get_list_of_actions(&self) -> Option<Vec<Action>> {
        match self {
            Self::LIST_OF_ACTIONS(actions) => Some(actions.to_owned()),
            _ => None,
        }
    }

    pub fn get_kv_list_value(&self, name: &str) -> Option<&TypedData> {
        match self {
            Self::KV_LIST(hash) => hash.get(&VarintString::new(name)),
            _ => None,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum FramePayloadType {
    LIST_OF_MESSAGES,
    LIST_OF_ACTIONS,
    KV_LIST,
}

#[derive(Error, PartialEq, Debug)]
pub enum FramePayloadParseError {
    #[error("invalid KV_LIST name")]
    InvalidKvListName,
    #[error("invalid KV_LIST value")]
    InvalidKvListValue,
    #[error("invalid LIST_OF_MESSAGES message_name")]
    InvalidListOfMessagesMessageName,
    #[error("invalid LIST_OF_MESSAGES nb_args")]
    InvalidListOfMessagesNBArgs,
    #[error("invalid LIST_OF_MESSAGES KV_LIST name")]
    InvalidListOfMessagesKvListName,
    #[error("invalid LIST_OF_MESSAGES KV_LIST value")]
    InvalidListOfMessagesKvListValue,
    #[error("invalid LIST_OF_ACTIONS")]
    InvalidListOfActions,
}

impl TryFrom<(&mut Bytes, &FrameHeader)> for FramePayload {
    type Error = FramePayloadParseError;

    fn try_from(t: (&mut Bytes, &FrameHeader)) -> Result<Self, FramePayloadParseError> {
        let (bytes, frame_header) = t;

        let r#type = match frame_header.r#type {
            FrameType::HAPROXY_HELLO => FramePayloadType::KV_LIST,
            FrameType::HAPROXY_DISCONNECT => FramePayloadType::KV_LIST,
            FrameType::AGENT_HELLO => FramePayloadType::KV_LIST,
            FrameType::AGENT_DISCONNECT => FramePayloadType::KV_LIST,
            FrameType::NOTIFY => FramePayloadType::LIST_OF_MESSAGES,
            FrameType::ACK => FramePayloadType::LIST_OF_ACTIONS,
            FrameType::UNSET => panic!("not support"),
        };

        match r#type {
            FramePayloadType::LIST_OF_MESSAGES => {
                let mut hash = HashMap::<VarintString, HashMap<VarintString, TypedData>>::new();

                while bytes.len() > 0 {
                    let name: VarintString = bytes
                        .try_into()
                        .map_err(|_| FramePayloadParseError::InvalidListOfMessagesMessageName)?;

                    let nb_args: NBArgs = bytes
                        .try_into()
                        .map_err(|_| FramePayloadParseError::InvalidListOfMessagesNBArgs)?;

                    let mut hash_kv = HashMap::<VarintString, TypedData>::new();
                    for _ in 0..nb_args.val() {
                        let name: VarintString = bytes
                            .try_into()
                            .map_err(|_| FramePayloadParseError::InvalidListOfMessagesKvListName)?;
                        let value: TypedData = bytes.try_into().map_err(|_| {
                            FramePayloadParseError::InvalidListOfMessagesKvListValue
                        })?;
                        hash_kv.insert(name, value);
                    }

                    hash.insert(name, hash_kv);
                }

                Ok(Self::LIST_OF_MESSAGES(hash))
            }
            FramePayloadType::LIST_OF_ACTIONS => {
                let mut actions: Vec<Action> = vec![];

                while bytes.len() > 0 {
                    let action: Action = bytes
                        .try_into()
                        .map_err(|_| FramePayloadParseError::InvalidListOfActions)?;
                    actions.push(action)
                }
                Ok(Self::LIST_OF_ACTIONS(actions))
            }
            FramePayloadType::KV_LIST => {
                let mut hash = HashMap::<VarintString, TypedData>::new();

                while bytes.len() > 0 {
                    let name: VarintString = bytes
                        .try_into()
                        .map_err(|_| FramePayloadParseError::InvalidKvListName)?;
                    let value: TypedData = bytes
                        .try_into()
                        .map_err(|_| FramePayloadParseError::InvalidKvListValue)?;
                    hash.insert(name, value);
                }

                Ok(Self::KV_LIST(hash))
            }
        }
    }
}

impl FramePayload {
    pub fn write_to(&self, buf: &mut BytesMut) {
        match self {
            Self::KV_LIST(hash) => {
                for (k, v) in hash {
                    k.write_to(buf);
                    v.write_to(buf);
                }
            }
            FramePayload::LIST_OF_MESSAGES(hash) => {
                for (k, hash_kv) in hash {
                    k.write_to(buf);
                    NBArgs::new(hash_kv.len() as u8).write_to(buf);

                    for (k, v) in hash_kv {
                        k.write_to(buf);
                        v.write_to(buf);
                    }
                }
            }
            FramePayload::LIST_OF_ACTIONS(actions) => {
                for action in actions {
                    action.write_to(buf);
                }
            }
        }

        ()
    }
}
