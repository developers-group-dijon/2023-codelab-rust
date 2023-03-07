use std::path::PathBuf;

use anyhow::{bail, Result};
use thiserror::Error;

use crate::constants::{RPASS_DATASTORE_FILENAME, RPASS_SUBFOLDER};

#[derive(Debug, Error)]
pub enum StoreFileError {
    #[error("Cannot find home directory in your system.")]
    HomeDirNotFound,
}

pub fn get_store_folder_path() -> Result<PathBuf> {
    let home_dir_found = dirs::home_dir();

    if home_dir_found.is_none() {
        bail!(StoreFileError::HomeDirNotFound);
    }

    let home_dir = home_dir_found.unwrap();

    Ok(home_dir.join(RPASS_SUBFOLDER))
}

pub fn get_store_file_path() -> Result<PathBuf> {
    Ok(get_store_folder_path()?.join(RPASS_DATASTORE_FILENAME))
}
