//! Cli Arguments Parsing

use std::net::IpAddr;

use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

/// Cli arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(flatten)]
    pub verbose: Verbosity,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    TCPEcho(TCPEchoArgs),
}

#[derive(Debug, Args)]
pub struct TCPEchoArgs {
    #[arg(long, default_value("0.0.0.0"))]
    pub ip: IpAddr,
    #[arg(long)]
    pub port: u16,
}
