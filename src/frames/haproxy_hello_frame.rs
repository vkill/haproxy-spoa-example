use crate::{Frame, FrameNewError, FrameStorage};
use thiserror::Error;

pub struct HAProxyHelloFrame {
    pub supported_versions: String,
    pub max_frame_size: u32,
    pub capabilities: String,
    pub healthcheck: Option<bool>,
    pub engine_id: String,
}

#[derive(Error, Debug)]
#[allow(non_camel_case_types)]
pub enum HAProxyHelloFrameNewError {
    #[error("STREAM-ID and FRAME-ID are must be set 0")]
    Invalid_STREAM_ID,
    #[error("STREAM-ID and FRAME-ID are must be set 0")]
    Invalid_FRAME_ID,
    #[error("field {0} not found")]
    FieldNotFound(String),
    #[error("field {0} value invalid")]
    FieldValueInvalid(String),
}

impl Frame for HAProxyHelloFrame {
    fn new(storage: FrameStorage) -> Result<Self, FrameNewError> {
        if storage.stream_id.val().is_empty() {
            return Err(HAProxyHelloFrameNewError::Invalid_STREAM_ID.into());
        }
        if storage.frame_id.val().is_empty() {
            return Err(HAProxyHelloFrameNewError::Invalid_FRAME_ID.into());
        }

        let supported_versions_name = "supported-versions";
        let supported_versions = storage
            .payload
            .get_kv_value(supported_versions_name)
            .ok_or(HAProxyHelloFrameNewError::FieldNotFound(
                supported_versions_name.to_owned(),
            ))?
            .get_string()
            .ok_or(HAProxyHelloFrameNewError::FieldValueInvalid(
                supported_versions_name.to_owned(),
            ))?
            .val();

        let max_frame_size_name = "max-frame-size";
        let max_frame_size = storage
            .payload
            .get_kv_value(max_frame_size_name)
            .ok_or(HAProxyHelloFrameNewError::FieldNotFound(
                max_frame_size_name.to_owned(),
            ))?
            .get_u32()
            .ok_or(HAProxyHelloFrameNewError::FieldValueInvalid(
                supported_versions_name.to_owned(),
            ))?;

        let capabilities_name = "capabilities";
        let capabilities = storage
            .payload
            .get_kv_value(capabilities_name)
            .ok_or(HAProxyHelloFrameNewError::FieldNotFound(
                capabilities_name.to_owned(),
            ))?
            .get_string()
            .ok_or(HAProxyHelloFrameNewError::FieldValueInvalid(
                capabilities_name.to_owned(),
            ))?
            .val();

        let mut healthcheck: Option<&bool> = None;
        let healthcheck_name = "healthcheck";
        if let Some(healthcheck_value) = storage.payload.get_kv_value(healthcheck_name) {
            let healthcheck_value = healthcheck_value.get_bool().ok_or(
                HAProxyHelloFrameNewError::FieldValueInvalid(healthcheck_name.to_owned()),
            )?;
            healthcheck = Some(healthcheck_value);
        }

        let engine_id_name = "engine-id";
        let engine_id = storage
            .payload
            .get_kv_value(engine_id_name)
            .ok_or(HAProxyHelloFrameNewError::FieldNotFound(
                engine_id_name.to_owned(),
            ))?
            .get_string()
            .ok_or(HAProxyHelloFrameNewError::FieldValueInvalid(
                engine_id_name.to_owned(),
            ))?
            .val();

        let frame = Self {
            supported_versions: supported_versions.to_owned(),
            max_frame_size: max_frame_size.to_owned(),
            capabilities: capabilities.to_owned(),
            healthcheck: healthcheck.map(|x| x.to_owned()),
            engine_id: engine_id.to_owned(),
        };

        Ok(frame)
    }
}
