use bytes::{BufMut, Bytes, BytesMut};
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct FrameFlags(u32);
impl FrameFlags {
    pub fn is_fin(&self) -> bool {
        self.0 & 0x00000001u32 != 0
    }
    pub fn is_abort(&self) -> bool {
        self.0 & 0x00000002u32 != 0
    }
}
impl FrameFlags {
    pub fn new(is_fin: bool, is_abort: bool) -> Self {
        let mut val = 0_u32;

        if is_fin {
            val |= 0x00000001u32;
        } else {
            val &= !0x00000001u32;
        }

        if is_abort {
            val |= 0x00000002u32;
        } else {
            val &= !0x00000002u32;
        }

        Self(val)
    }

    pub fn val(&self) -> u32 {
        self.0
    }
}

#[derive(Error, PartialEq, Debug)]
pub enum FrameFlagsParseError {
    #[error("Insufficient bytes")]
    InsufficientBytes,

    #[error("When it is set, the FIN bit must also be set.")]
    // https://github.com/haproxy/haproxy/blob/v2.1.0/doc/SPOE.txt#L714
    FINNotSet,
}

impl TryFrom<&mut Bytes> for FrameFlags {
    type Error = FrameFlagsParseError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, FrameFlagsParseError> {
        if bytes.len() < 4 {
            return Err(FrameFlagsParseError::InsufficientBytes);
        }
        let b = bytes.split_to(4);
        let r#u32 = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
        let flags = FrameFlags(r#u32);

        if flags.is_abort() {
            if flags.is_fin() == false {
                return Err(FrameFlagsParseError::FINNotSet);
            }
        }

        Ok(flags)
    }
}

impl FrameFlags {
    pub fn write_to(&self, buf: &mut BytesMut) {
        buf.put_u32(self.0);
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_new() -> anyhow::Result<()> {
        let val = FrameFlags::new(false, false).val();
        assert_eq!(val, 0);

        let val = FrameFlags::new(true, false).val();
        assert_eq!(val, 0x00000001u32);

        let val = FrameFlags::new(true, true).val();
        assert_eq!(val, 0x00000003u32);

        let val = FrameFlags::new(false, true).val();
        assert_eq!(val, 0x00000002u32);

        Ok(())
    }

    #[test]
    fn test_from() -> anyhow::Result<()> {
        let mut bytes = Bytes::from_static(b"\0\0\0\x00");
        let bytes = &mut bytes;
        let frame_flags: FrameFlags = bytes.try_into()?;
        assert_eq!(frame_flags.is_fin(), false);
        assert_eq!(frame_flags.is_abort(), false);

        let mut bytes = Bytes::from_static(b"\0\0\0\x01");
        let bytes = &mut bytes;
        let frame_flags: FrameFlags = bytes.try_into()?;
        assert_eq!(frame_flags.is_fin(), true);
        assert_eq!(frame_flags.is_abort(), false);

        let mut bytes = Bytes::from_static(b"\0\0\0\x02");
        let bytes = &mut bytes;
        if let Err(e) = FrameFlags::try_from(bytes) {
            assert_eq!(e, FrameFlagsParseError::FINNotSet);
        } else {
            assert!(false, "should err");
        }

        let mut bytes = Bytes::from_static(b"\0\0\0\x03");
        let bytes = &mut bytes;
        let frame_flags: FrameFlags = bytes.try_into()?;
        assert_eq!(frame_flags.is_fin(), true);
        assert_eq!(frame_flags.is_abort(), true);

        Ok(())
    }
}
