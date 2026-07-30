use base64ct::{Base64Unpadded, Encoding};
use serde::{de, ser};
use std::fmt;

use super::encoded_list::EncodedList;

pub type BoxedChain = Box<[Box<[u8]>]>;

pub struct ChainField;

impl ChainField {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<BoxedChain>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_option(Visitor)
    }

    pub fn serialize<S>(value: &Option<Box<[Box<[u8]>]>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match value {
            Some(value) => {
                let enc_vec: Vec<String> = value
                    .into_iter()
                    .map(|x| Base64Unpadded::encode_string(x))
                    .collect();
                let enc_str: String =
                    serde_json::to_string(&enc_vec).map_err(ser::Error::custom)?;

                serializer.serialize_some(&enc_str)
            }
            None => serializer.serialize_none(),
        }
    }
}

struct Visitor;

impl<'de> de::Visitor<'de> for Visitor {
    type Value = Option<Box<[Box<[u8]>]>>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an option with a json encoded array of base64 strings")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(Self)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let dec_vec: EncodedList = serde_json::from_str(v).map_err(de::Error::custom)?;

        Ok(Some(dec_vec.inner))
    }
}
