use serde::{Deserialize, Serialize};

pub use crate::ec::EcWebKey;
pub use crate::rsa::RsaWebKey;

#[derive(Deserialize, Serialize)]
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
