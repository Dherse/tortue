//! Parser module for bencoding
//! Provides functions to parse bencoded lists (arrays)

use super::{parse, parse_string, BencodedValue};
use nom::{
    character::complete::char,
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};
use std::collections::HashMap;

/// Nom parse compinator to parse a bencoded HashMap<&str, BencodedValue>
#[inline]
pub fn parse_dictionary<'a>(
    input: &'a [u8],
) -> IResult<&'a [u8], HashMap<&'a str, BencodedValue<'a>>> {
    delimited(char('d'), many0(pair(parse_string, parse)), char('e'))(input)
        .map(|(res, value)| (res, value.into_iter().collect()))
}

#[cfg(test)]
mod dictionary_tests {
    #[cfg(test)]
    extern crate maplit;

    use super::{super::BencodedValue, parse_dictionary};
    use maplit::hashmap;

    #[test]
    pub fn test_dict() {
        assert_eq!(parse_dictionary(b"de"), Ok((b"" as _, hashmap! {})));

        assert_eq!(
            parse_dictionary(b"d1:ai4ee"),
            Ok((
                b"" as _,
                hashmap! {
                    "a" => BencodedValue::Integer(4)
                }
            ))
        );

        assert_eq!(
            parse_dictionary(b"d1:ai4e1:b3:cowe"),
            Ok((
                b"" as _,
                hashmap! {
                    "a" => BencodedValue::Integer(4),
                    "b" => BencodedValue::String("cow")
                }
            ))
        );

        assert_eq!(
            parse_dictionary(b"d1:ai4e1:b3:cow1:cli1ei2ei3eee"),
            Ok((
                b"" as _,
                hashmap! {
                    "a" => BencodedValue::Integer(4),
                    "b" => BencodedValue::String("cow"),
                    "c" => BencodedValue::List(vec![
                        BencodedValue::Integer(1),
                        BencodedValue::Integer(2),
                        BencodedValue::Integer(3),
                    ])
                }
            ))
        );
    }
}
