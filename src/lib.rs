pub mod cert;
pub mod ec;
pub mod error;
pub mod jwk;
pub mod rsa;

pub type Result<T> = std::result::Result<T, error::Error>;

pub mod prelude {
    pub use crate::cert::Certificate;
    pub use crate::ec::EcWebKey;
    pub use crate::jwk::JsonWebKey;
    pub use crate::jwk::JsonWebKeySet;
    pub use crate::rsa::RsaWebKey;
}

mod encoded_bytes_field;

pub fn from_str<'a>(s: &'a str) -> Result<jwk::JsonWebKey> {
    let value: jwk::JsonWebKey = serde_json::from_str(&s)?;

    Ok(value)
}

pub fn to_string(value: &jwk::JsonWebKey) -> Result<String> {
    let value = serde_json::to_string(value)?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use super::*;
    use base64ct::{Base64UrlUnpadded, Encoding};
    use serde_json::json;

    #[test]
    fn test_rsa_jwk() {
        let webkey = json!({
            "alg": "RSA256",
            "e": "MzYzNTM1MzMzNzBhCg",
            "kid": "test",
            "key_ops": [],
            "n": "NDIzNzMzMzQzMTQyMzQ0NDMxMzUzMTM1Mzc0NDM2Mzg0NDM3NDMzOTQ2NDI0NDM4MzMzMTQyMzY0MzQ1CjQzMzc0MjM0NDMzOTQxMzAzMjMxMzkzNDMxNDQzMTM2NDQzODQ0NDI0NjQ2NDIzMjM1MzMzNjM3NDUzNQo0MjQyMzgzNDM4Mzg0MjM2NDUzNjM5NDQ0NDQ2NDI0NDMyMzgzOTMyNDU0NDQ1Mzk0MzQyMzAzNTQ0NDUKMzkzODMyMzczOTMzNDYzNzMzMzkzNjQxMzMzMzQ1NDI0NDMwMzczMjM2NDIzMzM2Mzk0NDQyNDM0NDM0CjM0MzI0NTQxMzIzMjQzNDE0MTM2NDUzNzMyNDI0MjQxMzIzNjMyNDYzMjQ2MzE0NTQxNDQ0NTMxMzMzMwozMjM0Mzg0NDMxMzY0NTQyMzUzMDQzNDUzOTM2NDY0NDQ1MzMzNjMyNDQzMzM5NDEzMzM2NDYzMjM0MzgKMzIzNzMyNDEzNTQxMzE0MzM2Mzg0NTMzNDU0NTM3MzYzMzQyNDE0MjM2MzA0MjM4MzgzMTM4MzMzMTQ1CjQyNDU0NTM1MzM0NDQ0Mzg0MTQ0NDE0NDMzNDIzNTQxMzEzMjM4NDQ0NTQ0MzYzNjMzMzMzNjQ0MzgzNAozODM5NDI0MzMyMzkzODQxNDUzNDQzNDY0MjM1MzYzNDMyMzk0MzM3NDEzNDMwMzIzOTM3MzMzOTMwNDQKNDMzMjMxMzEzOTMxNDE0MTQyNDQzOTM0NDYzODMwMzc0MzMxNDI0NjM3NDIzNjQxMzA0NjMwMzAzNjQ2CjM5MzE0MTMxNDIzOTQ1MzgzMDMyNDQ0MzQ0NDY0MjMzNDE0NTM5MzgzMDQ2MzI0MTQyNDMzOTM3MzUzNgo0MjQxNDIzNTQzMzgzMDMwNDMzNDQzMzIzNzQxMzE0MjM5NDQzNzQ0MzIzMTM0NDY0MzQ2MzEzMTQ0MzAKMzIzMDQyMzAzNDQ0MzQzMTM3MzQzMTMyMzU0NDM5NDIzNDMwMzAzODQxMzY0MjMwNDYzMDM0MzYzNTM5CjMwNDY0NDQ2NDMzMjM2NDMzMDM4NDQzNzQ2NDM0NDQxNDYzMzQ1MzAzNzQzMzYzOTQzMzAzNTM4MzQzMwo0NDQ2NDYzMzQyMzEzNzMxNDEzNzMxMzQzMTM5MzM0NjQ1MzY0MzMxMzM0MTQxMzk0NjMzMzYzMTMyMzIKMzEzODM4MzIzODQ1Mzc0MjMwNDQ0MjM1MzQzNTM4MzMzODMwMzMzMDMwMzA0MTM4NDI0MjQ1NDI0MTM4CjM1NDQzNTQxMzYzNzM3MzUzOTM2MzUzNTM3MzUzODMzMzM0NTMzNDQzMDMyMzUzNTQzMzE0MjMyNDMzNwozMzM5MGEK",
        });

        let empty_cert = Certificate {
            chain: None,
            sha256_thumbprint: None,
            thumbprint: None,
            url: None,
        };

        let wk_string = webkey.to_string();

        let jwk: JsonWebKey = from_str(wk_string.as_str()).unwrap();

        let JsonWebKey::RSA256(key) = &jwk else {
            panic!("Incorrect algorithm.")
        };

        let enc_mod = Base64UrlUnpadded::encode_string(&key.modulus);
        let enc_exp = Base64UrlUnpadded::encode_string(&key.exponent);

        assert_eq!(key.key_id, "test".into());
        assert_eq!(enc_mod, webkey["n"]);
        assert_eq!(enc_exp, webkey["e"]);
        assert_eq!(key.cert, empty_cert);
        assert_eq!(key.key_type, rsa::KeyType::RSA);
        assert_eq!(key.key_ops.len(), 0);
        assert_eq!(key.use_case, "".into());
    }
}
