use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::{cert::Certificate, encoded_bytes_field::EncodedBytesField};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RsaWebKey {
    #[serde(flatten)]
    pub cert: Certificate,
    #[serde(rename = "kid")]
    pub key_id: Box<str>,
    #[serde(default)]
    pub key_ops: Box<[Box<str>]>,
    #[serde(default)]
    pub key_type: KeyType,
    #[serde(rename = "use", default)]
    pub use_case: Box<str>,
    #[serde(rename = "e", with = "EncodedBytesField")]
    pub exponent: Box<[u8]>,
    #[serde(rename = "n", with = "EncodedBytesField")]
    pub modulus: Box<[u8]>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kty")]
pub enum KeyType {
    RSA,
}

impl Default for RsaWebKey {
    fn default() -> Self {
        Self {
            cert: Certificate::default(),
            key_id: "".into(),
            key_ops: [].into(),
            key_type: KeyType::default(),
            exponent: [].into(),
            modulus: [].into(),
            use_case: "".into(),
        }
    }
}

// Derive equality based off key_id, not accurate but --
// just like database records, we can save a lot of
// performance by just assuming the key_id is accurate.
impl Eq for RsaWebKey {}

impl Hash for RsaWebKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key_id.hash(state)
    }
}

impl PartialEq for RsaWebKey {
    fn eq(&self, other: &Self) -> bool {
        self.key_id.eq(&other.key_id)
    }
}

impl Default for KeyType {
    fn default() -> Self {
        Self::RSA
    }
}
