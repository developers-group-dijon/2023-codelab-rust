use crate::cli::{Cli, Command};
use anyhow::Result;

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
    }

    Ok(())
}

fn list(master_password: &str) -> Result<()> {
    todo!()
}

fn init(master_password: &str) -> Result<()> {
    todo!()
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

fn require_master_password() -> Result<String> {
    todo!()
}
