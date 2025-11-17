use base64ct::{Base64Unpadded, Encoding};
use serde::de;
use std::fmt;

pub struct EncodedItem;

impl<'de> de::DeserializeSeed<'de> for EncodedItem {
    type Value = Box<[u8]>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Box<[u8]>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a utf8 encoded byte string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let decoded_bytes: Vec<u8> =
                    Base64Unpadded::decode_vec(v).map_err(de::Error::custom)?;

                Ok(decoded_bytes.into_boxed_slice())
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
