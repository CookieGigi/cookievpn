use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

use anyhow::Result;
use cli::Command;

pub mod cli;
pub mod config;
pub mod errors;

#[cfg(not(tarpaulin_include))]
pub fn run(command: Command) -> Result<()> {
    match command {
        Command::Start => run_start(),
    }
    Ok(())
}

pub fn run_start() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut data = [0_u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            println!("Data send back : {size}");
            let s = match std::str::from_utf8(&data) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            println!("{s}");
            Write::write(&mut stream, &data[0..size]).unwrap();
            size != 0
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
