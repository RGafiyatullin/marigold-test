use tezos_crypto_rs::hash::{Ed25519Signature, PublicKeyEd25519};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Nonce(pub u64);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PublicKey(pub PublicKeyEd25519);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Signature(pub Ed25519Signature);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Signed<Data> {
    pub public_key: PublicKey,
    pub signature: Signature,
    pub nonce: Nonce,
    pub data: Data,
}
