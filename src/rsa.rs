use serde::{Deserialize, Serialize};

use crate::{cert::Certificate, encoded_bytes_field::EncodedBytesField};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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

impl Default for KeyType {
    fn default() -> Self {
        Self::RSA
    }
}
