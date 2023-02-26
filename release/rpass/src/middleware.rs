use crate::{
    cli::{Cli, Command},
    console_utils::ConsoleIO,
    data_store::{DataStore, PasswordStore, Unlocked},
};
use anyhow::{bail, Result};
use chrono::{DateTime, Local, Utc};
use console::style;
use inquire::{required, PasswordDisplayMode};
use thiserror::Error;
use zxcvbn::zxcvbn;

#[derive(Debug, Error)]
pub enum HandlingError {
    #[error("Datastore already initialized")]
    AlreadyInitialized,
    #[error("Datastore must be initialized before using. Use rpass init.")]
    NotInitialized,
    #[error("Datastore destroy aborted")]
    DestroyAborted,
}

pub fn handle(cli: &Cli) -> Result<()> {
    let master_password: String;

    if cli.master_password.is_none() {
        master_password = require_master_password()?;
    } else {
        master_password = cli.master_password.as_ref().unwrap().to_owned();
    }

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

    for (_, data) in opened.data() {
        let url = sanitize_none_option_string(data.url);
        let login = sanitize_none_option_string(data.login);
        let comment = sanitize_none_option_string(data.comment);

        let local_time: DateTime<Local> = DateTime::from(data.creation_date);

        lines.push(vec![
            data.label,
            url,
            login,
            comment,
            local_time.format("%v %X").to_string(),
            format_password_strength(&data.password)?,
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

    let url = console.ask_question_default("URL for this password:", "");
    let login = console.ask_question_default("Login for this password:", "");

    let password = console
        .input_password("Password:")
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_validator(required!())
        .prompt()?;

    let comment = console.ask_question_default("Comment for this password:", "");

    let data = PasswordStore {
        label: label.clone(),
        login: if "" == login { None } else { Some(login) },
        password,
        url: if "" == url { None } else { Some(url) },
        comment: if "" == comment { None } else { Some(comment) },
        creation_date: Utc::now(),
    };

    opened.insert(&data)?;

    console.success(&format!("Password {label} added !"));

    Ok(opened)
}

fn delete(data_store: DataStore, name: &str, master_password: &str) -> Result<DataStore<Unlocked>> {
    let opened = data_store.unlock(master_password)?;

    Ok(opened)
}

fn dump(data_store: DataStore, name: &str, master_password: &str) -> Result<DataStore<Unlocked>> {
    let opened = data_store.unlock(master_password)?;

    Ok(opened)
}

fn generate(data_store: DataStore, master_password: &str) -> Result<DataStore<Unlocked>> {
    let opened = data_store.unlock(master_password)?;

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
    return if opt.is_none() {
        "".to_string()
    } else {
        opt.unwrap()
    };
}

fn format_password_strength(password: &str) -> Result<String> {
    let estimate = zxcvbn(password, &[])?;

    Ok(match estimate.score() {
        0 => "0/4 - You must change it !".to_string(),
        1 => "1/4 - Nowhere near safe !".to_string(),
        2 => "2/4 - Not safe !".to_string(),
        3 => "3/4 - Safe".to_string(),
        4 => "4/4 - Ultra safe".to_string(),
        _ => bail!("Error while parsing password score !"),
    })
}
