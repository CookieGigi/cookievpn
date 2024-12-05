use std::{net::IpAddr, str::FromStr};

use anyhow::{bail, Result};
use cli::Command;
use futures::stream::TryStreamExt;
use netlink_packet_route::link::LinkMessage;
use rtnetlink::{new_connection, Handle};
use tun::{AbstractDevice, Device};

pub mod cli;
pub mod config;
pub mod errors;
pub mod tcp_echo;

#[cfg(not(tarpaulin_include))]
pub async fn run(command: Command) -> Result<()> {
    match command {
        Command::TCPEcho(args) => tcp_echo::run_start(args.ip, args.port).await?,
        Command::ListenTUN(args) => run_start().await?,
    }
    Ok(())
}

async fn run_start() -> Result<()> {
    let name = "tun0";
    let ip = "10.0.0.1";
    let device = add_tun_device(name, IpAddr::from_str(ip)?)?;
    let mut stdin = std::io::stdin();
    let _ = std::io::Read::read(&mut stdin, &mut [0u8])?;
    Ok(())
}

fn add_tun_device(name: &str, ip: IpAddr) -> Result<Device> {
    let mut config = tun::Configuration::default();
    config.tun_name(name);
    let mut tun_device = tun::create(&config)?;
    tun_device.set_address(ip)?;
    tun_device.enabled(true)?;
    Ok(tun_device)
}
