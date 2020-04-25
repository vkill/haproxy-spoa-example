use crate::{Frame, FrameNewError, FrameStorage};
use thiserror::Error;

pub struct HAProxyHelloFrame {}

#[derive(Error, Debug)]
pub enum HAProxyHelloFrameNewError {}

impl Frame for HAProxyHelloFrame {
    fn new(storage: FrameStorage) -> Result<Self, FrameNewError> {
        Ok(Self {})
    }
}
