use crate::{
    cli::{Cli, Command},
    console_utils::ConsoleIO,
    data_store::DataStore,
};
use anyhow::{bail, Result};
use inquire::PasswordDisplayMode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HandlingError {
    #[error("Datastore already initialized")]
    AlreadyInitialized,
    #[error("Datastore must be initialized before using")]
    NotInitialized,
    #[error("Datastore destroy aborted")]
    DestroyAborted,
}

pub fn handle(cli: &Cli) -> Result<()> {
    match cli.command.clone() {
        Command::List {
            mut master_password,
        } => {
            if master_password.is_none() {
                master_password = Some(require_master_password()?);
            }

            list(&master_password.unwrap())?
        }
        Command::Init {
            mut master_password,
        } => {
            if master_password.is_none() {
                master_password = Some(require_master_password()?);
            }

            init(&master_password.unwrap())?
        }
        Command::Add {
            mut master_password,
        } => {
            if master_password.is_none() {
                master_password = Some(require_master_password()?);
            }

            add(&master_password.unwrap())?
        }
        Command::Delete {
            mut master_password,
            name,
        } => {
            if master_password.is_none() {
                master_password = Some(require_master_password()?);
            }

            delete(&name, &master_password.unwrap())?
        }
        Command::Dump {
            mut master_password,
            name,
        } => {
            if master_password.is_none() {
                master_password = Some(require_master_password()?);
            }

            dump(&name, &master_password.unwrap())?
        }
        Command::Generate {
            mut master_password,
        } => {
            if master_password.is_none() {
                master_password = Some(require_master_password()?);
            }

            generate(&master_password.unwrap())?
        }
        Command::Destroy {
            mut master_password,
        } => {
            if master_password.is_none() {
                master_password = Some(require_master_password()?);
            }

            destroy(&master_password.unwrap())?
        }
    }

    Ok(())
}

fn list(master_password: &str) -> Result<()> {
    let data_store = DataStore::new();

    if !data_store.is_initialized()? {
        bail!(HandlingError::NotInitialized)
    }

    let opened = data_store.unlock(master_password)?;

    let headers: Vec<String> = vec![
        "label".to_string(),
        "url".to_string(),
        "comment".to_string(),
        "creation date".to_string(),
    ];

    let mut lines: Vec<Vec<String>> = vec![];

    for (_, data) in opened.data() {
        lines.push(vec![
            data.label,
            format!("{:?}", data.url),
            format!("{:?}", data.comment),
            format!("{:?}", data.creation_date),
        ]);
    }

    let console = ConsoleIO::new();

    console.string_table(headers, lines);

    Ok(())
}

fn init(master_password: &str) -> Result<()> {
    let data_store = DataStore::new();

    if data_store.is_initialized()? {
        bail!(HandlingError::AlreadyInitialized)
    }

    data_store.initialize(master_password)?;

    let console = ConsoleIO::new();

    console.success("Datastore initialized !");

    Ok(())
}

fn add(master_password: &str) -> Result<()> {
    todo!()
}

fn delete(name: &str, master_password: &str) -> Result<()> {
    todo!()
}

fn dump(name: &str, master_password: &str) -> Result<()> {
    todo!()
}

fn generate(master_password: &str) -> Result<()> {
    todo!()
}

fn destroy(master_password: &str) -> Result<()> {
    let data_store = DataStore::new();

    if !data_store.is_initialized()? {
        bail!(HandlingError::NotInitialized)
    }

    let opened = data_store.unlock(master_password)?;

    let console = ConsoleIO::new();

    console.warning("You are about to delete your datastore, this will erase all your passwords");

    let confirmed = console.ask_confirm("Are you sure ?");

    if confirmed {
        opened.destroy()?;
    } else {
        bail!(HandlingError::DestroyAborted);
    }

    Ok(())
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
