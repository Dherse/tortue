//! Parser module for bencoding
//! Provides functions to parse bencoded bytes

use super::base10_length;
use nom::{
    bytes::complete::tag, multi::length_value, sequence::preceded, IResult,
};

/// Nom parse compinator to parse a bencoded string
#[inline]
pub fn parse_bytes<'a>(input: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    length_value(
        base10_length,
        preceded(tag(":"), |input| Ok((&[] as &[u8], input))),
    )(input)
}

#[cfg(test)]
mod string_tests {
    use super::parse_bytes;
    use nom::{
        error::ErrorKind,
        Err::{Error, Incomplete},
        Needed,
    };

    #[test]
    pub fn test_str() {
        assert_eq!(parse_bytes(b"3:abc"), Ok((b"" as &_, b"abc" as &_)));
        assert_eq!(parse_bytes(b"3:abcdef"), Ok((b"def" as &_, b"abc" as &_)));
        assert_eq!(
            parse_bytes(b"4:abc\x3C"),
            Ok((b"" as &_, b"abc\x3C" as &_))
        );
        assert_eq!(
            parse_bytes(b"4:abc\x3Cdef"),
            Ok((b"def" as &_, b"abc\x3C" as &_))
        );
        assert_eq!(parse_bytes(b"0:"), Ok((b"" as &_, b"" as &_)));
        assert_eq!(
            parse_bytes(b"0:abc\xFF"),
            Ok((b"abc\xFF" as &_, b"" as &_))
        );

        assert_eq!(
            parse_bytes(b"e:"),
            Err(Error((b"e:" as &_, ErrorKind::TakeWhileMN)))
        );
        assert_eq!(
            parse_bytes(b"3abcd"),
            Err(Error((b"abcd" as &_, ErrorKind::Tag)))
        );

        assert_eq!(parse_bytes(b"3:ab"), Err(Incomplete(Needed::Size(4))));
    }
}
