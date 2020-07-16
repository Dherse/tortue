use std::collections::HashMap;

pub mod parser;
pub mod writer;

pub mod de;
pub mod error;
pub mod ser;

pub use de::{from_bytes, from_value};
pub use parser::{parse, parse_all, parse_all_incomplete};
pub use ser::{to_bytes, to_value, to_writer};

/// A bencoded value that has been parsed
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

    /// An empty value
    None,
}

impl<'a> BencodedValue<'a> {
    pub fn is_bin(&self) -> bool {
        match self {
            BencodedValue::Binary(_) | BencodedValue::BinaryOwned(_) => true,
            _ => false,
        }
    }

    pub fn unwrap_bin(self) -> Vec<u8> {
        match self {
            BencodedValue::Binary(v) => v.to_vec(),
            BencodedValue::BinaryOwned(v) => v,
            _ => panic!("not a bin"),
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            BencodedValue::String(_) | BencodedValue::StringOwned(_) => true,
            _ => false,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            BencodedValue::Integer(_) => true,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            BencodedValue::List(_) => true,
            _ => false,
        }
    }

    pub fn unwrap_list(self) -> Vec<BencodedValue<'a>> {
        match self {
            BencodedValue::List(list) => list,
            _ => panic!("not a list"),
        }
    }

    pub fn is_dict(&self) -> bool {
        match self {
            BencodedValue::Dictionary(_)
            | BencodedValue::DictionaryOwned(_) => true,
            _ => false,
        }
    }

    pub fn unwrap_dict(self) -> HashMap<String, BencodedValue<'a>> {
        match self {
            BencodedValue::Dictionary(dict) => {
                dict.into_iter().map(|(k, v)| (k.to_owned(), v)).collect()
            }

            BencodedValue::DictionaryOwned(dict) => dict,
            _ => panic!("not a dictionary"),
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            BencodedValue::None => true,
            _ => false,
        }
    }
}
