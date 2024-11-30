use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

use anyhow::{bail, Result};
use cli::Command;

pub mod cli;
pub mod config;
pub mod errors;

#[cfg(not(tarpaulin_include))]
pub fn run(command: Command) -> Result<()> {
    match command {
        Command::Start => run_start()?,
    }
    Ok(())
}

pub fn run_start() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        match handle_connection(stream) {
            Ok(_) => continue,
            Err(e) => tracing::error!("{e}"),
        };
    }
    Ok(())
}

#[tracing::instrument]
fn handle_connection(mut stream: TcpStream) -> Result<()> {
    tracing::info!("Connection established!");
    let mut data = [0_u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            tracing::debug!("Data send back : {size}");
            Write::write(&mut stream, &data[0..size]).unwrap();
            size != 0
        }
        Err(_) => {
            stream.shutdown(Shutdown::Both).unwrap();
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
