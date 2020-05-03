use futures::{SinkExt, TryStreamExt};
use smol::{Async, Task, Timer};
// use std::net::TcpListener;
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
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
mod frame_header;
pub use frame_header::{FrameHeader, FrameHeaderParseError};
mod frame_payload;
pub use frame_payload::{FramePayload, FramePayloadParseError, FramePayloadType};
mod frame;
pub use frame::Frame;
mod frames;
pub use frames::*;

mod frame_error;
pub use frame_error::FrameKnownError;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    ctrlc::set_handler(|| {
        let sock_path = PathBuf::new()
            .join("haproxy_run/spoa_demo.sock")
            .to_str()
            .unwrap()
            .to_string();

        match std::fs::remove_file(&sock_path) {
            Ok(_) => info!("delete sock done"),
            Err(e) => error!("delete sock error: {}", e),
        }

        std::process::exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    // smol::run(accept_loop("127.0.0.1:6001"))
    smol::run(async move {
        let sock_path = PathBuf::new()
            .join("haproxy_run/spoa_demo.sock")
            .to_str()
            .unwrap()
            .to_string();

        let r = accept_loop(sock_path.as_str()).await;

        match r {
            Ok(_) => info!("accept_loop done"),
            Err(e) => error!("accept_loop error: {}", e),
        }

        std::fs::remove_file(sock_path)?;

        Ok(())
    })
}

async fn accept_loop(addr: &str) -> anyhow::Result<()> {
    // let listener = Async::<TcpListener>::bind(addr)?;
    let listener = Async::<UnixListener>::bind(addr)?;

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        info!("Accepted client: {:?}", peer_addr);

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

async fn connection_loop<S>(stream: Async<S>) -> anyhow::Result<()>
where
    S: std::io::Read + std::io::Write,
{
    let mut framed = Framed::new(stream, FrameCodec());

    let mut frame = Frame::new();

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
