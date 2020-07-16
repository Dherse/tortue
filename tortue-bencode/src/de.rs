use crate::{error::Error, parser, BencodedValue};
use serde::{de, Deserialize};

mod map;
mod seq;

pub(crate) struct Deserializer<'data> {
    input: BencodedValue<'data>,
}

/// Deserializes a data structure from an already parsed value
pub fn from_value<'de, T: Deserialize<'de>>(
    value: BencodedValue<'de>,
) -> Result<T, Error> {
    T::deserialize(Deserializer::from_value(value)?)
}

/// Deserializes a data structure from a slice of bytes
pub fn from_bytes<'de, T: Deserialize<'de>>(
    data: &'de [u8],
) -> Result<T, Error> {
    T::deserialize(Deserializer::new(data)?)
}

impl<'data> Deserializer<'data> {
    pub fn new(data: &'data [u8]) -> Result<Self, Error> {
        Self::from_value(if let Ok(input) = parser::parse_all(data) {
            input.1
        } else {
            return Err(Error::Message("failed to parse input".to_owned()));
        })
    }

    pub fn from_value(input: BencodedValue<'data>) -> Result<Self, Error> {
        Ok(Deserializer { input })
    }

    pub fn parse_bool(self) -> Result<bool, Error> {
        match &self.input {
            BencodedValue::Integer(value) => match value {
                1 => Ok(true),
                0 => Ok(false),
                _ => Err(Error::Message(
                    "incorrect bool from int conversion".to_owned(),
                )),
            },
            v => Err(Error::Message(format!(
                "cannot convert from {:?} to bool",
                v
            ))),
        }
    }

    pub fn parse_int(self) -> Result<i64, Error> {
        match self.input {
            BencodedValue::Integer(value) => Ok(value),
            v => Err(Error::Message(format!(
                "cannot convert from {:?} to int",
                v
            ))),
        }
    }

    pub fn parse_uint(self) -> Result<u64, Error> {
        let value = self.parse_int()?;
        if value < 0 {
            Err(Error::Message("uint cannot be negative".to_owned()))
        } else {
            Ok(value as _)
        }
    }

    pub fn parse_float(self) -> Result<f64, Error> {
        Ok(self.parse_int()? as i32 as _)
    }

    pub fn parse_char(self) -> Result<char, Error> {
        match self.input {
            BencodedValue::String(value) => {
                if value.len() == 1 {
                    Ok(value.chars().next().unwrap())
                } else {
                    Err(Error::Message(
                        "incorrect char from string conversion".to_owned(),
                    ))
                }
            }
            BencodedValue::StringOwned(value) => {
                if value.len() == 1 {
                    Ok(value.chars().next().unwrap())
                } else {
                    Err(Error::Message(
                        "incorrect char from string conversion".to_owned(),
                    ))
                }
            }
            v => Err(Error::Message(format!(
                "cannot convert from {:?} to char",
                v
            ))),
        }
    }

    pub fn parse_str(self) -> Result<&'data str, Error> {
        match self.input {
            BencodedValue::String(value) => Ok(value),
            //BencodedValue::StringOwned(value) => Ok(&value),
            v => Err(Error::Message(format!(
                "cannot convert from {:?} to str",
                v
            ))),
        }
    }

    pub fn parse_string(self) -> Result<String, Error> {
        match &self.input {
            BencodedValue::String(value) => Ok((*value).to_owned()),
            BencodedValue::StringOwned(value) => Ok(value.clone()),
            v => Err(Error::Message(format!(
                "cannot convert from {:?} to string",
                v
            ))),
        }
    }

    pub fn parse_bytes(self) -> Result<&'data [u8], Error> {
        match self.input {
            BencodedValue::Binary(value) => Ok(value),
            //BencodedValue::BinaryOwned(value) => Ok(&value[..]),
            v => Err(Error::Message(format!(
                "cannot convert from {:?} to bytes",
                v
            ))),
        }
    }

    pub fn parse_bytes_owned(self) -> Result<Vec<u8>, Error> {
        match self.input {
            BencodedValue::Binary(value) => Ok(value.to_vec()),
            BencodedValue::BinaryOwned(value) => Ok(value),
            v => Err(Error::Message(format!(
                "cannot convert from {:?} to owned bytes",
                v
            ))),
        }
    }
}

