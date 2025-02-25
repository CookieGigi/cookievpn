use core::str;
use std::net::IpAddr;

use anyhow::Result;
use cli::Command;
use etherparse::{NetSlice, SlicedPacket};
use tun::AsyncDevice;

pub mod cli;
pub mod config;
pub mod errors;
pub mod tcp_echo;

#[cfg(not(tarpaulin_include))]
pub async fn run(command: Command) -> Result<()> {
    match command {
        Command::TCPEcho(args) => tcp_echo::run_start(args.ip, args.port).await?,
        Command::ListenTUN(args) => {
            run_start(&args.name, args.src, args.dest, args.netmask).await?
        }
    }
    Ok(())
}

async fn run_start(name: &str, src: IpAddr, dest: IpAddr, netmask: IpAddr) -> Result<()> {
    let device = add_tun_device(name, src, dest, netmask)?;
    listen_tun_interface(device).await?;
    let mut stdin = std::io::stdin();
    let _ = std::io::Read::read(&mut stdin, &mut [0u8])?;
    Ok(())
}

fn add_tun_device(
    name: &str,
    source: IpAddr,
    dest: IpAddr,
    netmask: IpAddr,
) -> Result<AsyncDevice> {
    let mut config = tun::Configuration::default();
    config
        .tun_name(name)
        .address(source)
        .netmask(netmask)
        .destination(dest)
        .mtu(tun::DEFAULT_MTU)
        .up();

    #[cfg(target_os = "linux")]
    config.platform_config(|config| {
        config.ensure_root_privileges(true);
    });

    let tun_device = tun::create_as_async(&config)?;
    Ok(tun_device)
}

async fn listen_tun_interface(dev: AsyncDevice) -> Result<()> {
    let size = tun::DEFAULT_MTU as usize + tun::PACKET_INFORMATION_LENGTH;
    let mut buf: Vec<u8> = vec![0; size];
    let mut len: usize;
    loop {
        len = dev.recv(&mut buf).await?;
        println!("pkt {len} bytes : {:?}", &buf[0..len]);
        match SlicedPacket::from_ip(&buf) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => match value.net.unwrap() {
                NetSlice::Ipv4(value) => {
                    let header = value.header();
                    println!(
                        "{} -> {} : ",
                        header.source_addr(),
                        header.destination_addr()
                    );
                    println!("payload: {:?}", value.payload());
                }
                NetSlice::Ipv6(value) => {
                    let header = value.header();
                    println!(
                        "{} -> {} : ",
                        header.source_addr(),
                        header.destination_addr()
                    );
                    println!("payload: {:?}", value.payload());
                }
            },
        }
    }
}
