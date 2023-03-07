use clap::{Parser, Subcommand};

/// A representation of the CLI command with its own options and args.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// master password to unlock the DataStore
    #[arg(short, long)]
    pub master_password: Option<String>,

    /// sub-command to actually run a part of the program.
    #[command(subcommand)]
    pub command: Command,
}

/// Possible sub-commands to run.
#[derive(Subcommand, Clone, PartialEq)]
pub enum Command {
    /// List all the password stored in the DataStore
    List,
    /// Initializes a new DataStore
    Init,
    /// Adds a new password to the DataStore
    Add,
    /// Delete a given password from the DataStore
    Delete {
        /// name of the password to delete
        name: String,
    },
    /// Dumps a given password into standard output
    Dump {
        /// name of the password to dump
        name: String,
    },
    /// Generates a new strong password and stores it
    Generate,
}
