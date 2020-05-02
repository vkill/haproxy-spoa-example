use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::EnumString;

#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, EnumString, Display, Clone, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FrameKnownError {
    #[strum(serialize = "normal (no error occurred)")]
    normal = 0,
    #[strum(serialize = "I/O error")]
    io_error = 1,
    #[strum(serialize = "A timeout occurred")]
    timeout = 2,
    #[strum(serialize = "frame is too big")]
    frame_is_too_big = 3,
    #[strum(serialize = "invalid frame received")]
    invalid_frame_received = 4,
    #[strum(serialize = "version value not found")]
    version_value_not_found = 5,
    #[strum(serialize = "max-frame-size value not found")]
    max_frame_size_value_not_found = 6,
    #[strum(serialize = "capabilities_value_not_found")]
    capabilities_value_not_found = 7,
    #[strum(serialize = "unsupported version")]
    unsupported_version = 8,
    #[strum(serialize = "max-frame-size too big or too small")]
    max_frame_size_too_big_or_too_small = 9,
    #[strum(serialize = "payload fragmentation is not supported")]
    payload_fragmentation_is_not_supported = 10,
    #[strum(serialize = "invalid interlaced frames")]
    invalid_interlaced_frames = 11,
    #[strum(serialize = "frame-id not found (it does not match any referenced frame)")]
    frame_id_not_found = 12,
    #[strum(serialize = "resource allocation error")]
    resource_allocation_error = 13,
    #[strum(serialize = "an unknown error occurrde")]
    unknown = 99,
}
