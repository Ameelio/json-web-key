use serde::de;
use std::fmt;

use super::encoded_item::EncodedItem;

pub struct EncodedList {
    pub inner: Box<[Box<[u8]>]>,
}

impl<'de> de::Deserialize<'de> for EncodedList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let inner = deserializer.deserialize_seq(Visitor)?;

        Ok(Self { inner })
    }
}

struct Visitor;

impl<'de> de::Visitor<'de> for Visitor {
    type Value = Box<[Box<[u8]>]>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an array of base64 encoded strings")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut value: Vec<Box<[u8]>> = Vec::new();

        // Try to only alloc when needed.
        if let Some(size_hint) = seq.size_hint() {
            value.reserve(size_hint);
        }

        while let Some(elem) = seq.next_element_seed(EncodedItem)? {
            value.push(elem);
        }

        Ok(value.into_boxed_slice())
    }
}
