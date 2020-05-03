use futures::{SinkExt, TryStreamExt};
use smol::{Async, Task, Timer};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use futures_codec::Framed;
use log::*;

extern crate strum;
#[macro_use]
extern crate strum_macros;

#[macro_use]
mod macros;

mod varint;
pub use varint::{Varint, VarintParseError};
mod varint_binary;
pub use varint_binary::{VarintBinary, VarintBinaryParseError};
mod varint_string;
pub use varint_string::{VarintString, VarintStringParseError};
mod typed_data;
pub use typed_data::{TypedData, TypedDataParseError};
mod nb_args;
pub use nb_args::{NBArgs, NBArgsParseError};
mod action;
pub use action::{Action, ActionParseError, ActionType, ActionVarScope};
mod support_version;
pub use support_version::SupportVersion;

mod frame_codec;
pub use frame_codec::FrameCodec;
mod frame_type;
pub use frame_type::{FrameType, FrameTypeParseError};
mod frame_flags;
pub use frame_flags::{FrameFlags, FrameFlagsParseError};
mod frame_payload;
pub use frame_payload::{FramePayload, FramePayloadParseError, FramePayloadType};
mod frame_storage;
pub use frame_storage::{FrameStorage, FrameStorageParseError};
mod frame;
pub use frame::Frame;
mod frames;
pub use frames::*;

mod frame_error;
pub use frame_error::FrameKnownError;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    smol::run(accept_loop("127.0.0.1:6001"))
}

async fn accept_loop(addr: &str) -> anyhow::Result<()> {
    let listener = Async::<TcpListener>::bind(addr)?;

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        info!("Accepted client: {}", peer_addr);

        // Spawn a task that echoes messages from the client back to it.
        Task::spawn(async move {
            if let Err(e) = connection_loop(stream).await {
                error!("connection error: {:?}", e)
            } else {
                info!("connection closed")
            }
        })
        .detach();
    }
}

async fn connection_loop(stream: Async<TcpStream>) -> anyhow::Result<()> {
    let mut framed = Framed::new(stream, FrameCodec());

    let frame = Frame::new();

    while let Some(mut bytes) = framed.try_next().await? {
        debug!("read len: {} bytes: {:?}", bytes.len(), bytes);
        let bytes = &mut bytes;

        let (bytes, do_close) = frame.handle(bytes)?;

        if let Some(bytes) = bytes {
            info!("write len: {}, bytes: {:?}", bytes.len(), bytes);

            Timer::after(Duration::from_nanos(100)).await;

            framed.send(bytes.freeze()).await.map_err(|e| {
                error!("on send {:?}", e);
                e
            })?;
        }

        if do_close {
            info!("do close");
            framed.flush().await.map_err(|e| {
                error!("on flush {:?}", e);
                e
            })?;
            framed.close().await.map_err(|e| {
                error!("on close {:?}", e);
                e
            })?;
        }
    }

    Ok(())
}
