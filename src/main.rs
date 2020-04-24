use async_std::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use log::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    env_logger::init();

    task::block_on(accept_loop("127.0.0.1:8080"))
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

async fn connection_loop(stream: TcpStream) -> Result<()> {
    unimplemented!();

    Ok(())
}
