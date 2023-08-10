#[derive(Debug, thiserror::Error)]
pub enum HashingError {
    #[error("Blake2b Error")]
    Blake2bError(tezos_crypto_rs::blake2b::Blake2bError),

    #[error("Invalid digest size")]
    InvalidDigestSize,
}

macro_rules! define_blake2b {
    ($name:ident, $size:expr) => {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            inner: [u8; $size],
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                self.inner
                    .iter()
                    .fold("".to_string(), |acc, elt| format!("{}{:02x?}", acc, elt))
            }
        }

        impl $name {
            pub fn hash(data: &[u8]) -> Result<Self, $crate::blake2::HashingError> {
                let digest = tezos_crypto_rs::blake2b::digest(data, $size)
                    .map_err($crate::blake2::HashingError::Blake2bError)?;
                let out = Self {
                    inner: <[u8; $size] as TryFrom<_>>::try_from(digest.as_ref())
                        .map_err(|_| $crate::blake2::HashingError::InvalidDigestSize)?,
                };
                Ok(out)
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                &self.inner
            }
        }
    };
}
define_blake2b!(Blake2b, 32);
define_blake2b!(Blake2b20, 20);
