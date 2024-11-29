use anyhow::Result;

pub mod cli;
pub mod config;
pub mod errors;

#[cfg(not(tarpaulin_include))]
pub fn run() -> Result<()> {
    Ok(())
}
