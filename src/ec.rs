use serde::{Deserialize, Serialize};

use crate::{cert::Certificate, encoded_bytes_field::EncodedBytesField};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
