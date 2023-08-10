use tezos_crypto_rs::hash::ContractTz1Hash;

#[derive(Debug)]
pub struct Account {
    pub public_key_hash: ContractTz1Hash,
    pub nonce: u64,
}
