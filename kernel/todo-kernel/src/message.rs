pub mod signed;

use signed::Signed;

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
    Create(Signed<TodoCreate>),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TodoCreate {}

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
