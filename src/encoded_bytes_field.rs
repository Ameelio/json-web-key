use base64ct::{Base64UrlUnpadded, Encoding};
use serde::{de, ser};
use std::fmt;

pub struct EncodedBytesField;
pub struct EncodedOptionalBytesField;

impl EncodedBytesField {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Box<[u8]>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor)?.ok_or_else(|| {
            de::Error::invalid_type(de::Unexpected::Option, &"a base64 encoded string")
        })
    }

    pub fn serialize<S>(value: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let encoded_str: String = Base64UrlUnpadded::encode_string(&value);

        serializer.serialize_str(&encoded_str)
    }
}

impl EncodedOptionalBytesField {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Box<[u8]>>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_option(Visitor)
    }

    pub fn serialize<S>(value: &Option<Box<[u8]>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match value {
            Some(x) => {
                let encoded_str: String = Base64UrlUnpadded::encode_string(&x);
                serializer.serialize_some(&encoded_str)
            }
            None => serializer.serialize_none(),
        }
    }
}

struct Visitor;

impl<'de> de::Visitor<'de> for Visitor {
    type Value = Option<Box<[u8]>>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a base64 encoded string")
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
        Ok(deserializer.deserialize_str(Self)?)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decoded_bytes: Vec<u8> = Base64UrlUnpadded::decode_vec(v).map_err(de::Error::custom)?;

        Ok(Some(decoded_bytes.into_boxed_slice()))
    }
}
