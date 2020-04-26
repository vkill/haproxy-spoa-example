use crate::{
    Varint, VarintBinary, VarintBinaryParseError, VarintParseError, VarintString,
    VarintStringParseError,
};
use bytes::Bytes;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::net::{Ipv4Addr, Ipv6Addr};
use thiserror::Error;

#[derive(PartialEq, Debug)]
pub enum TypedData {
    NULL,
    BOOL(bool),
    INT32(i32),
    UINT32(u32),
    INT64(i64),
    UNIT64(u64),
    IPV4(Ipv4Addr),
    IPV6(Ipv6Addr),
    STRING(VarintString),
    BINARY(VarintBinary),
}

#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Debug)]
#[repr(u8)]
enum TypedDataType {
    NULL = 0,
    BOOL = 1,
    INT32 = 2,
    UINT32 = 3,
    INT64 = 4,
    UNIT64 = 5,
    IPV4 = 6,
    IPV6 = 7,
    STRING = 8,
    BINARY = 9,
}

#[derive(Error, PartialEq, Debug)]
pub enum TypedDataParseError {
    #[error("Insufficient bytes")]
    InsufficientBytes,
    #[error("Invalid type")]
    InvalidType,
    #[error("Invalid")]
    Invalid,
}

impl From<VarintParseError> for TypedDataParseError {
    fn from(e: VarintParseError) -> Self {
        match e {
            VarintParseError::InsufficientBytes => Self::InsufficientBytes,
        }
    }
}

impl From<VarintStringParseError> for TypedDataParseError {
    fn from(e: VarintStringParseError) -> Self {
        match e {
            VarintStringParseError::InsufficientBytes => Self::InsufficientBytes,
            VarintStringParseError::Invalid => Self::Invalid,
        }
    }
}

impl From<VarintBinaryParseError> for TypedDataParseError {
    fn from(e: VarintBinaryParseError) -> Self {
        match e {
            VarintBinaryParseError::InsufficientBytes => Self::InsufficientBytes,
        }
    }
}

