use crate::{
    error::{Error, Result},
    writer, BencodedValue,
};
use compound::Compound;
use serde::{ser, Serialize};
use std::{
    io::{self, Write},
    marker::PhantomData,
    mem::size_of,
};

mod compound;

#[derive(Default)]
pub(crate) struct Serializer<'se>(PhantomData<BencodedValue<'se>>);

/// Serializes a data structure into a byte vec
pub fn to_bytes<T>(value: &T) -> std::result::Result<Vec<u8>, io::Error>
where
    T: Serialize,
{
    let mut out = Vec::with_capacity(size_of::<T>());
    to_writer(value, &mut out)?;
    Ok(out)
}

/// Serializes a data structure into a writer
pub fn to_writer<T, W>(
    value: &T,
    writer: &mut W,
) -> std::result::Result<(), io::Error>
where
    T: Serialize,
    W: Write,
{
    writer::write(&to_value(value)?, writer)
}

/// Serializes a data structure into a BencodedValue
pub fn to_value<T>(
    value: &'_ T,
) -> std::result::Result<BencodedValue<'_>, io::Error>
where
    T: Serialize,
{
    value
        .serialize(Serializer::default())
        .map_err(Into::<io::Error>::into)
}

impl<'serializer> ser::Serializer for Serializer<'serializer> {
    type Ok = BencodedValue<'serializer>;
    type Error = Error;
    type SerializeSeq = Compound<'serializer>;
    type SerializeTuple = Compound<'serializer>;
    type SerializeTupleStruct = Compound<'serializer>;
    type SerializeTupleVariant = Compound<'serializer>;
    type SerializeMap = Compound<'serializer>;
    type SerializeStruct = Compound<'serializer>;
    type SerializeStructVariant = Compound<'serializer>;

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Ok(BencodedValue::Integer(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        if cfg!(test) {
            eprintln!("[bencode] casting char to string of length 1");
        }

        self.serialize_str(&format!("{}", v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(BencodedValue::StringOwned(v.to_owned()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        Ok(BencodedValue::BinaryOwned(v.to_vec()))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(BencodedValue::None)
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        if cfg!(test) {
            eprintln!(
                "[bencode] casting boolean to int (true => 1, false => 0)"
            );
        }

        self.serialize_i64(if v { 1 } else { 0 })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        if cfg!(test) {
            eprintln!("[bencode] rounding f32 to nearest int");
        }

        self.serialize_i64(v.round() as i64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        if cfg!(test) {
            eprintln!("[bencode] rounding f64 to nearest int");
        }

        self.serialize_i64(v.round() as i64)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<Self::Ok> {
        if cfg!(test) {
            eprintln!("[bencode] unit cannot be serialize");
        }

        Err(Error::Message("cannot serialize units".to_owned()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Ok(BencodedValue::Dictionary(maplit::hashmap! {
            variant => value.serialize(self)?
        }))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(Compound::new_array(len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(Compound::new_array(Some(len)))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(Compound::new_map(Some(len)))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(Compound::new_map(Some(0)))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(Compound::new_map(len))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        Ok(Compound::new_map(Some(len)))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(Compound::new_map(Some(0)))
    }
}

#[cfg(test)]
mod serialize_tests {
    use super::{to_bytes, to_value};
    use crate::BencodedValue;
    use maplit::hashmap;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestStruct {
        name: String,
        age: i64,
        friends: Vec<String>,
    }

    #[test]
    fn test_string() {
        let hello_world = "Hello, world!";
        assert_eq!(
            to_value(&hello_world).unwrap(),
            BencodedValue::StringOwned(hello_world.to_owned())
        );

        let bytes = to_bytes(&hello_world).unwrap();
        assert_eq!(bytes, b"13:Hello, world!");
    }

    #[test]
    fn test_number() {
        let num = 64;
        assert_eq!(to_value(&num).unwrap(), BencodedValue::Integer(num));

        let bytes = to_bytes(&num).unwrap();
        assert_eq!(bytes, b"i64e");
    }

    #[test]
    fn test_list() {
        let list = vec!["Hello", "World", "!"];
        assert_eq!(
            to_value(&list).unwrap(),
            BencodedValue::List(
                list.iter()
                    .map(|v| BencodedValue::StringOwned((*v).to_owned()))
                    .collect()
            )
        );

        let bytes = to_bytes(&list).unwrap();
        assert_eq!(bytes, b"l5:Hello5:World1:!e");
    }

    #[test]
    fn test_dict() {
        let map = hashmap![
            "a" => 1,
            "b" => 2,
            "c" => 3,
            "d" => 4
        ];

        assert_eq!(
            to_value(&map).unwrap(),
            BencodedValue::DictionaryOwned(
                map.iter()
                    .map(|(k, v)| ((*k).to_owned(), BencodedValue::Integer(*v)))
                    .collect()
            )
        );
    }

    #[test]
    fn test_struct() {
        let value = TestStruct {
            name: "Tom".to_owned(),
            age: 24,
            friends: vec![
                "David".to_owned(),
                "Donald".to_owned(),
                "Barrack".to_owned(),
            ],
        };

        if let Ok(value) = to_value(&value) {
            assert_eq!(
                value,
                BencodedValue::DictionaryOwned(hashmap! {
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
                },)
            )
        } else {
            assert!(false, "could not transform value");
        }
    }
}
