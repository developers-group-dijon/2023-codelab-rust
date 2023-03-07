use anyhow::{bail, Result};
use base64::{engine::general_purpose, Engine};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, marker::PhantomData};
use thiserror::Error;

use crate::{
    crypto::{self, EncryptedMessage},
    utils::{get_store_file_path, get_store_folder_path},
};

/// Possible errors while operating with the DataStore.
#[derive(Debug, Error)]
pub enum DataStoreError {
    #[error("Datastore not found in the system")]
    NotFound,
    #[error("Key {0} not found in the datastore")]
    KeyNotFound(String),
    #[error("Key {0} already exists in the datastore")]
    KeyAlreadyExists(String),
}

/// DataStore representation for JSON serialization & deserialization.
///
/// This is not the real DataStore, but a given representation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct StoreModel {
    /// Actual data store.
    data: HashMap<String, PasswordStore>,
}

/// Representation of password data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordStore {
    /// A label (identifier) linked to a password.
    pub label: String,
    /// A login linked to a password (optionnal).
    pub login: Option<String>,
    /// The actual password.
    pub password: String,
    /// An URL linked to a password (optionnal).
    pub url: Option<String>,
    /// A comment linked to a password (optionnal).
    pub comment: Option<String>,
    /// The creation date for the password data.
    pub creation_date: DateTime<Utc>,
}

/// Locked state representation for the DataStore.
#[derive(Default)]
pub struct Locked;

/// Unlocked state representation for the DataStore.
#[derive(Default)]
pub struct Unlocked;

/// Representation of the DataStore.
#[derive(Default, Debug, Clone)]
pub struct DataStore<State = Locked> {
    /// A Map storing all password data.
    data: HashMap<String, PasswordStore>,
    /// The state of the DataStore, Either Locked or Unlocked.
    state: PhantomData<State>,
    /// Master password for this DataStore.
    master_password: String,
}

/// Implementation for the `Locked` state of the DataStore.
impl DataStore<Locked> {
    /// Unlocks the DataStore given a master password.
    ///
    /// This will return an error if :
    /// - the DataStore is not initialized.
    /// - the content of the DataStore cannot be loaded (various reasons).
    pub fn unlock(self, master_password: &str) -> Result<DataStore<Unlocked>> {
        if !self.is_initialized()? {
            bail!(DataStoreError::NotFound);
        }

        let datastore_content = self.load_content()?;

        let mut buffer = Vec::<u8>::new();

        general_purpose::STANDARD.decode_vec(&datastore_content, &mut buffer)?;

        let mid_cipher_content = String::from_utf8(buffer)?;

        let cipher_content = serde_json::from_str::<EncryptedMessage>(&mid_cipher_content)?;

        let store_model_content = crypto::decrypt(cipher_content, master_password)?;

        let model = serde_json::from_str::<StoreModel>(&store_model_content)?;

        Ok(DataStore {
            data: model.data,
            state: PhantomData::<Unlocked>,
            master_password: master_password.into(),
        })
    }
}

/// Implementation for the `Unlocked` state of the DataStore.
impl DataStore<Unlocked> {
    /// Locks the DataStore and saves it to the file system.
    ///
    /// This will return an error if :
    /// - the content of the DataStore cannot be saved & crypted (various reasons).
    pub fn lock(self) -> Result<DataStore<Locked>> {
        let store_model = StoreModel { data: self.data };

        let store_model_content = serde_json::to_string(&store_model)?;

        let cipher_content = crypto::encrypt(&store_model_content, &self.master_password)?;

        let mid_cipher_content = serde_json::to_string(&cipher_content)?;

        let mut b64_content = String::new();

        general_purpose::STANDARD.encode_string(mid_cipher_content, &mut b64_content);

        fs::write(get_store_file_path()?, b64_content)?;

        Ok(DataStore::default())
    }

    /// Gets a reference to a given password dataset by its identifier.
    ///
    /// This will return an error if the passsword dataset cannot be found.
    pub fn get(&self, key: &str) -> Result<&PasswordStore> {
        if !self.data.contains_key(key) {
            bail!(DataStoreError::KeyNotFound(key.to_string()));
        }

        Ok(self.data.get(key).unwrap())
    }

    /// Inserts a new password dataset into the DataStore.
    ///
    /// This will return an error if the identifier of this dataset already exists.
    pub fn insert(&mut self, new_store: &PasswordStore) -> Result<()> {
        let label = new_store.label.clone();

        if self.data.contains_key(&label) {
            bail!(DataStoreError::KeyAlreadyExists(label));
        }

        self.data.insert(label, new_store.clone());

        Ok(())
    }

    /// deletes a password dataset from the DataStore.
    ///
    /// This will return an error if the passsword dataset cannot be found.
    pub fn delete(&mut self, key: &str) -> Result<()> {
        if !self.data.contains_key(key) {
            bail!(DataStoreError::KeyNotFound(key.to_string()));
        }

        self.data.remove(key);

        Ok(())
    }

    /// returns the actual Map of password dataset.
    pub fn data(&self) -> HashMap<String, PasswordStore> {
        self.data.clone()
    }

    /// Destroys the current DataStore by removing it from the filesystem.
    ///
    /// This will return an error if :
    /// - the DataStore file cannot be found in the filesystem.
    /// - the DataStore file cannot be removed from the filesystem.
    pub fn destroy(self) -> Result<()> {
        fs::remove_file(get_store_file_path()?)?;

        Ok(())
    }
}

/// Default implementation of the DataStore.
impl DataStore {
    /// Initializes a DataStore instance to be operated on.
    pub fn new() -> DataStore<Locked> {
        Self {
            data: Default::default(),
            state: Default::default(),
            master_password: Default::default(),
        }
    }

    /// Checks either the current DataStore instance is initialized or not.
    ///
    /// This will return an error if :
    /// - the DataStore cannot be found in the filesystem.
    pub fn is_initialized(&self) -> Result<bool> {
        let datastore_path = get_store_file_path()?;

        let exists = datastore_path.try_exists();

        if let Err(error) = exists {
            bail!(error);
        }

        Ok(exists.unwrap())
    }

    /// Initializes the DataStore in the filesystem.
    ///
    /// This will return an error if :
    /// - the DataStore file cannot be created.
    pub fn initialize(&self, master_password: &str) -> Result<DataStore<Locked>> {
        let store_folder = get_store_folder_path()?;

        if !store_folder.exists() {
            fs::create_dir(store_folder)?;
        }

        let store_file = get_store_file_path()?;

        if !store_file.exists() {
            fs::write(get_store_file_path()?, "")?
        }

        let mock = DataStore {
            data: Default::default(),
            state: PhantomData::<Unlocked>,
            master_password: master_password.into(),
        };

        let locked = mock.lock()?;

        Ok(locked)
    }

    /// Loads the content of the DataStore file.
    ///
    /// This will return an error if :
    /// - the DataStore file cannot be found in the filesystem.
    /// - the DataStore file cannot be read.
    fn load_content(&self) -> Result<String> {
        Ok(fs::read_to_string(get_store_file_path()?)?)
    }
}
