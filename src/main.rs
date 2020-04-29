use futures::TryStreamExt;
use smol::{Async, Task};
use std::net::{TcpListener, TcpStream};

use futures_codec::Framed;
use log::*;
use std::convert::TryInto;

mod varint;
pub use varint::{Varint, VarintParseError};
mod varint_binary;
pub use varint_binary::{VarintBinary, VarintBinaryParseError};
mod varint_string;
pub use varint_string::{VarintString, VarintStringParseError};
mod typed_data;
pub use typed_data::{TypedData, TypedDataParseError};

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
pub use frame::{Frame, FrameNewError};
mod frames;
pub use frames::*;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    smol::run(accept_loop("127.0.0.1:6001"))
}

async fn accept_loop(addr: &str) -> anyhow::Result<()> {
    let listener = Async::<TcpListener>::bind(addr)?;

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        println!("Accepted client: {}", peer_addr);

        // Spawn a task that echoes messages from the client back to it.
        Task::spawn(async move {
            if let Err(e) = connection_loop(stream).await {
                error!("connection error: {:?}", e)
            }
        })
        .detach();
    }
}

async fn connection_loop(stream: Async<TcpStream>) -> anyhow::Result<()> {
    let mut framed = Framed::new(stream, FrameCodec());
    while let Some(mut bytes) = framed.try_next().await? {
        debug!("frame bytes: {:?}", bytes);

        let bytes = &mut bytes;
        let frame_storage: FrameStorage = bytes.try_into()?;

        info!("frame_storage: {:?}", frame_storage);

        unimplemented!();
    }

    Ok(())
}
