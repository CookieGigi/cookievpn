use std::net::Shutdown;

use anyhow::{bail, Result};
use cli::Command;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub mod cli;
pub mod config;
pub mod errors;

#[cfg(not(tarpaulin_include))]
pub async fn run(command: Command) -> Result<()> {
    match command {
        Command::Start => run_start().await?,
    }
    Ok(())
}

async fn run_start() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        let stream = listener.accept().await?;

        tokio::spawn(async move {
            match handle_connection(stream.0).await {
                Ok(_) => (),
                Err(e) => tracing::error!("{e}"),
            };
        });
    }
    Ok(())
}

#[tracing::instrument]
async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    tracing::info!("Connection established!");
    let mut data = [0_u8; 50];
    while match stream.read(&mut data).await {
        Ok(size) => {
            tracing::debug!("Data send back : {size}");
            AsyncWriteExt::write(&mut stream, &data[0..size]).await?;
            size != 0
        }
        Err(_) => {
            stream.shutdown().await?;
            tracing::error!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            bail!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
        }
    } {}
    tracing::info!("Connection finished !");
    Ok(())
}
