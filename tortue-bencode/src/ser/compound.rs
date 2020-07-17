use super::Serializer;
use crate::{error::Error, BencodedValue};
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};
use std::collections::HashMap;
pub(crate) enum Compound<'se> {
    Map {
        current_key: Option<String>,
        values: HashMap<String, BencodedValue<'se>>,
    },
    Array {
        values: Vec<BencodedValue<'se>>,
    },
}

impl<'serializer> Compound<'serializer> {
    pub fn new_array(capacity_hint: Option<usize>) -> Self {
        Compound::Array {
            values: if let Some(hint) = capacity_hint {
                Vec::with_capacity(hint)
            } else {
                Vec::new()
            },
        }
    }

    pub fn new_map(capacity_hint: Option<usize>) -> Self {
        Compound::Map {
            current_key: None,
            values: if let Some(hint) = capacity_hint {
                HashMap::with_capacity(hint)
            } else {
                HashMap::new()
            },
        }
    }
}

impl<'serializer> SerializeSeq for Compound<'serializer> {
    type Ok = BencodedValue<'serializer>;
    type Error = Error;
    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        match self {
            Compound::Array { values, .. } => {
                values.push(value.serialize(Serializer::default())?);
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Compound::Array { values, .. } => Ok(BencodedValue::List(values)),
            _ => unreachable!(),
        }
    }
}

impl<'serializer> SerializeTuple for Compound<'serializer> {
    type Ok = BencodedValue<'serializer>;
    type Error = Error;

    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as SerializeSeq>::end(self)
    }
}

impl<'serializer> SerializeTupleStruct for Compound<'serializer> {
    type Ok = BencodedValue<'serializer>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as SerializeSeq>::end(self)
    }
}

impl<'serializer> SerializeTupleVariant for Compound<'serializer> {
    type Ok = BencodedValue<'serializer>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as SerializeSeq>::end(self)
    }
}

impl<'serializer> SerializeStruct for Compound<'serializer> {
    type Ok = BencodedValue<'serializer>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        match *self {
            Compound::Map { ref mut values, .. } => {
                //keys.push(key.to_owned());
                values.insert(
                    key.to_owned(),
                    value.serialize(Serializer::default())?,
                );
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Compound::Map { values, .. } => {
                Ok(BencodedValue::DictionaryOwned(values))
            }
            _ => unreachable!(),
        }
    }
}

impl<'serializer> SerializeMap for Compound<'serializer> {
    type Ok = BencodedValue<'serializer>;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        match *self {
            Compound::Map {
                ref mut current_key,
                ..
            } => {
                match key.serialize(Serializer::default())? {
                    BencodedValue::String(value) => {
                        current_key.replace(value.to_owned())
                    }
                    BencodedValue::StringOwned(value) => {
                        current_key.replace(value)
                    }
                    _ => {
                        return Err(Error::Message(
                            "Only string keys are supported in maps".to_owned(),
                        ))
                    }
                };
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn serialize_value<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        match self {
            Compound::Map {
                current_key,
                values,
                ..
            } => {
                values.insert(
                    current_key.take().unwrap(),
                    value.serialize(Serializer::default())?,
                );
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as SerializeStruct>::end(self)
    }
}

impl<'serializer> SerializeStructVariant for Compound<'serializer> {
    type Ok = BencodedValue<'serializer>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        <Self as SerializeStruct>::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as SerializeStruct>::end(self)
    }
}
