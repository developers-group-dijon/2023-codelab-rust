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
    let master_password = if cli.master_password.is_none() {
        require_master_password()?
    } else {
        cli.master_password.as_ref().unwrap().to_owned()
    };

    let data_store = DataStore::new();

    if !data_store.is_initialized()? && !matches!(cli.command.clone(), Command::Init) {
        bail!(HandlingError::NotInitialized);
    }

    let unlocked = match cli.command.clone() {
        Command::List => list(data_store, &master_password)?,
        Command::Init => init(data_store, &master_password)?,
        Command::Add => add(data_store, &master_password)?,
        Command::Delete { name } => delete(data_store, &name, &master_password)?,
        Command::Dump { name } => dump(data_store, &name, &master_password)?,
        Command::Generate => generate(data_store, &master_password)?,
    };

    unlocked.lock()?;

    Ok(())
}

/// Initializes the DataStore on the system.
///
/// This will return an error if :
/// - the initialization fails.
/// - the datastore cannot be unlocked.
fn init(data_store: DataStore, master_password: &str) -> Result<DataStore<Unlocked>> {
    data_store.initialize(master_password)?;
    let opened = data_store.unlock(master_password)?;

    let console = ConsoleIO::new();

    console.success("Datastore initialized !");

    Ok(opened)
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
    let console = ConsoleIO::new();

    let mut opened = data_store.unlock(master_password)?;

    let label = console
        .input_text("Label/name for this password:")
        .with_validator(required!())
        .prompt()?;

    if opened.get(&label).is_ok() {
        bail!(HandlingError::KeyAlreadyExists(label));
    }

    let url = console.ask_question_default("URL for this password:", "");
    let login = console.ask_question_default("Login for this password:", "");

    let password = console
        .input_password("Password:")
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_validator(required!())
        .prompt()?;

    let password_strength_label = passwords::format_password_strength(&password)?;
    console.writeln(&format!("Password strength: {password_strength_label}"));

    if passwords::get_password_strength(&password)? < 3 {
        let confirmed = console.ask_confirm(
            "Your password seems to be not safe enough, are you sure you want to store it as it is",
        );

        if !confirmed {
            bail!(HandlingError::AdditionAborted);
        }
    }

    let comment = console.ask_question_default("Comment for this password:", "");

    let data = PasswordStore {
        label: label.clone(),
        login: if login.is_empty() { None } else { Some(login) },
        password,
        url: if url.is_empty() { None } else { Some(url) },
        comment: if comment.is_empty() {
            None
        } else {
            Some(comment)
        },
        creation_date: Utc::now(),
    };

    opened.insert(&data)?;

    console.success(&format!("Password \"{label}\" added !"));

    Ok(opened)
}

/// Lists all the stored password in the DataStore.
///
/// This will return an error if the DataStore cannot be unlocked.
fn list(data_store: DataStore, master_password: &str) -> Result<DataStore<Unlocked>> {
    let opened = data_store.unlock(master_password)?;

    let headers: Vec<String> = vec![
        "Label".to_string(),
        "Url".to_string(),
        "Login".to_string(),
        "Comment".to_string(),
        "Creation date".to_string(),
        "Password strength".to_string(),
    ];

    let mut lines: Vec<Vec<String>> = vec![];

    for (_, data) in opened.data().iter().sorted_by_key(|x| x.0) {
        let url = sanitize_none_option_string(data.url.clone());
        let login = sanitize_none_option_string(data.login.clone());
        let comment = sanitize_none_option_string(data.comment.clone());

        let local_time: DateTime<Local> = DateTime::from(data.creation_date);

        lines.push(vec![
            data.label.clone(),
            url,
            login,
            comment,
            local_time.format("%v %X").to_string(),
            passwords::format_password_strength(&data.password)?,
        ]);
    }

    let console = ConsoleIO::new();

    console.string_table(headers, lines);

    Ok(opened)
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
    let console = ConsoleIO::new();

    let mut opened = data_store.unlock(master_password)?;

    if opened.get(label).is_err() {
        bail!(HandlingError::KeyNotFound(label.into()));
    }

    let confirmed = console.ask_confirm(&format!(
        "Are you sure you want to delete entry \"{label}\""
    ));

    if !confirmed {
        bail!(HandlingError::DeleteAborted);
    }

    opened.delete(label)?;

    console.success(&format!("Entry \"{label}\" deleted !"));

    Ok(opened)
}

/// Prints a password to the standard output given its label.
///
/// ⚠️ This should only be used in command chains. ⚠️
///
/// This will return an error if :
/// - the DataStore cannot be unlocked.
/// - the password label is not found in the DataStore.
fn dump(data_store: DataStore, label: &str, master_password: &str) -> Result<DataStore<Unlocked>> {
    let console = ConsoleIO::new();

    let opened = data_store.unlock(master_password)?;

    if let Ok(data) = opened.get(label) {
        console.write(&data.password);
    } else {
        bail!(HandlingError::KeyNotFound(label.into()));
    }

    Ok(opened)
}

/// Generates a strong random password of 24 chars.
///
/// This will return an error if :
/// - the DataStore cannot be unlocked.
/// - the password cannot be generated.
fn generate(data_store: DataStore, master_password: &str) -> Result<DataStore<Unlocked>> {
    let console = ConsoleIO::new();

    let opened = data_store.unlock(master_password)?;

    let generated = passwords::generate(24)?;

    console.success(&format!("Password generated: {generated}"));

    Ok(opened)
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
