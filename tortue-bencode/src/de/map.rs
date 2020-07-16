use crate::{error::Error, BencodedValue};
use serde::de;
use std::collections::HashMap;

pub struct MapAccess<'re> {
    len: usize,
    index: usize,
    values: Vec<(String, BencodedValue<'re>)>,
    current_value: Option<BencodedValue<'re>>,
}

impl<'re> MapAccess<'re> {
    pub fn new(values: HashMap<String, BencodedValue<'re>>) -> Self {
        MapAccess {
            index: 0,
            len: values.len(),
            values: values.into_iter().collect(),
            current_value: None,
        }
    }
}

impl<'de, 'da> de::MapAccess<'de> for MapAccess<'de> {
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

            let (key, value) = self.values.remove(0);

            self.current_value = Some(value);

            let mut deser = super::Deserializer::from_value(
                BencodedValue::StringOwned(key),
            )?;
            let out = seed.deserialize(&mut deser).map(|v| Some(v))?;

            Ok(out)
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
            let mut deser = super::Deserializer::from_value(
                self.current_value.take().unwrap(),
            )?;
            seed.deserialize(&mut deser)
        }
    }
}
