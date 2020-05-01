use crate::{TypedData, VarintString};
use bytes::{Bytes, BytesMut};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use thiserror::Error;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum FramePayload {
    LIST_OF_MESSAGES,
    LIST_OF_ACTIONS,
    KV_LIST(HashMap<String, TypedData>),
}

impl FramePayload {
    pub fn get_kv_value(&self, name: &str) -> Option<&TypedData> {
        match self {
            Self::KV_LIST(v) => v.get(name),
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
}

impl TryFrom<(&mut Bytes, FramePayloadType)> for FramePayload {
    type Error = FramePayloadParseError;

    fn try_from(t: (&mut Bytes, FramePayloadType)) -> Result<Self, FramePayloadParseError> {
        let (bytes, r#type) = t;

        match r#type {
            FramePayloadType::LIST_OF_MESSAGES => unimplemented!(),
            FramePayloadType::LIST_OF_ACTIONS => unimplemented!(),
            FramePayloadType::KV_LIST => {
                let mut maps = HashMap::<String, TypedData>::new();

                while bytes.len() > 0 {
                    let name: VarintString = bytes
                        .try_into()
                        .map_err(|_| FramePayloadParseError::InvalidKvListName)?;
                    let value: TypedData = bytes
                        .try_into()
                        .map_err(|_| FramePayloadParseError::InvalidKvListValue)?;
                    maps.insert(name.val().to_owned(), value);
                }

                Ok(Self::KV_LIST(maps))
            }
        }
    }
}

impl FramePayload {
    pub fn write_to(&self, buf: &mut BytesMut) {
        match self {
            Self::KV_LIST(h) => {
                for (k, v) in h {
                    VarintString::new(k).write_to(buf);
                    v.write_to(buf);
                }
            }
            _ => unimplemented!(),
        }

        ()
    }
}
