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

fn init(data_store: DataStore, master_password: &str) -> Result<DataStore<Unlocked>> {
    data_store.initialize(master_password)?;
    let opened = data_store.unlock(master_password)?;

    let console = ConsoleIO::new();

    console.success("Datastore initialized !");

    Ok(opened)
}

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

fn delete(data_store: DataStore, name: &str, master_password: &str) -> Result<DataStore<Unlocked>> {
    let console = ConsoleIO::new();

    let mut opened = data_store.unlock(master_password)?;

    if opened.get(name).is_err() {
        bail!(HandlingError::KeyNotFound(name.into()));
    }

    let confirmed =
        console.ask_confirm(&format!("Are you sure you want to delete entry \"{name}\""));

    if !confirmed {
        bail!(HandlingError::DeleteAborted);
    }

    opened.delete(name)?;

    console.success(&format!("Entry \"{name}\" deleted !"));

    Ok(opened)
}

fn dump(data_store: DataStore, name: &str, master_password: &str) -> Result<DataStore<Unlocked>> {
    let console = ConsoleIO::new();

    let opened = data_store.unlock(master_password)?;

    if let Ok(data) = opened.get(name) {
        console.write(&data.password);
    } else {
        bail!(HandlingError::KeyNotFound(name.into()));
    }

    Ok(opened)
}

fn generate(data_store: DataStore, master_password: &str) -> Result<DataStore<Unlocked>> {
    let console = ConsoleIO::new();

    let opened = data_store.unlock(master_password)?;

    let generated = passwords::generate(24)?;

    console.success(&format!("Password generated: {generated}"));

    Ok(opened)
}

fn require_master_password() -> Result<String> {
    let console_utils = ConsoleIO::new();

    let password_question = console_utils.input_password("Enter master password:");

    let master_password = password_question
        .without_confirmation()
        .with_display_mode(PasswordDisplayMode::Masked)
        .prompt()?;

    Ok(master_password)
}

fn sanitize_none_option_string(opt: Option<String>) -> String {
    if let Some(value) = opt {
        value
    } else {
        "".into()
    }
}
