use clap::{Parser, Subcommand};

/// rpass - A rust implementation of 'nix pass.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {}

/// Possible sub-commands to run.
#[derive(Subcommand, Clone, PartialEq)]
pub enum Command {}
