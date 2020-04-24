use async_std::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use bytes::BytesMut;
use log::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    env_logger::init();

    task::block_on(accept_loop("127.0.0.1:6001"))
}

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        info!("Accepting from: {}", stream.peer_addr()?);

        task::spawn(async move {
            if let Err(e) = connection_loop(stream).await {
                error!("{:?}", e)
            }
        });
    }

    Ok(())
}

const U32_LENGTH: usize = std::mem::size_of::<u32>();

async fn connection_loop(stream: TcpStream) -> Result<()> {
    let (reader, mut writer) = (&stream, &stream);

    let mut length_buf = [0; U32_LENGTH];
    let mut length_take = reader.take(U32_LENGTH as u64);
    length_take.read(&mut length_buf).await?;
    let length = u32::from_be_bytes(length_buf);

    let mut frame_buf = Vec::<u8>::with_capacity(length as usize);
    let mut frame_take = reader.take(length as u64);
    frame_take.read(&mut frame_buf).await?;

    let buf: BytesMut = frame_buf[..].into();

    unimplemented!();

    Ok(())
}
