use async_std::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use futures::TryStreamExt;
use futures_codec::Framed;
use log::*;
use std::convert::TryInto;

mod varint;
pub use varint::{Varint, VarintParseError};
mod varint_binary;
pub use varint_binary::{VarintBinary, VarintBinaryParseError};
mod varint_string;
pub use varint_string::{VarintString, VarintStringParseError};

mod frame_codec;
pub use frame_codec::FrameCodec;
mod frame_type;
pub use frame_type::{FrameType, FrameTypeParseError};
mod frame_flags;
pub use frame_flags::{FrameFlags, FrameFlagsParseError};
mod frame_storage;
pub use frame_storage::{FrameStorage, FrameStorageParseError};
mod frame;
pub use frame::{Frame, FrameNewError};
mod frames;
pub use frames::*;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    task::block_on(accept_loop("127.0.0.1:6001"))
}

async fn accept_loop(addr: impl ToSocketAddrs) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        info!("Accepting from: {}", stream.peer_addr()?);

        task::spawn(async move {
            if let Err(e) = connection_loop(stream).await {
                error!("connection error: {:?}", e)
            }
        });
    }

    Ok(())
}

async fn connection_loop(stream: TcpStream) -> anyhow::Result<()> {
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
