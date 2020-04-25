use bytes::Bytes;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug)]
pub struct FrameFlags(u32);
impl FrameFlags {
    pub fn is_fin(&self) -> bool {
        self.0 & 0x80000000u32 != 0
    }
    pub fn is_abort(&self) -> bool {
        self.0 & 0x40000000u32 != 0
    }
}

#[derive(Error, PartialEq, Debug)]
pub enum FrameFlagsFromError {
    #[error("Insufficient bytes")]
    InsufficientBytes,

    #[error("When it is set, the FIN bit must also be set.")]
    // https://github.com/haproxy/haproxy/blob/v2.1.0/doc/SPOE.txt#L714
    FINNotSet,
}

impl TryFrom<&mut Bytes> for FrameFlags {
    type Error = FrameFlagsFromError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, FrameFlagsFromError> {
        if bytes.len() < 4 {
            return Err(FrameFlagsFromError::InsufficientBytes);
        }
        let bytes = bytes.split_to(4);
        let r#u32 = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let flags = FrameFlags(r#u32.reverse_bits());

        if flags.is_abort() {
            if flags.is_fin() == false {
                return Err(FrameFlagsFromError::FINNotSet);
            }
        }

        Ok(flags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_from() -> anyhow::Result<()> {
        let mut bytes = Bytes::from_static(b"\0\0\0\x01");
        let bytes = &mut bytes;
        let frame_flags: FrameFlags = bytes.try_into()?;
        assert_eq!(frame_flags.is_fin(), true);
        assert_eq!(frame_flags.is_abort(), false);

        let mut bytes = Bytes::from_static(b"\0\0\0\x02");
        let bytes = &mut bytes;
        if let Err(e) = FrameFlags::try_from(bytes) {
            assert_eq!(e, FrameFlagsFromError::FINNotSet);
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
