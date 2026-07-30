use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub use crate::ec::EcWebKey;
pub use crate::rsa::RsaWebKey;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "alg")]
pub enum JsonWebKey {
    ES256(EcWebKey),
    ES384(EcWebKey),
    ES512(EcWebKey),
    RS256(RsaWebKey),
    RS384(RsaWebKey),
    RS512(RsaWebKey),
    PS256(RsaWebKey),
    PS384(RsaWebKey),
    PS512(RsaWebKey),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct JsonWebKeySet {
    pub keys: HashSet<JsonWebKey>,
}
