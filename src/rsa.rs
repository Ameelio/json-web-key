use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::cert::Certificate;
use crate::encoded_bytes_field::EncodedBytesField;
use crate::key_operation::KeyOperation;
use crate::use_case::UseCase;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RsaWebKey {
    /// The X.509 Certificate
    #[serde(default, flatten)]
    pub cert: Certificate,
    #[serde(rename = "kid")]
    pub key_id: Box<str>,
    /// Valid operations the key can perform.
    #[serde(default)]
    pub key_ops: Box<[KeyOperation]>,
    #[serde(default, flatten)]
    pub key_type: KeyType,
    /// Public Key Use Case (signature or encryption)
    #[serde(rename = "use", default)]
    pub use_case: UseCase,
    /// A public exponent (Usually 65537) used for
    /// encryption and signature verification.
    #[serde(rename = "e", with = "EncodedBytesField")]
    pub exponent: Box<[u8]>,
    /// The product of two large prime numbers, forms the main
    /// body of the public key.
    #[serde(rename = "n", with = "EncodedBytesField")]
    pub modulus: Box<[u8]>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kty")]
pub enum KeyType {
    #[default]
    RSA,
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
