use std::collections::HashMap;

pub mod parser;
pub mod writer;

pub mod de;
pub mod error;
pub mod ser;

pub use de::{from_bytes, from_value};
pub use parser::{parse, parse_all, parse_all_incomplete};
pub use ser::{to_bytes, to_value, to_writer};
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeSeq},
    Deserialize, Deserializer, Serialize,
};

/// A bencoded value that has been parsed
///
/// This value implements Serialize and Deserialized, this is useful if you are writing data
/// structure that can contain "any" bencoded value as you can just make the field BencodedValue<'a>
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BencodedValue<'a> {
    /// A binary array, this is a convinience method at binary data is stored in string-like
    /// fiels inside of the data
    Binary(&'a [u8]),

    /// An owned binary array
    BinaryOwned(Vec<u8>),

    /// A string, uses a substring to make it more allocation friendly
    String(&'a str),

    /// An owned string
    StringOwned(String),

    /// A parsed integer, note that it is recommended to use i64 for numbers
    /// otherwise files larger than 4GB cannot be supported
    Integer(i64),

    /// A list of bencoded values
    List(Vec<BencodedValue<'a>>),

    /// A dictionary (map) of bencoded values
    Dictionary(HashMap<&'a str, BencodedValue<'a>>),

    /// A dictionary (map) with owned keys
    DictionaryOwned(HashMap<String, BencodedValue<'a>>),

    /// An empty value. Note that this does **not** exist in bencode, it is used
    /// as a helper value internally to represent empty values and Option::None.
    None,
}

impl<'a> Default for BencodedValue<'a> {
    fn default() -> Self {
        BencodedValue::None
    }
}

impl<'a> BencodedValue<'a> {
    /// Checks if this is an owned value (string, binary, dictionary)
    pub fn is_owned(&self) -> bool {
        match self {
            BencodedValue::BinaryOwned(_)
            | BencodedValue::StringOwned(_)
            | BencodedValue::DictionaryOwned(_)
            | BencodedValue::None => true,
            _ => false,
        }
    }

    /// Checks if the value is a binary (owned or not)
    pub fn is_bin(&self) -> bool {
        match self {
            BencodedValue::Binary(_) | BencodedValue::BinaryOwned(_) => true,
            _ => false,
        }
    }

    /// Assumes self to be a binary (owned or not), consumes it and output its owned content
    pub fn unwrap_bin(self) -> Vec<u8> {
        match self {
            BencodedValue::Binary(v) => v.to_vec(),
            BencodedValue::BinaryOwned(v) => v,
            _ => panic!("not a bin"),
        }
    }

    /// Checks if the value is a string (owned or not)
    pub fn is_string(&self) -> bool {
        match self {
            BencodedValue::String(_) | BencodedValue::StringOwned(_) => true,
            _ => false,
        }
    }

    /// Checks if the value is an int
    pub fn is_int(&self) -> bool {
        match self {
            BencodedValue::Integer(_) => true,
            _ => false,
        }
    }

    /// Checks if the value is a list
    pub fn is_list(&self) -> bool {
        match self {
            BencodedValue::List(_) => true,
            _ => false,
        }
    }

    /// Assumes self to be a list, consumes it and output its owned content
    pub fn unwrap_list(self) -> Vec<BencodedValue<'a>> {
        match self {
            BencodedValue::List(list) => list,
            _ => panic!("not a list"),
        }
    }

    /// Checks if the value is a dictionary (owned or not)
    pub fn is_dict(&self) -> bool {
        match self {
            BencodedValue::Dictionary(_)
            | BencodedValue::DictionaryOwned(_) => true,
            _ => false,
        }
    }

    /// Assumes self to be a dictionary, consumes it and output its owned content
    pub fn unwrap_dict(self) -> HashMap<String, BencodedValue<'a>> {
        match self {
            BencodedValue::Dictionary(dict) => {
                dict.into_iter().map(|(k, v)| (k.to_owned(), v)).collect()
            }

            BencodedValue::DictionaryOwned(dict) => dict,
            _ => panic!("not a dictionary"),
        }
    }

    /// Checks if this is a none. A none does not exist in bencode, it is simply used
    /// to make (de)serialization of options possible/easier
    pub fn is_none(&self) -> bool {
        match self {
            BencodedValue::None => true,
            _ => false,
        }
    }
}

impl<'de> Deserialize<'de> for BencodedValue<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(BencodedValueVisitor)
    }
}

