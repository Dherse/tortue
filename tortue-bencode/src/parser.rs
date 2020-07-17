//! Parser module for bencoding
//! Provides functions to parse bencoded values

mod bytes;
mod dictionary;
mod int;
mod list;
mod string;

pub use self::{bytes::*, dictionary::*, int::*, list::*, string::*};

use nom::{
    branch::alt,
    combinator::{all_consuming, iterator, map},
    IResult,
};

pub use crate::BencodedValue;

/// Parses an input string and returns a BencodedValue
#[inline]
pub fn parse<'a>(input: &'a [u8]) -> IResult<&'a [u8], BencodedValue<'a>> {
    alt((
        map(string::parse_string, BencodedValue::String),
        map(bytes::parse_bytes, BencodedValue::Binary),
        map(int::parse_int, BencodedValue::Integer),
        list::parse_list,
        map(dictionary::parse_dictionary, BencodedValue::Dictionary),
    ))(input)
}

/// Parses an input string and returns a Vec<BencodedValue>, fails if the string is not fully consummed
#[inline]
pub fn parse_all<'a>(input: &'a [u8]) -> IResult<&'a [u8], BencodedValue<'a>> {
    all_consuming(parse_all_incomplete)(input)
}

/// Parses an input string and returns a grouped BencodedValue, does **not** fail if the string is not fully consummed
#[inline]
pub fn parse_all_incomplete<'a>(
    input: &'a [u8],
) -> IResult<&'a [u8], BencodedValue<'a>> {
    let mut iter = iterator(input, parse);
    let values = iter.collect::<Vec<_>>();
    let (rest, _) = iter.finish()?;
    if values.is_empty() {
        Ok((rest, BencodedValue::None))
    } else if values.len() == 1 {
        Ok((rest, values.into_iter().next().unwrap()))
    } else {
        Ok((rest, BencodedValue::List(values)))
    }
}

/// Parses an input string and returns a Vec<BencodedValue>, does **not** fail if the string is not fully consummed
#[inline]
pub fn parse_all_no_group<'a>(
    input: &'a [u8],
) -> IResult<&'a [u8], Vec<BencodedValue<'a>>> {
    let mut iter = iterator(input, parse);
    let values = iter.collect::<Vec<_>>();
    let (rest, _) = iter.finish()?;
    Ok((rest, values))
}

#[cfg(test)]
mod parse_tests {
    use super::{parse, parse_all, BencodedValue};
    use nom::{
        error::ErrorKind,
        Err::{Error, Incomplete},
        Needed,
    };

    #[test]
    pub fn test_all() {
        assert_eq!(
            parse_all(b"i3ei4e"),
            Ok((
                b"" as _,
                BencodedValue::List(vec![
                    BencodedValue::Integer(3),
                    BencodedValue::Integer(4)
                ])
            ))
        );

        assert_eq!(
            parse_all(b"i3ei4eabc"),
            Err(Error((b"abc" as _, ErrorKind::Eof)))
        );

        assert_eq!(
            parse_all(b"i3e4:abcd"),
            Ok((
                b"" as _,
                BencodedValue::List(vec![
                    BencodedValue::Integer(3),
                    BencodedValue::String("abcd")
                ])
            ))
        );

        assert_eq!(
            parse_all(b"i3e4:abcdefg"),
            Err(Error((b"efg" as _, ErrorKind::Eof)))
        );
    }

    #[test]
    pub fn test_int() {
        assert_eq!(parse(b"i3e"), Ok((b"" as _, BencodedValue::Integer(3))));

        assert_eq!(
            parse(b"i3eabc"),
            Ok((b"abc" as _, BencodedValue::Integer(3)))
        );

        assert_eq!(parse(b"i-3e"), Ok((b"" as _, BencodedValue::Integer(-3))));

        assert_eq!(
            parse(b"i-3eabc"),
            Ok((b"abc" as _, BencodedValue::Integer(-3)))
        );

        assert_eq!(
            parse(b"i1234567890e"),
            Ok((b"" as _, BencodedValue::Integer(1234567890)))
        );

        assert_eq!(
            parse(b"i1234567890eabc"),
            Ok((b"abc" as _, BencodedValue::Integer(1234567890)))
        );

        assert_eq!(
            parse(b"i-1234567890e"),
            Ok((b"" as _, BencodedValue::Integer(-1234567890)))
        );

        assert_eq!(
            parse(b"i3"),
            Err(Error((b"i3" as _, ErrorKind::TakeWhileMN)))
        );

        assert_eq!(
            parse(b"ie"),
            Err(Error((b"ie" as _, ErrorKind::TakeWhileMN)))
        );
    }

    #[test]
    pub fn test_string() {
        assert_eq!(parse(b"3e"), Err(Incomplete(Needed::Size(4))));

        assert_eq!(
            parse(b"3:abc"),
            Ok((b"" as _, BencodedValue::String("abc")))
        );

        assert_eq!(
            parse(b"3:abcdef"),
            Ok((b"def" as _, BencodedValue::String("abc")))
        );

        assert_eq!(parse(b"0:"), Ok((b"" as _, BencodedValue::String(""))));

        assert_eq!(
            parse(b"0:abc"),
            Ok((b"abc" as _, BencodedValue::String("")))
        );

        assert_eq!(
            parse(b"e:"),
            Err(Error((b"e:" as _, ErrorKind::TakeWhileMN)))
        );

        // This is actually parsed by string/bytes
        assert_eq!(parse(b"3abcd"), Err(Error((b"abcd" as _, ErrorKind::Tag))));

        assert_eq!(parse(b"3:ab"), Err(Incomplete(Needed::Size(4))));
    }

    #[test]
    pub fn test_bytes() {
        assert_eq!(parse(b"3e"), Err(Incomplete(Needed::Size(4))));

        assert_eq!(
            parse(b"3:ab\xFF"),
            Ok((b"" as _, BencodedValue::Binary(b"ab\xFF" as _)))
        );
    }
}
