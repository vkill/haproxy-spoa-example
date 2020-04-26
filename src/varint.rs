use bytes::Bytes;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug)]
enum VarintStorage {
    U32(u32),
    U64(u64),
}
#[derive(Debug)]
pub struct Varint(VarintStorage);

impl From<u32> for Varint {
    fn from(v: u32) -> Self {
        Self(VarintStorage::U32(v))
    }
}

impl From<u64> for Varint {
    fn from(v: u64) -> Self {
        Self(VarintStorage::U64(v))
    }
}

impl Varint {
    pub fn u64_val(&self) -> u64 {
        match self.0 {
            VarintStorage::U32(val) => val as u64,
            VarintStorage::U64(val) => val,
        }
    }

    pub fn i64_val(&self) -> i64 {
        self.u64_val() as i64
    }

    pub fn u32_val(&self) -> Option<u32> {
        match self.0 {
            VarintStorage::U32(val) => Some(val),
            VarintStorage::U64(val) => u32::try_from(val).ok(),
        }
    }

    pub fn i32_val(&self) -> Option<i32> {
        self.u32_val().map(|v| v as i32)
    }
}

#[derive(Error, PartialEq, Debug)]
pub enum VarintParseError {
    #[error("Insufficient bytes")]
    InsufficientBytes,
}

impl TryFrom<&mut Bytes> for Varint {
    type Error = VarintParseError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, VarintParseError> {
        let mut val_u64: u64 = 0;
        let mut n: u8 = 0;

        if bytes.len() < 1 {
            return Err(VarintParseError::InsufficientBytes);
        }
        let b = bytes.split_to(1);
        n += 1;
        let _val_u8 = b[0];

        val_u64 = _val_u8 as u64;

        if val_u64 >= 240 {
            let mut _r: u8 = 4;

            loop {
                if bytes.len() < 1 {
                    return Err(VarintParseError::InsufficientBytes);
                }
                let b = bytes.split_to(1);
                n += 1;
                let _val_u8 = b[0];

                val_u64 += (_val_u8 as u64) << _r;

                _r += 7;

                if _val_u8 < 128 {
                    break;
                }
            }
        }

        if val_u64 <= (u32::max_value() as u64) {
            Ok((val_u64 as u32).into())
        } else {
            Ok(val_u64.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_x_val() -> anyhow::Result<()> {
        let varint = Varint(VarintStorage::U32(u32::max_value()));
        assert_eq!(varint.u32_val(), Some(u32::max_value()));
        assert_eq!(varint.i32_val(), Some(u32::max_value() as i32));
        assert_eq!(varint.u64_val(), u32::max_value() as u64);
        assert_eq!(varint.i64_val(), u32::max_value() as i64);

        let varint = Varint(VarintStorage::U64(u64::max_value()));
        assert_eq!(varint.u32_val(), None);
        assert_eq!(varint.i32_val(), None);
        assert_eq!(varint.u64_val(), u64::max_value());
        assert_eq!(varint.i64_val(), u64::max_value() as i64);

        let varint = Varint(VarintStorage::U32(1));
        assert_eq!(varint.u32_val(), Some(1));
        assert_eq!(varint.i32_val(), Some(1));
        assert_eq!(varint.u64_val(), 1);
        assert_eq!(varint.i64_val(), 1);

        let varint = Varint(VarintStorage::U64(1));
        assert_eq!(varint.u32_val(), Some(1));
        assert_eq!(varint.i32_val(), Some(1));
        assert_eq!(varint.u64_val(), 1);
        assert_eq!(varint.i64_val(), 1);

        Ok(())
    }

    #[test]
    fn test_from() -> anyhow::Result<()> {
        let results: Vec<(u64, Vec<u8>)> = vec![
            //
            (0, vec![0]),
            (240, vec![0b_11110000_u8, 0]),
            (2288, vec![0b_11110000_u8, 0b_10000000_u8, 0]),
            (
                264432,
                vec![0b_11110000_u8, 0b_10000000_u8, 0b_10000000_u8, 0],
            ),
            (
                33818864,
                vec![
                    0b_11110000_u8,
                    0b_10000000_u8,
                    0b_10000000_u8,
                    0b_10000000_u8,
                    0,
                ],
            ),
            //
            (240 - 1, vec![0b_11101111_u8]),
            (2288 - 1, vec![0b_11111111_u8, 0b_01111111_u8]),
            (
                264432 - 1,
                vec![0b_11111111_u8, 0b_11111111_u8, 0b_01111111_u8],
            ),
            (
                33818864 - 1,
                vec![
                    0b_11111111_u8,
                    0b_11111111_u8,
                    0b_11111111_u8,
                    0b_01111111_u8,
                ],
            ),
            (
                4328786160 - 1,
                vec![
                    0b_11111111_u8,
                    0b_11111111_u8,
                    0b_11111111_u8,
                    0b_11111111_u8,
                    0b_01111111_u8,
                ],
            ),
            //
            (u8::max_value() as u64, vec![0b_11111111_u8, 0]),
            (
                u16::max_value() as u64,
                vec![0b_11111111_u8, 0b_11110000_u8, 30],
            ),
            (
                u32::max_value() as u64,
                vec![0b_11111111_u8, 0b_11110000_u8, 254, 254, 126],
            ),
            (
                u64::max_value(),
                vec![
                    0b_11111111_u8,
                    0b_11110000_u8,
                    254,
                    254,
                    254,
                    254,
                    254,
                    254,
                    254,
                    14,
                ],
            ),
        ];

        for (val, bytes) in results {
            let mut bytes = Bytes::from(bytes);
            let bytes = &mut bytes;
            let varint: Varint = bytes.try_into()?;
            assert_eq!(varint.u64_val(), val);
        }

        Ok(())
    }
}
