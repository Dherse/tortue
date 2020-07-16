use super::Serializer;
use crate::{error::Error, BencodedValue};
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};
pub enum Compound<'data, 'serializer> {
    Map {
        ser: &'serializer mut Serializer<'data>,
        local_serializer: Serializer<'data>,
        keys: Vec<String>,
    },
    Array {
        ser: &'serializer mut Serializer<'data>,
        local_serializer: Serializer<'data>,
    },
}

impl<'data, 'serializer> Compound<'data, 'serializer> {
    pub fn new_array(
        ser: &'serializer mut Serializer<'data>,
        capacity_hint: Option<usize>,
    ) -> Self {
        Compound::Array {
            ser,
            local_serializer: Serializer {
                output: if let Some(hint) = capacity_hint {
                    Vec::with_capacity(hint)
                } else {
                    Vec::new()
                },
            },
        }
    }

    pub fn new_map(
        ser: &'serializer mut Serializer<'data>,
        capacity_hint: Option<usize>,
    ) -> Self {
        Compound::Map {
            ser,
            keys: if let Some(hint) = capacity_hint {
                Vec::with_capacity(hint)
            } else {
                Vec::new()
            },
            local_serializer: Serializer {
                output: if let Some(hint) = capacity_hint {
                    Vec::with_capacity(hint)
                } else {
                    Vec::new()
                },
            },
        }
    }
}

impl<'data, 'serializer> SerializeSeq for Compound<'data, 'serializer> {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        match *self {
            Compound::Array {
                ref mut local_serializer,
                ..
            } => {
                value.serialize(&mut *local_serializer)?;
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Compound::Array {
                ser,
                local_serializer,
                ..
            } => {
                ser.output
                    .push(BencodedValue::List(local_serializer.output));
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}

impl<'data, 'serializer> SerializeTuple for Compound<'data, 'serializer> {
    type Ok = ();
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

impl<'data, 'serializer> SerializeTupleStruct for Compound<'data, 'serializer> {
    type Ok = ();
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

impl<'data, 'serializer> SerializeTupleVariant
    for Compound<'data, 'serializer>
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        Err(Error::Message("Enum variants are not supported".to_owned()))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Message("Enum variants are not supported".to_owned()))
    }
}

impl<'data, 'serializer> SerializeStruct for Compound<'data, 'serializer> {
    type Ok = ();
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
            Compound::Map {
                ref mut local_serializer,
                ref mut keys,
                ..
            } => {
                keys.push(key.to_owned());
                value.serialize(&mut *local_serializer)?;
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Compound::Map {
                ser,
                local_serializer,
                keys,
            } => {
                ser.output.push(BencodedValue::DictionaryOwned(
                    keys.into_iter()
                        .zip(local_serializer.output.into_iter())
                        .collect(),
                ));
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}

impl<'data, 'serializer> SerializeMap for Compound<'data, 'serializer> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        match *self {
            Compound::Map {
                ref mut local_serializer,
                ref mut keys,
                ..
            } => {
                key.serialize(&mut *local_serializer)?;
                match local_serializer.output.pop().unwrap() {
                    BencodedValue::String(value) => keys.push(value.to_owned()),
                    BencodedValue::StringOwned(value) => keys.push(value),
                    _ => {
                        return Err(Error::Message(
                            "Only string keys are supported in maps".to_owned(),
                        ))
                    }
                }
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
        match *self {
            Compound::Map {
                ref mut local_serializer,
                ..
            } => value.serialize(local_serializer),
            _ => unreachable!(),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as SerializeStruct>::end(self)
    }
}

impl<'data, 'serializer> SerializeStructVariant
    for Compound<'data, 'serializer>
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        Err(Error::Message("Enum variants are not supported".to_owned()))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Message("Enum variants are not supported".to_owned()))
    }
}
