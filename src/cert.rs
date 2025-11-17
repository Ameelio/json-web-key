use serde::{Deserialize, Serialize};

use crate::encoded_bytes_field::EncodedOptionalBytesField;

mod chain_field;
mod encoded_item;
mod encoded_list;

use chain_field::ChainField;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Certificate {
    #[serde(
        default,
        rename = "x5c",
        skip_serializing_if = "Option::is_none",
        with = "ChainField"
    )]
    pub chain: Option<Box<[Box<[u8]>]>>,
    #[serde(
        default,
        rename = "x5t#S256",
        skip_serializing_if = "Option::is_none",
        with = "EncodedOptionalBytesField"
    )]
    pub sha256_thumbprint: Option<Box<[u8]>>,
    #[serde(
        default,
        rename = "x5t",
        skip_serializing_if = "Option::is_none",
        with = "EncodedOptionalBytesField"
    )]
    pub thumbprint: Option<Box<[u8]>>,
    #[serde(default, rename = "x5u", skip_serializing_if = "Option::is_none")]
    pub url: Option<Box<str>>,
}
