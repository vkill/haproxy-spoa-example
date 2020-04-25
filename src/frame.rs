use crate::{FrameStorage, HAProxyHelloFrameNewError};
use thiserror::Error;

pub trait Frame
where
    Self: std::marker::Sized,
{
    fn new(storage: FrameStorage) -> Result<Self, FrameNewError>;
}

#[derive(Error, Debug)]
pub enum FrameNewError {
    #[error("invalid HAProxyHello frame")]
    InvalidHAProxyHelloFrame(#[from] HAProxyHelloFrameNewError),
}
