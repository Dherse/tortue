use crate::{error::Error, BencodedValue};
use serde::de;

pub struct SeqAccess<'re> {
    len: usize,
    index: usize,
    values: Vec<BencodedValue<'re>>,
}

impl<'re> SeqAccess<'re> {
    pub fn new(values: Vec<BencodedValue<'re>>) -> Self {
        SeqAccess {
            index: 0,
            len: values.len(),
            values,
        }
    }
}

impl<'de> de::SeqAccess<'de> for SeqAccess<'de> {
    type Error = Error;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.len <= self.index {
            Ok(None)
        } else {
            self.index += 1;
            let mut deser =
                super::Deserializer::from_value(self.values.remove(0))?;
            let out = seed.deserialize(&mut deser).map(Some)?;

            Ok(out)
        }
    }
}
