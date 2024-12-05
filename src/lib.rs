
use anyhow::{bail, Result};
use cli::Command;

pub mod cli;
pub mod config;
pub mod errors;

#[cfg(not(tarpaulin_include))]
pub async fn run(command: Command) -> Result<()> {
    match command {
        Command::TCPEcho(args) => tcp_echo::run_start(args.ip, args.port).await?,
        Command::ListenTUN(args) => run_start().await?,
    }
    Ok(())
}

async fn run_start() -> Result<()> {
    Ok(())
}

}
