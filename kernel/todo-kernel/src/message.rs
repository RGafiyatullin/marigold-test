use crate::data::{AccountID, EntryData, EntryID, SpaceID};
use crate::signing::{PublicKey, Signed};

pub const MAGIC: u8 = 0xAF;

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("Message did not contain the expected magic: {:02X?}", MAGIC)]
    BadMagic,

    #[error("Failed to parse json")]
    JsonParseError(#[source] serde_json_wasm::de::Error),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TodoMessage {
    AccountRegister(AccountRegister),
    SpaceCreate(AccountID, SpaceID, Signed<SpaceCreate>),
    TodoEntrySet(AccountID, SpaceID, EntryID, Signed<TodoEntrySet>),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct AccountRegister {
    pub account_id: AccountID,
    pub public_key: PublicKey,
    pub signed: Signed<()>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct SpaceCreate {
    pub space_id: SpaceID,
    pub accounts: Vec<AccountID>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct TodoEntrySet {
    pub space_id: SpaceID,
    pub entry_id: EntryID,
    pub data: EntryData,
}

impl TodoMessage {
    pub fn decode(input: &[u8]) -> Result<Self, DecodeError> {
        match input {
            [MAGIC, data @ ..] => {
                let message: Self =
                    serde_json_wasm::from_slice(data).map_err(DecodeError::JsonParseError)?;

                Ok(message)
            },
            &[..] => Err(DecodeError::BadMagic),
        }
    }
}
