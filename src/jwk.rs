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
    RSA256(RsaWebKey),
    RSA384(RsaWebKey),
    RSA512(RsaWebKey),
    PS256(RsaWebKey),
    PS384(RsaWebKey),
    PS512(RsaWebKey),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct JsonWebKeySet {
    pub jwks: HashSet<JsonWebKey>,
}
