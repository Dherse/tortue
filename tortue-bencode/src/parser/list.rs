//! Parser module for bencoding
//! Provides functions to parse bencoded lists (arrays)

use super::{parse_all_no_group, BencodedValue};
use nom::{
    bytes::complete::tag, combinator::map, sequence::delimited, IResult,
};

/// Nom parse compinator to parse a bencoded Vec<BencodedValue>
#[inline]
pub fn parse_list<'a>(input: &'a [u8]) -> IResult<&'a [u8], BencodedValue<'a>> {
    delimited(
        tag("l"),
        map(parse_all_no_group, BencodedValue::List),
        tag("e"),
    )(input)
}

#[cfg(test)]
mod list_tests {
    use super::{super::BencodedValue, parse_list};
    use nom::{error::ErrorKind, Err::Error};

    #[test]
    pub fn test_list() {
        assert_eq!(
            parse_list(b"le"),
            Ok((b"" as _, BencodedValue::List(vec![])))
        );

        assert_eq!(
            parse_list(b"li3ei4ee"),
            Ok((
                b"" as _,
                BencodedValue::List(vec![
                    BencodedValue::Integer(3),
                    BencodedValue::Integer(4)
                ])
            ))
        );

        assert_eq!(
            parse_list(b"li3ei4e4:abcde"),
            Ok((
                b"" as _,
                BencodedValue::List(vec![
                    BencodedValue::Integer(3),
                    BencodedValue::Integer(4),
                    BencodedValue::String("abcd")
                ])
            ))
        );

        assert_eq!(
            parse_list(b"lli5eee"),
            Ok((
                b"" as _,
                BencodedValue::List(vec![BencodedValue::List(vec![
                    BencodedValue::Integer(5)
                ])])
            ))
        );

        assert_eq!(
            parse_list(b"li3ei4eli5eee"),
            Ok((
                b"" as _,
                BencodedValue::List(vec![
                    BencodedValue::Integer(3),
                    BencodedValue::Integer(4),
                    BencodedValue::List(vec![BencodedValue::Integer(5)])
                ])
            ))
        );

        assert_eq!(parse_list(b"li3e"), Err(Error((b"" as _, ErrorKind::Tag))));

        assert_eq!(parse_list(b"l"), Err(Error((b"" as _, ErrorKind::Tag))));

        assert_eq!(
            parse_list(b"labc"),
            Err(Error((b"abc" as _, ErrorKind::Tag)))
        );

        assert_eq!(
            parse_list(b"labce"),
            Err(Error((b"abce" as _, ErrorKind::Tag)))
        );
    }
}
