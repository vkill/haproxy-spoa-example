use crate::{FrameStorage, SupportVersion};
use std::convert::TryFrom;
use std::str::FromStr;
use strum_macros::EnumString;
use thiserror::Error;

#[derive(Debug)]
pub struct HAProxyHelloFrame {
    pub payload: HAProxyHelloFramePayload,
}

make_frame_kv_list_payload! {
#[derive(Clone, Debug)]
pub struct HAProxyHelloFramePayload {
    pub supported_versions: Vec<SupportVersion>,
    pub max_frame_size: u32,
    pub capabilities: Vec<HAProxyHelloFrameCapability>,
    pub healthcheck: Option<bool>,
    pub engine_id: String,
}
}

#[derive(EnumString, Clone, Debug, Display)]
#[allow(non_camel_case_types)]
pub enum HAProxyHelloFrameCapability {
    #[strum(serialize = "pipelining")]
    pipelining,
    #[strum(serialize = "async")]
    r#async,
}

#[derive(Error, Debug)]
#[allow(non_camel_case_types)]
pub enum HAProxyHelloFrameParseError {
    #[error("STREAM-ID and FRAME-ID are must be set 0")]
    Invalid_STREAM_ID,
    #[error("STREAM-ID and FRAME-ID are must be set 0")]
    Invalid_FRAME_ID,
    #[error("field {0} not found")]
    FieldNotFound(String),
    #[error("field {0} value invalid")]
    FieldValueInvalid(String),
}

impl TryFrom<FrameStorage> for HAProxyHelloFrame {
    type Error = HAProxyHelloFrameParseError;
    fn try_from(storage: FrameStorage) -> Result<Self, HAProxyHelloFrameParseError> {
        if storage.stream_id.u64_val() != 0 {
            return Err(HAProxyHelloFrameParseError::Invalid_STREAM_ID);
        }
        if storage.frame_id.u64_val() != 0 {
            return Err(HAProxyHelloFrameParseError::Invalid_FRAME_ID);
        }

        let supported_versions_name = &HAProxyHelloFramePayload::supported_versions_name();
        let supported_versions_value: Vec<Option<SupportVersion>> = storage
            .payload
            .get_kv_list_value(supported_versions_name)
            .ok_or(HAProxyHelloFrameParseError::FieldNotFound(
                supported_versions_name.to_owned(),
            ))?
            .get_string()
            .ok_or(HAProxyHelloFrameParseError::FieldValueInvalid(
                supported_versions_name.to_owned(),
            ))?
            .val()
            .split(",")
            .map(|x| SupportVersion::parse(x))
            .collect();

        let mut supported_versions: Vec<SupportVersion> = vec![];
        for v in supported_versions_value {
            let v = v.ok_or(HAProxyHelloFrameParseError::FieldValueInvalid(
                supported_versions_name.to_owned(),
            ))?;
            supported_versions.push(v);
        }
        if supported_versions.is_empty() {
            return Err(HAProxyHelloFrameParseError::FieldValueInvalid(
                supported_versions_name.to_owned(),
            ));
        }

        let max_frame_size_name = &HAProxyHelloFramePayload::max_frame_size_name();
        let max_frame_size = storage
            .payload
            .get_kv_list_value(max_frame_size_name)
            .ok_or(HAProxyHelloFrameParseError::FieldNotFound(
                max_frame_size_name.to_owned(),
            ))?
            .get_u32()
            .ok_or(HAProxyHelloFrameParseError::FieldValueInvalid(
                max_frame_size_name.to_owned(),
            ))?;

        let capabilities_name = &HAProxyHelloFramePayload::capabilities_name();
        let capabilities_value: Vec<Option<HAProxyHelloFrameCapability>> = storage
            .payload
            .get_kv_list_value(capabilities_name)
            .ok_or(HAProxyHelloFrameParseError::FieldNotFound(
                capabilities_name.to_owned(),
            ))?
            .get_string()
            .ok_or(HAProxyHelloFrameParseError::FieldValueInvalid(
                capabilities_name.to_owned(),
            ))?
            .val()
            .split(",")
            .map(|x| HAProxyHelloFrameCapability::from_str(x.trim()).ok())
            .collect();

        let mut capabilities: Vec<HAProxyHelloFrameCapability> = vec![];
        for v in capabilities_value {
            let v = v.ok_or(HAProxyHelloFrameParseError::FieldValueInvalid(
                capabilities_name.to_owned(),
            ))?;
            capabilities.push(v);
        }

        let mut healthcheck: Option<&bool> = None;
        let healthcheck_name = &HAProxyHelloFramePayload::healthcheck_name();
        if let Some(healthcheck_value) = storage.payload.get_kv_list_value(healthcheck_name) {
            let healthcheck_value = healthcheck_value.get_bool().ok_or(
                HAProxyHelloFrameParseError::FieldValueInvalid(healthcheck_name.to_owned()),
            )?;
            healthcheck = Some(healthcheck_value);
        }

        let engine_id_name = &HAProxyHelloFramePayload::engine_id_name();
        let engine_id = storage
            .payload
            .get_kv_list_value(engine_id_name)
            .ok_or(HAProxyHelloFrameParseError::FieldNotFound(
                engine_id_name.to_owned(),
            ))?
            .get_string()
            .ok_or(HAProxyHelloFrameParseError::FieldValueInvalid(
                engine_id_name.to_owned(),
            ))?
            .val();

        let payload = HAProxyHelloFramePayload {
            supported_versions: supported_versions,
            max_frame_size: max_frame_size.to_owned(),
            capabilities: capabilities,
            healthcheck: healthcheck.map(|x| x.to_owned()),
            engine_id: engine_id.to_owned(),
        };

        let frame = Self { payload };
        Ok(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FrameType;
    use bytes::Bytes;
    use semver::Version;
    use std::convert::TryInto;

    /*
    b"\x01\0\0\0\x01\0\0\x12supported-versions\x08\x032.0\x0emax-frame-size\x03\xfc\xf0\x06\x0ccapabilities\x08\x10pipelining,async\tengine-id\x08$6bdec4ec-6b9a-4705-83f4-8817766c0c57"
    */

    #[test]
    fn test_from() -> anyhow::Result<()> {
        let bytes = b"\x01\0\0\0\x01\0\0\x12supported-versions\x08\x032.0\x0emax-frame-size\x03\xfc\xf0\x06\x0ccapabilities\x08\x10pipelining,async\tengine-id\x08$6bdec4ec-6b9a-4705-83f4-8817766c0c57";
        let mut bytes = Bytes::from_static(bytes);
        let bytes = &mut bytes;

        let frame_storage: FrameStorage = bytes.try_into()?;
        println!("{:?}", frame_storage);

        assert_eq!(frame_storage.r#type, FrameType::HAPROXY_HELLO);
        assert_eq!(frame_storage.flags.is_fin(), true);
        assert_eq!(frame_storage.flags.is_abort(), false);
        assert_eq!(frame_storage.stream_id.u64_val(), 0);
        assert_eq!(frame_storage.frame_id.u64_val(), 0);

        let frame = HAProxyHelloFrame::try_from(frame_storage)?;
        println!("{:?}", frame);

        assert_eq!(
            frame.payload.supported_versions,
            vec![SupportVersion::new(Version::new(2, 0, 0))]
        );

        Ok(())
    }
}
