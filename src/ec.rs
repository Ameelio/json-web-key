use serde::{Deserialize, Serialize};
use std::hash;

use crate::{cert::Certificate, encoded_bytes_field::EncodedBytesField};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EcWebKey {
    #[serde(flatten)]
    pub cert: Certificate,
    #[serde(rename = "kid")]
    pub key_id: Box<str>,
    pub key_ops: Box<[Box<str>]>,
    pub key_type: KeyType,
    #[serde(rename = "use")]
    pub use_case: Box<str>,
    #[serde(rename = "crv")]
    pub curve: Box<str>,
    #[serde(with = "EncodedBytesField")]
    pub x: Box<[u8]>,
    #[serde(with = "EncodedBytesField")]
    pub y: Box<[u8]>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kty")]
pub enum KeyType {
    EC,
}

impl Default for EcWebKey {
    fn default() -> Self {
        Self {
            cert: Certificate::default(),
            curve: "".into(),
            key_id: "".into(),
            key_ops: [].into(),
            key_type: KeyType::default(),
            x: [].into(),
            y: [].into(),
            use_case: "".into(),
        }
    }
}
// Derive equality based off key_id, not accurate but --
// just like database records, we can save a lot of
// performance by just assuming the key_id is accurate.
impl Eq for EcWebKey {}

impl hash::Hash for EcWebKey {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.key_id.hash(state)
    }
}

impl PartialEq for EcWebKey {
    fn eq(&self, other: &Self) -> bool {
        self.key_id.eq(&other.key_id)
    }
}

impl Default for KeyType {
    fn default() -> Self {
        Self::EC
    }
}
