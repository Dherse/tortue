//! Parser module for bencoding
//! Provides functions to parse bencoded strings

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::is_digit,
    combinator::map_res,
    error::ErrorKind,
    multi::length_value,
    sequence::preceded,
    IResult,
};
use std::num::ParseIntError;

/// Parse a base 10 number from an input string
#[inline]
pub fn parse_base10(input: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(input, 10)
}

pub fn parse_utf8_str<'a>(input: &'a [u8]) -> IResult<&'a [u8], &'a str> {
    std::str::from_utf8(input)
        .map(|v| (&[] as &'a [u8], v as &'a str))
        .map_err(|_e| nom::Err::Error((input, ErrorKind::Verify)))
}

/// Parse a base 10 encoded u32
#[inline]
pub fn base10_length(input: &[u8]) -> IResult<&[u8], u32> {
    map_res(
        take_while_m_n(1, 20, is_digit),
        map_res(parse_utf8_str, parse_base10),
    )(input)
    .map(|(r, (_res, v))| (r, v + 1))
}

/// Nom parse compinator to parse a bencoded string
#[inline]
pub fn parse_string<'a>(input: &'a [u8]) -> IResult<&'a [u8], &'a str> {
    length_value(base10_length, preceded(tag(":"), parse_utf8_str))(input)
}

#[cfg(test)]
mod string_tests {
    use super::parse_string;
    use nom::{
        error::ErrorKind,
        Err::{Error, Incomplete},
        Needed,
    };

    #[test]
    pub fn test_str() {
        assert_eq!(parse_string(b"3:abc"), Ok((b"" as &_, "abc")));
        assert_eq!(parse_string(b"3:abcdef"), Ok((b"def" as &_, "abc")));
        assert_eq!(parse_string(b"0:"), Ok((b"" as &_, "")));
        assert_eq!(parse_string(b"0:abc"), Ok((b"abc" as &_, "")));

        assert_eq!(
            parse_string(b"e:"),
            Err(Error((b"e:" as &_, ErrorKind::TakeWhileMN)))
        );
        assert_eq!(
            parse_string(b"3abcd"),
            Err(Error((b"abcd" as &_, ErrorKind::Tag)))
        );
        assert_eq!(parse_string(b"3:ab"), Err(Incomplete(Needed::Size(4))));
    }
}
