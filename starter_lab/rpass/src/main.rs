#![allow(unused)]

use clap::Parser;
use cli::Cli;
use console_utils::ConsoleIO;

pub mod cli;
pub mod console_utils;
pub mod constants;
pub mod crypto;
pub mod data_store;
pub mod middleware;
pub mod passwords;
pub mod utils;

#[allow(unreachable_code)]
fn main() {
    let cli = Cli::parse();

    todo!();

    if let Err(error) = middleware::handle(&cli) {
        let console = ConsoleIO::new();
        console.error(&format!("{error}"));
    }
}
