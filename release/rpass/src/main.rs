use anyhow::Result;
use data_store::DataStore;
pub mod cli;
pub mod console_utils;
pub mod constants;
pub mod crypto;
pub mod data_store;
pub mod middleware;
pub mod utils;

fn main() -> Result<()> {
    let data_store = DataStore::new()?;

    Ok(())
}