impl TryFrom<&mut Bytes> for TypedData {
    type Error = TypedDataParseError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, TypedDataParseError> {
        if bytes.len() < 1 {
            return Err(TypedDataParseError::InsufficientBytes);
        }
        let b = bytes.split_to(1);
        let (r#type, ok) = b[0].overflowing_shr(4);
        if ok {
            return Err(TypedDataParseError::InvalidType);
        }
        let r#type =
            TypedDataType::try_from(r#type).map_err(|_| TypedDataParseError::InvalidType)?;

        let v = match r#type {
            TypedDataType::NULL => Self::NULL,
            TypedDataType::BOOL => Self::BOOL(b[0] & 0b_00001000_u8 != 0),
            TypedDataType::INT32 => {
                let varint = Varint::try_from(bytes).map_err(|e| TypedDataParseError::from(e))?;
                let val = varint.i32_val().ok_or(TypedDataParseError::Invalid)?;
                Self::INT32(val)
            }
            TypedDataType::UINT32 => {
                let varint = Varint::try_from(bytes).map_err(|e| TypedDataParseError::from(e))?;
                let val = varint.u32_val().ok_or(TypedDataParseError::Invalid)?;
                Self::UINT32(val)
            }
            TypedDataType::INT64 => {
                let varint = Varint::try_from(bytes).map_err(|e| TypedDataParseError::from(e))?;
                Self::INT64(varint.i64_val())
            }
            TypedDataType::UNIT64 => {
                let varint = Varint::try_from(bytes).map_err(|e| TypedDataParseError::from(e))?;
                Self::UNIT64(varint.u64_val())
            }
            TypedDataType::IPV4 => {
                if bytes.len() < 4 {
                    return Err(TypedDataParseError::InsufficientBytes);
                }
                let b = bytes.split_to(4);
                let val = Ipv4Addr::new(b[0], b[1], b[2], b[3]);
                Self::IPV4(val)
            }
            TypedDataType::IPV6 => {
                if bytes.len() < 16 {
                    return Err(TypedDataParseError::InsufficientBytes);
                }
                let b = bytes.split_to(16);
                let val = Ipv6Addr::from([
                    b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11],
                    b[12], b[13], b[14], b[15],
                ]);
                Self::IPV6(val)
            }
            TypedDataType::STRING => {
                let varint_string =
                    VarintString::try_from(bytes).map_err(|e| TypedDataParseError::from(e))?;
                Self::STRING(varint_string)
            }
            TypedDataType::BINARY => {
                let varint_binary =
                    VarintBinary::try_from(bytes).map_err(|e| TypedDataParseError::from(e))?;
                Self::BINARY(varint_binary)
            }
        };

        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_from() -> anyhow::Result<()> {
        let mut bytes = Bytes::from_static(&[0b_0000_0000_u8]);
        let bytes = &mut bytes;
        let typed_data: TypedData = bytes.try_into()?;
        assert_eq!(typed_data, TypedData::NULL);

        let mut bytes = Bytes::from_static(&[0b_0001_0000_u8]);
        let bytes = &mut bytes;
        let typed_data: TypedData = bytes.try_into()?;
        assert_eq!(typed_data, TypedData::BOOL(false));

        let mut bytes = Bytes::from_static(&[0b_0001_1000_u8]);
        let bytes = &mut bytes;
        let typed_data: TypedData = bytes.try_into()?;
        assert_eq!(typed_data, TypedData::BOOL(true));

        let mut bytes = Bytes::from_static(&[0b_0010_0000_u8, 0x01]);
        let bytes = &mut bytes;
        let typed_data: TypedData = bytes.try_into()?;
        assert_eq!(typed_data, TypedData::INT32(1));

        let mut bytes = Bytes::from_static(&[0b_0011_0000_u8, 0x01]);
        let bytes = &mut bytes;
        let typed_data: TypedData = bytes.try_into()?;
        assert_eq!(typed_data, TypedData::UINT32(1));

        let mut bytes = Bytes::from_static(&[0b_0100_0000_u8, 0x01]);
        let bytes = &mut bytes;
        let typed_data: TypedData = bytes.try_into()?;
        assert_eq!(typed_data, TypedData::INT64(1));

        let mut bytes = Bytes::from_static(&[0b_0101_0000_u8, 0x01]);
        let bytes = &mut bytes;
        let typed_data: TypedData = bytes.try_into()?;
        assert_eq!(typed_data, TypedData::UNIT64(1));

        let mut bytes = Bytes::from_static(&[0b_0110_0000_u8, 0x01, 0x01, 0x01, 0x01]);
        let bytes = &mut bytes;
        let typed_data: TypedData = bytes.try_into()?;
        assert_eq!(typed_data, TypedData::IPV4(Ipv4Addr::new(1, 1, 1, 1)));

        let mut bytes = Bytes::from_static(&[
            0b_0111_0000_u8,
            0x00,
            0x01,
            0x00,
            0x01,
            0x00,
            0x01,
            0x00,
            0x01,
            0x00,
            0x01,
            0x00,
            0x01,
            0x00,
            0x01,
            0x00,
            0x01,
        ]);
        let bytes = &mut bytes;
        let typed_data: TypedData = bytes.try_into()?;
        assert_eq!(
            typed_data,
            TypedData::IPV6(Ipv6Addr::new(1, 1, 1, 1, 1, 1, 1, 1))
        );

        let mut bytes = Bytes::from_static(&[0b_1000_0000_u8, 0x01, 'a' as u8]);
        let bytes = &mut bytes;
        let typed_data: TypedData = bytes.try_into()?;
        assert_eq!(typed_data, TypedData::STRING(VarintString::new("a")));

        let mut bytes = Bytes::from_static(&[0b_1001_0000_u8, 0x01, 'a' as u8]);
        let bytes = &mut bytes;
        let typed_data: TypedData = bytes.try_into()?;
        assert_eq!(
            typed_data,
            TypedData::BINARY(VarintBinary::new(&vec!['a' as u8]))
        );

        Ok(())
    }
}
