use crate::{NBArgs, TypedData, VarintString};
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
    pub fn set_val(scope: ActionVarScope, name: VarintString, value: TypedData) -> Self {
        Self::SET_VAR {
            var_scope: scope,
            var_name: name,
            var_value: value,
        }
    }

    pub fn unset_val(scope: ActionVarScope, name: VarintString) -> Self {
        Self::UNSET_VAR {
            var_scope: scope,
            var_name: name,
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

    #[error("invalid type")]
    InvalidType,

    #[error("invalid nb_args")]
    InvalidNBArgs,

    #[error("invalid var scope")]
    InvalidVarScope,

    #[error("invalid var name")]
    InvalidVarName,

    #[error("invalid var value")]
    InvalidVarValue,
}

impl TryFrom<&mut Bytes> for Action {
    type Error = ActionParseError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, ActionParseError> {
        if bytes.len() < 1 {
            return Err(ActionParseError::InsufficientBytes);
        }
        let b = bytes.split_to(1);
        let r#u8 = u8::from_be_bytes([b[0]]);
        let r#type = ActionType::try_from(r#u8).map_err(|_| ActionParseError::InvalidType)?;

        let nb_args: NBArgs = bytes
            .try_into()
            .map_err(|_| ActionParseError::InvalidNBArgs)?;
        match r#type {
            ActionType::SET_VAR => {
                if nb_args.val() != 3 {
                    return Err(ActionParseError::InvalidNBArgs);
                }
            }
            ActionType::UNSET_VAR => {
                if nb_args.val() != 2 {
                    return Err(ActionParseError::InvalidNBArgs);
                }
            }
        }

        if bytes.len() < 1 {
            return Err(ActionParseError::InsufficientBytes);
        }
        let b = bytes.split_to(1);
        let r#u8 = u8::from_be_bytes([b[0]]);
        let var_scope =
            ActionVarScope::try_from(r#u8).map_err(|_| ActionParseError::InvalidVarScope)?;

        let var_name: VarintString = bytes
            .try_into()
            .map_err(|_| ActionParseError::InvalidVarName)?;

        let v = match r#type {
            ActionType::SET_VAR => {
                let var_value: TypedData = bytes
                    .try_into()
                    .map_err(|_| ActionParseError::InvalidVarValue)?;

                Action::set_val(var_scope, var_name, var_value)
            }
            ActionType::UNSET_VAR => Action::unset_val(var_scope, var_name),
        };

        Ok(v)
    }
}

impl Action {
    pub fn write_to(&self, buf: &mut BytesMut) {
        match self {
            Action::SET_VAR {
                var_scope,
                var_name,
                var_value,
            } => {
                buf.put_u8(ActionType::SET_VAR.into());
                buf.put_u8(3);
                buf.put_u8(var_scope.to_owned().into());
                var_name.write_to(buf);
                var_value.write_to(buf);
            }
            Action::UNSET_VAR {
                var_scope,
                var_name,
            } => {
                buf.put_u8(ActionType::SET_VAR.into());
                buf.put_u8(3);
                buf.put_u8(var_scope.to_owned().into());
                var_name.write_to(buf);
            }
        }
    }
}
