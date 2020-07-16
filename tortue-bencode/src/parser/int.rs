//! Parser module for bencoding
//! Provides functions to parse bencoded ints

use super::parse_utf8_str;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::is_digit,
    combinator::map_res,
    sequence::delimited,
    IResult,
};
use std::num::ParseIntError;

/// Parse a base 10 number from an input string
#[inline]
pub fn parse_base10(input: &str) -> Result<i64, ParseIntError> {
    i64::from_str_radix(input, 10)
}

/// Parse a base 10 encoded i64
#[inline]
pub fn base10_primary<'a>(input: &'a [u8]) -> IResult<&'a [u8], i64> {
    map_res(
        take_while_m_n(1, 20, |d| is_digit(d) || d == b'-'),
        map_res(parse_utf8_str, parse_base10),
    )(input)
    .map(|(r, (_res, v))| (r, v))
}

/// Nom parse compinator to parse a bencoded i64
#[inline]
pub fn parse_int<'a>(input: &'a [u8]) -> IResult<&'a [u8], i64> {
    delimited(tag("i"), base10_primary, tag("e"))(input)
}

#[cfg(test)]
mod string_tests {
    use super::parse_int;
    use nom::{error::ErrorKind, Err::Error};

    #[test]
    pub fn test_int() {
        assert_eq!(parse_int(b"i3e"), Ok((b"" as &_, 3)));
        assert_eq!(parse_int(b"i3eabc"), Ok((b"abc" as &_, 3)));
        assert_eq!(parse_int(b"i-3e"), Ok((b"" as &_, -3)));
        assert_eq!(parse_int(b"i-3eabc"), Ok((b"abc" as &_, -3)));
        assert_eq!(parse_int(b"i1234567890e"), Ok((b"" as &_, 1234567890)));
        assert_eq!(
            parse_int(b"i1234567890eabc"),
            Ok((b"abc" as &_, 1234567890))
        );
        assert_eq!(parse_int(b"i-1234567890e"), Ok((b"" as &_, -1234567890)));
        assert_eq!(
            parse_int(b"i-1234567890eabc"),
            Ok((b"abc" as &_, -1234567890))
        );

        assert_eq!(parse_int(b"i3"), Err(Error((b"" as &_, ErrorKind::Tag))));

        assert_eq!(parse_int(b"3e"), Err(Error((b"3e" as &_, ErrorKind::Tag))));

        assert_eq!(
            parse_int(b"ie"),
            Err(Error((b"e" as &_, ErrorKind::TakeWhileMN)))
        );
    }
}
