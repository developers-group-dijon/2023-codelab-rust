#![allow(unused)]
use crate::{
    cli::{Cli, Command},
    console_utils::ConsoleIO,
    data_store::{DataStore, PasswordStore, Unlocked},
    passwords,
};
use anyhow::{bail, Result};
use chrono::{DateTime, Local, Utc};
use inquire::{required, PasswordDisplayMode};
use itertools::Itertools;
use thiserror::Error;

/// Possible errors upon handling passwords and datastore.
#[derive(Debug, Error)]
pub enum HandlingError {
    #[error("Datastore already initialized")]
    AlreadyInitialized,
    #[error("Datastore must be initialized before using. Use rpass init.")]
    NotInitialized,
    #[error("Datastore destroy aborted")]
    DestroyAborted,
    #[error("Password addition aborted")]
    AdditionAborted,
    #[error("Password deletion aborted")]
    DeleteAborted,
    #[error("Key \"{0}\" already exists in datastore")]
    KeyAlreadyExists(String),
    #[error("Key \"{0}\" is not in datastore")]
    KeyNotFound(String),
}

/// Entrypoint of the middleware.
///
/// Dispatch given CLI arguments to dedicated functions.
///
/// This will transmit any error in middleware to the `main()` function.
pub fn handle(cli: &Cli) -> Result<()> {
    todo!();

    Ok(())
}

/// Initializes the DataStore on the system.
///
/// This will return an error if :
/// - the initialization fails.
/// - the datastore cannot be unlocked.
fn init(data_store: DataStore, master_password: &str) -> Result<DataStore<Unlocked>> {
    todo!();
}

/// Adds a password to the DataStore.
///
/// This will return an error if :
/// - the datastore cannot be unlocked.
/// - the password label cannot be read.
/// - the password label already exists in the DataStore.
/// - the password cannot be read.
/// - the password strength cannot be calculated.
/// - the "unsafe password addition" is not confirmed.
/// - the password data cannot be recorded in the DataStore.
fn add(data_store: DataStore, master_password: &str) -> Result<DataStore<Unlocked>> {
    todo!();
}

/// Lists all the stored password in the DataStore.
///
/// This will return an error if the DataStore cannot be unlocked.
fn list(data_store: DataStore, master_password: &str) -> Result<DataStore<Unlocked>> {
    todo!();
}

/// Deletes password data from the DataStore given a password label.
///
/// This will return an error if :
/// - the datastore cannot be unlocked.
/// - the label of the password to delete is not found in the DataStore.
/// - the deletion is not confirmed.
/// - the actual data deletion cannot be made.
fn delete(
    data_store: DataStore,
    label: &str,
    master_password: &str,
) -> Result<DataStore<Unlocked>> {
    todo!();
}

/// Prints a password to the standard output given its label.
///
/// ⚠️ This should only be used in command chains. ⚠️
///
/// This will return an error if :
/// - the DataStore cannot be unlocked.
/// - the password label is not found in the DataStore.
fn dump(data_store: DataStore, label: &str, master_password: &str) -> Result<DataStore<Unlocked>> {
    todo!();
}

/// Generates a strong random password of 24 chars.
///
/// This will return an error if :
/// - the DataStore cannot be unlocked.
/// - the password cannot be generated.
fn generate(data_store: DataStore, master_password: &str) -> Result<DataStore<Unlocked>> {
    todo!();
}

///////////////////// UTILITY FUNCTIONS /////////////////////

/// Requests the master password to unlock a DataStore.
///
/// This will return an error if the password cannot be read.
fn require_master_password() -> Result<String> {
    let console_utils = ConsoleIO::new();

    let password_question = console_utils.input_password("Enter master password:");

    let master_password = password_question
        .without_confirmation()
        .with_display_mode(PasswordDisplayMode::Masked)
        .prompt()?;

    Ok(master_password)
}

/// Transforms an `Option<String>` into a `String` by filling a default empty string
/// in the value in place of a `None`.
fn sanitize_none_option_string(opt: Option<String>) -> String {
    if let Some(value) = opt {
        value
    } else {
        "".into()
    }
}
