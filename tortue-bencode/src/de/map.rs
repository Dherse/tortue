use crate::{error::Error, BencodedValue};
use serde::de;
use std::collections::{hash_map::IntoIter, HashMap};

pub struct MapAccess<'re, KeyType> {
    len: usize,
    index: usize,
    values: IntoIter<KeyType, BencodedValue<'re>>,
    current_value: Option<BencodedValue<'re>>,
}

impl<'re, KeyType> MapAccess<'re, KeyType> {
    pub fn new(values: HashMap<KeyType, BencodedValue<'re>>) -> Self {
        MapAccess {
            index: 0,
            len: values.len(),
            values: values.into_iter(),
            current_value: None,
        }
    }
}

impl<'de> de::MapAccess<'de> for MapAccess<'de, &'de str> {
    type Error = Error;

    fn next_key_seed<K>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.len <= self.index {
            Ok(None)
        } else {
            if self.current_value.is_some() {
                self.index += 1;
            }

            let (key, value) = self.values.next().unwrap();

            self.current_value = Some(value);

            let deser =
                super::Deserializer::from_value(BencodedValue::String(key));

            Ok(seed.deserialize(deser).map(Some)?)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        if self.len <= self.index {
            panic!("overflow")
        } else {
            self.index += 1;
            let deser = super::Deserializer::from_value(
                self.current_value.take().unwrap(),
            );
            seed.deserialize(deser)
        }
    }
}

impl<'de> de::MapAccess<'de> for MapAccess<'de, String> {
    type Error = Error;

    fn next_key_seed<K>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.len <= self.index {
            Ok(None)
        } else {
            if self.current_value.is_some() {
                self.index += 1;
            }

            let (key, value) = self.values.next().unwrap();

            self.current_value = Some(value);

            let deser = super::Deserializer::from_value(
                BencodedValue::StringOwned(key),
            );

            Ok(seed.deserialize(deser).map(Some)?)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        if self.len <= self.index {
            panic!("overflow")
        } else {
            self.index += 1;
            let deser = super::Deserializer::from_value(
                self.current_value.take().unwrap(),
            );
            seed.deserialize(deser)
        }
    }
}
