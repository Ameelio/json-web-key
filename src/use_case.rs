use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum UseCase {
    #[default]
    #[serde(rename = "")]
    None,
    #[serde(rename = "enc")]
    Encryption,
    #[serde(rename = "sig")]
    Signature,
    #[serde(untagged)]
    Other(Box<str>),
}

impl UseCase {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}
