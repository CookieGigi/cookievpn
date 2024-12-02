//! Cli Arguments Parsing

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
    Start(StartArgs),
}

#[derive(Debug, Args)]
pub struct StartArgs {
    #[arg(long)]
    pub ip: String,
    #[arg(long)]
    pub port: u32,
}
