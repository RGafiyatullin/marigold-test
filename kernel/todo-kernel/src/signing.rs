use tezos_crypto_rs::hash::{Ed25519Signature, PublicKeyEd25519};

use crate::data::AccountID;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Nonce(pub u64);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PublicKey(pub PublicKeyEd25519);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Signature(pub Ed25519Signature);

#[derive(Debug, thiserror::Error)]
#[error("Verification Error")]
pub struct VerificationError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Signed<Message> {
    account_id: AccountID,
    signature: Signature,
    nonce: Nonce,

    data: Message,
}

impl<M> Signed<M>
where
    M: serde::Serialize,
{
    pub fn nonce(&self) -> Nonce {
        self.nonce
    }
    pub fn verify(&self, _public_key: &PublicKey) -> Result<&M, VerificationError> {
        // TODO: actually verify something, okay?

        let _message_digest = self.message_digest();

        Ok(&self.data)
    }

    fn message_digest(&self) -> Vec<u8> {
        unimplemented!()
    }
}
