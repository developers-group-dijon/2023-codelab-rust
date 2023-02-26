use std::process::exit;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
pub mod cli;
pub mod console_utils;
pub mod constants;
pub mod crypto;
pub mod data_store;
pub mod middleware;
pub mod utils;

fn main() -> Result<()> {
    middleware::handle(&Cli::parse())
}
