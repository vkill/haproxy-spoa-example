use crate::{TypedData, VarintString};
use bytes::{BufMut, Bytes, BytesMut};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::{TryFrom, TryInto};
use std::str;
use thiserror::Error;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum Action {
    SET_VAR {
        var_scope: ActionVarScope,
        var_name: VarintString,
        var_value: TypedData,
    },
    UNSET_VAR {
        var_scope: ActionVarScope,
        var_name: VarintString,
    },
}

impl Action {
    pub fn set_val(name: &str, value: TypedData, scope: ActionVarScope) -> Self {
        Self::SET_VAR {
            var_scope: scope,
            var_name: VarintString::new(name),
            var_value: value,
        }
    }

    pub fn unset_val(name: &str, value: TypedData, scope: ActionVarScope) -> Self {
        Self::UNSET_VAR {
            var_scope: scope,
            var_name: VarintString::new(name),
        }
    }
}

#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ActionType {
    SET_VAR = 1,
    UNSET_VAR = 2,
}

#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ActionVarScope {
    PROCESS = 0,
    SESSION = 1,
    TRANSACTION = 2,
    REQUEST = 3,
    RESPONSE = 4,
}

#[derive(Error, PartialEq, Debug)]
pub enum ActionParseError {
    #[error("Insufficient bytes")]
    InsufficientBytes,
}

impl TryFrom<&mut Bytes> for Action {
    type Error = ActionParseError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, ActionParseError> {
        unimplemented!()
    }
}

impl Action {
    pub fn write_to(&self, buf: &mut BytesMut) {
        unimplemented!()
    }
}
