// reference.rs

use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1},
    combinator::map,
    multi::many1,
    sequence::delimited,
    IResult,
};

use crate::decode::{decode_digit, decode_hex};
use crate::parse::Parse;

#[derive(Clone, PartialEq)]
pub enum Reference<'a> {
    EntityRef(Cow<'a, str>),
    CharRef {
        value: Cow<'a, str>,
        state: CharRefState,
    },
}

impl<'a> Parse<'a> for Reference<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        alt((Self::parse_entity_ref, Self::parse_char_reference))(input)
    }
}

impl<'a> ParseReference<'a> for Reference<'a> {}

#[derive(Clone, PartialEq)]
pub enum CharRefState {
    Decimal,
    Hexadecimal,
}

pub trait ParseReference<'a>: Parse<'a> {
    fn parse_entity_ref(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        let (input, _) = tag("&")(input)?;
        let (input, name) = Self::parse_name(input)?;
        let (input, _) = tag(";")(input)?;
        Ok((input, Reference::EntityRef(name)))
    }

    fn parse_parameter_reference(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        let (input, _) = tag("%")(input)?;
        let (input, name) = Self::parse_name(input)?;
        let (input, _) = tag(";")(input)?;
        Ok((input, Reference::EntityRef(name)))
    }

    fn parse_char_reference(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        alt((
            map(
                delimited(tag("&#"), many1(digit1), tag(";")),
                |digits_vec: Vec<&str>| {
                    let digits_str = digits_vec.concat();
                    let (_, decoded) = decode_digit("", &digits_str).unwrap();
                    Reference::CharRef {
                        value: Cow::Owned(decoded.into_owned()),
                        state: CharRefState::Decimal,
                    }
                },
            ),
            map(
                delimited(tag("&#x"), many1(hex_digit1), tag(";")),
                |hex_vec: Vec<&str>| {
                    let hex_str = hex_vec.concat();
                    let (_, decoded) = decode_hex("", &hex_str).unwrap();
                    Reference::CharRef {
                        value: Cow::Owned(decoded.into_owned()),
                        state: CharRefState::Hexadecimal,
                    }
                },
            ),
        ))(input)
    }
}
