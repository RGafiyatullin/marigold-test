use tezos_smart_rollup::host::RuntimeError;
use tezos_smart_rollup::storage::path::{OwnedPath, Path};
use tezos_smart_rollup_debug::Runtime;

use crate::kernel::Kernel;
use crate::signing::PublicKey;

pub type AccountID = String;
pub type EntryID = String;
pub type SpaceID = String;
pub type EntryData = Vec<u8>;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("Duplicate Key")]
    DuplicateKey,
    #[error("Serialize error")]
    SerializeError(#[source] serde_json_wasm::ser::Error),
    #[error("Runtime error")]
    RuntimeError(#[source] RuntimeError),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("Deserialize error")]
    DeserializeError(#[source] serde_json_wasm::de::Error),
    #[error("Runtime error")]
    RuntimeError(#[source] RuntimeError),
}

pub fn store_new(
    mut kernel: impl Kernel,
    path: impl Path,
    entry: impl serde::Serialize,
) -> Result<(), StoreError> {
    let None = kernel.host().store_has(&path).map_err(StoreError::RuntimeError)? else {
        return Err(StoreError::DuplicateKey)
    };
    store(kernel, path, entry)
}

pub fn path_account(_account_id: &AccountID) -> OwnedPath {
    unimplemented!()
}
pub fn path_space(_space_id: &SpaceID) -> OwnedPath {
    unimplemented!()
}

pub fn read<Entry>(mut kernel: impl Kernel, path: impl Path) -> Result<Option<Entry>, ReadError>
where
    Entry: serde::de::DeserializeOwned,
{
    match kernel.host().store_read_all(&path) {
        Ok(entry_json) => {
            let entry: Entry = serde_json_wasm::from_slice(entry_json.as_ref())
                .map_err(ReadError::DeserializeError)?;
            Ok(Some(entry))
        },
        Err(RuntimeError::PathNotFound) => Ok(None),
        Err(reason) => Err(ReadError::RuntimeError(reason)),
    }
}

pub fn store(
    mut kernel: impl Kernel,
    path: impl Path,
    entry: impl serde::Serialize,
) -> Result<(), StoreError> {
    let entry_json = serde_json_wasm::to_vec(&entry).map_err(StoreError::SerializeError)?;
    kernel
        .host()
        .store_write_all(&path, entry_json.as_ref())
        .map_err(StoreError::RuntimeError)?;
    Ok(())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub public_key: PublicKey,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Space {
    pub accounts: Vec<AccountID>,
}
