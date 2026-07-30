use serde::{Deserialize, Serialize};

/// A valid operation the key was created for.
/// See: [RFC7517 4.3](https://datatracker.ietf.org/doc/html/rfc7517#section-4.3)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum KeyOperation {
    /// Decrypt content.
    Decrypt,
    /// Derive Key
    DeriveKey,
    /// Derive bits not to be used as a key.
    DeriveBits,
    /// Encrypt content.
    Encrypt,
    /// Compute digital signature or MAC.
    Sign,
    /// Decrypt key.
    UnwrapKey,
    /// Verify digital signature or MAC.
    Verify,
    /// Encrypt key.
    WrapKey,
    #[serde(untagged)]
    Other(Box<str>),
}