impl<'a> Serialize for BencodedValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            BencodedValue::Binary(bin) => serializer.serialize_bytes(bin),
            BencodedValue::BinaryOwned(bin) => serializer.serialize_bytes(bin),
            BencodedValue::String(str) => serializer.serialize_str(str),
            BencodedValue::StringOwned(str) => serializer.serialize_str(str),
            BencodedValue::Integer(i) => serializer.serialize_i64(*i),
            BencodedValue::List(list) => {
                let mut seq = serializer.serialize_seq(Some(list.len()))?;
                for e in list.iter() {
                    seq.serialize_element(e)?;
                }

                seq.end()
            }
            BencodedValue::Dictionary(dict) => {
                let mut seq = serializer.serialize_map(Some(dict.len()))?;
                for (k, v) in dict.iter() {
                    seq.serialize_entry(k, v)?;
                }

                seq.end()
            }
            BencodedValue::DictionaryOwned(dict) => {
                let mut seq = serializer.serialize_map(Some(dict.len()))?;
                for (k, v) in dict.iter() {
                    seq.serialize_entry(k, v)?;
                }

                seq.end()
            }
            BencodedValue::None => serializer.serialize_none(),
        }
    }
}

struct BencodedValueVisitor;

impl<'de> Visitor<'de> for BencodedValueVisitor {
    type Value = BencodedValue<'de>;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("expected string|bytes|int|list|map")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BencodedValue::Integer(value))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BencodedValue::String(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(v.to_owned())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BencodedValue::StringOwned(v))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_byte_buf(v.to_owned())
    }

    #[inline]
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BencodedValue::Binary(v))
    }

    #[inline]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BencodedValue::BinaryOwned(v))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BencodedValue::None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        BencodedValue::deserialize(deserializer)
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut seq = seq;

        let mut out = if let Some(size) = seq.size_hint() {
            Vec::with_capacity(size)
        } else {
            Vec::new()
        };

        while let Some(v) = seq.next_element()? {
            out.push(v);
        }

        Ok(BencodedValue::List(out))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut map = map;

        let mut out = if let Some(size) = map.size_hint() {
            HashMap::with_capacity(size)
        } else {
            HashMap::new()
        };

        while let Some((k, v)) = map.next_entry()? {
            out.insert(k, v);
        }

        Ok(BencodedValue::Dictionary(out))
    }
}

#[cfg(test)]
mod test_value {
    use crate::{from_bytes, to_bytes, BencodedValue};

    #[test]
    pub fn test_deser() {
        let bytes = b"i3e";
        assert_eq!(from_bytes(bytes), Ok(BencodedValue::Integer(3)));

        let bytes = b"3:abc";
        assert_eq!(from_bytes(bytes), Ok(BencodedValue::String("abc")));

        // A bunch of values is decoded as a tupple (the opposite is not true)
        let bytes = b"3:abci64e";
        assert_eq!(
            from_bytes(bytes),
            Ok((BencodedValue::String("abc"), BencodedValue::Integer(64)))
        );

        // However a list can also be decoded as a tupple!
        let bytes = b"l3:abci64ee";
        assert_eq!(
            from_bytes(bytes),
            Ok((BencodedValue::String("abc"), BencodedValue::Integer(64)))
        );
    }

    #[test]
    pub fn test_ser() {
        let bytes = b"i3e";
        assert_eq!(to_bytes(&BencodedValue::Integer(3)).unwrap(), bytes);

        let bytes = b"3:abc";
        assert_eq!(to_bytes(&BencodedValue::String("abc")).unwrap(), bytes);

        // A tupple is encoded as a list (the opposite is not nescessarily true)
        let bytes = b"l3:abci64ee";
        assert_eq!(
            to_bytes(&(
                BencodedValue::String("abc"),
                BencodedValue::Integer(64)
            ))
            .unwrap(),
            bytes
        );
    }
}
