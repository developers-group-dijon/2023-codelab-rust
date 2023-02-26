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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct StoreModel {
    data: HashMap<String, PasswordStore>,
}

#[derive(Debug, Error)]
pub enum DataStoreError {
    #[error("Datastore not found in the system")]
    NotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordStore {
    pub label: String,
    pub password: String,
    pub url: Option<String>,
    pub comment: Option<String>,
    pub creation_date: DateTime<Utc>,
}

#[derive(Default)]
pub struct Locked;

#[derive(Default)]
pub struct Unlocked;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DataStore<State = Locked> {
    data: HashMap<String, PasswordStore>,
    state: PhantomData<State>,
    master_password: String,
}

impl DataStore<Locked> {
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

impl DataStore<Unlocked> {
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

    pub fn get(&self, key: &str) -> Result<PasswordStore> {
        todo!();
    }

    pub fn insert(&mut self, new_store: &PasswordStore) -> Result<()> {
        todo!();
    }

    pub fn delete(&mut self, key: &str) -> Result<()> {
        todo!();
    }

    pub fn data(&self) -> HashMap<String, PasswordStore> {
        self.data.clone()
    }

    pub fn destroy(self) -> Result<()> {
        fs::remove_file(get_store_file_path()?)?;

        Ok(())
    }
}

impl DataStore {
    pub fn new() -> DataStore<Locked> {
        Self {
            data: Default::default(),
            state: Default::default(),
            master_password: Default::default(),
        }
    }

    pub fn is_initialized(&self) -> Result<bool> {
        let datastore_path = get_store_file_path()?;

        let exists = datastore_path.try_exists();

        if let Err(error) = exists {
            bail!(error);
        }

        Ok(exists.unwrap())
    }

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

    fn load_content(&self) -> Result<String> {
        Ok(fs::read_to_string(get_store_file_path()?)?)
    }
}
