use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// sub-command to actually run a part of the program.
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Clone)]
pub enum Command {
    /// List all the password stored in the DataStore
    List {
        /// master password to unlock the DataStore
        #[arg(short, long)]
        master_password: Option<String>,
    },
    /// Initializes a new DataStore
    Init {
        /// master password to unlock the DataStore
        #[arg(short, long)]
        master_password: Option<String>,
    },
    /// Adds a new password to the DataStore
    Add {
        /// master password to unlock the DataStore
        #[arg(short, long)]
        master_password: Option<String>,
    },
    /// Delete a given password from the DataStore
    Delete {
        /// master password to unlock the DataStore
        #[arg(short, long)]
        master_password: Option<String>,
        /// name of the password to delete
        name: String,
    },
    /// Dumps a given password into standard output
    Dump {
        /// master password to unlock the DataStore
        #[arg(short, long)]
        master_password: Option<String>,
        /// name of the password to dump
        name: String,
    },
    /// Generates a new strong password and stores it
    Generate {
        /// master password to unlock the DataStore
        #[arg(short, long)]
        master_password: Option<String>,
    },
}