impl<'de> de::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn is_human_readable(&self) -> bool {
        false
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match &self.input {
            BencodedValue::Binary(_) => self.deserialize_seq(visitor),
            BencodedValue::BinaryOwned(_) => self.deserialize_seq(visitor),
            BencodedValue::String(_) => self.deserialize_str(visitor),
            BencodedValue::StringOwned(_) => self.deserialize_str(visitor),
            BencodedValue::Integer(_) => self.deserialize_i64(visitor),
            BencodedValue::List(_) => self.deserialize_seq(visitor),
            BencodedValue::Dictionary(_) => self.deserialize_map(visitor),
            BencodedValue::DictionaryOwned(_) => self.deserialize_map(visitor),
            BencodedValue::None => self.deserialize_option(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse_int()? as _)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse_int()? as _)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse_int()? as _)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse_int()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.parse_uint()? as _)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse_uint()? as _)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse_uint()? as _)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.parse_uint()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.parse_float()? as _)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(self.parse_float()? as _)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_char(self.parse_char()?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.input.is_owned() {
            visitor.visit_string(self.parse_string()?)
        } else {
            visitor.visit_borrowed_str(self.parse_str()?)
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.parse_string()?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.input.is_owned() {
            visitor.visit_byte_buf(self.parse_bytes_owned()?)
        } else {
            visitor.visit_borrowed_bytes(self.parse_bytes()?)
        }
    }

    fn deserialize_byte_buf<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.parse_bytes_owned()?)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.input {
            BencodedValue::None => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Message("cannot deserialize units".to_owned()))
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Message("cannot deserialize units".to_owned()))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.input.is_list() {
            let list = self.input.unwrap_list();

            visitor.visit_seq(seq::SeqAccess::new(list))
        } else if self.input.is_bin() {
            visitor.visit_seq(seq::SeqAccess::new(
                self.input
                    .unwrap_bin()
                    .into_iter()
                    .map(|e| BencodedValue::Integer(e as _))
                    .collect(),
            ))
        } else {
            Err(Error::Message(format!(
                "cannot convert from {:?} to list",
                self.input
            )))
        }
    }

    fn deserialize_tuple<V>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.input.is_dict() {
            let map = self.input.unwrap_dict();

            visitor.visit_map(map::MapAccess::new(map))
        } else {
            Err(Error::Message(format!(
                "cannot convert from {:?} to list",
                self.input
            )))
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.input.is_list() {
            self.deserialize_seq(visitor)
        } else if self.input.is_dict() {
            self.deserialize_map(visitor)
        } else {
            Err(Error::Message(format!(
                "cannot convert from {:?} to list/dictionary",
                self.input
            )))
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Message("enums are not supported".to_owned()))
    }

    fn deserialize_identifier<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(
        mut self,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.input = BencodedValue::None;

        visitor.visit_unit()
    }
}

#[cfg(test)]
mod deserialize_tests {
    use super::{from_bytes, from_value};
    use crate::BencodedValue;
    use maplit::hashmap;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    struct TestStruct {
        name: String,
        age: i64,
        friends: Vec<String>,
    }

    #[test]
    fn test_string() {
        let hello_world = "Hello, world!";

        assert_eq!(
            from_value(BencodedValue::StringOwned(hello_world.to_owned())),
            Ok(hello_world.to_owned())
        );

        let bytes = b"13:Hello, world!";

        assert_eq!(from_bytes(bytes), Ok(hello_world.to_owned()));
    }

    #[test]
    fn test_number() {
        assert_eq!(from_value(BencodedValue::Integer(64)), Ok(64_i64));

        assert_eq!(from_value(BencodedValue::Integer(64)), Ok(64_i8));

        assert_eq!(from_value(BencodedValue::Integer(64)), Ok(64_u8));

        assert_eq!(from_bytes(b"i64e"), Ok(64_i64));
    }

    #[test]
    fn test_list() {
        // TODO: there is a bug with &str instead of String, should try and fix that!

        let hello_world = vec!["hello".to_owned(), "world".to_owned()];

        assert_eq!(
            from_value(BencodedValue::List(vec![
                BencodedValue::StringOwned("hello".to_owned()),
                BencodedValue::StringOwned("world".to_owned()),
            ])),
            Ok(hello_world.clone())
        );

        let bytes = b"l5:hello5:worlde";

        assert_eq!(from_bytes(bytes), Ok(hello_world));
    }

    #[test]
    fn test_dict() {
        let map = hashmap![
            "a".to_owned() => 1,
            "b".to_owned() => 2,
            "c".to_owned() => 3,
            "d".to_owned() => 4
        ];

        let value = BencodedValue::DictionaryOwned(
            map.iter()
                .map(|(k, v)| ((*k).to_owned(), BencodedValue::Integer(*v)))
                .collect(),
        );

        assert_eq!(from_value(value), Ok(map))
    }

    #[test]
    fn test_struct() {
        let test_data = TestStruct {
            name: "Tom".to_owned(),
            age: 24,
            friends: vec![
                "David".to_owned(),
                "Donald".to_owned(),
                "Barrack".to_owned(),
            ],
        };

        let encoded = BencodedValue::DictionaryOwned(hashmap! {
            "age".to_owned() => BencodedValue::Integer(
                24,
            ),
            "name".to_owned() => BencodedValue::StringOwned(
                "Tom".to_owned(),
            ),
            "friends".to_owned() => BencodedValue::List(
                vec![
                    BencodedValue::StringOwned(
                        "David".to_owned(),
                    ),
                    BencodedValue::StringOwned(
                        "Donald".to_owned(),
                    ),
                    BencodedValue::StringOwned(
                        "Barrack".to_owned(),
                    ),
                ],
            ),
        });

        if let Ok(value) = from_value::<TestStruct>(encoded) {
            assert_eq!(value, test_data);
        } else {
            assert!(false, "could not transform value");
        }
    }
}
