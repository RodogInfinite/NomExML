// reference.rs

use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1},
    combinator::map,
    sequence::delimited,
    IResult,
};

use crate::parse::Parse;
use crate::{
    decode::{decode_digit, decode_hex},
    Name,
};

#[derive(Clone, PartialEq)]
pub enum Reference<'a> {
    EntityRef(Name<'a>),
    CharRef {
        value: Cow<'a, str>,
        state: CharRefState,
    },
}

impl<'a> Parse<'a> for Reference<'a> {
    //[67] Reference ::= EntityRef | CharRef
    fn parse(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        alt((Self::parse_entity_ref, Self::parse_char_reference))(input)
    }
}

impl<'a> Reference<'a> {
    pub fn normalize(&self) -> Cow<'a, str> {
        match self {
            Reference::EntityRef(name) => Cow::Owned(name.local_part.to_string()),
            Reference::CharRef { value, .. } => Cow::Owned(value.to_string()),
        }
    }
}

impl<'a> ParseReference<'a> for Reference<'a> {}

#[derive(Clone, PartialEq)]
pub enum CharRefState {
    Decimal,
    Hexadecimal,
}

pub trait ParseReference<'a>: Parse<'a> {
    //[68] EntityRef ::= '&' Name ';'
    fn parse_entity_ref(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        println!("\n-----\nPARSING ENTITY REFERENCE");
        let (input, _) = tag("&")(input)?;
        let (input, name) = Self::parse_name(input)?;
        let (input, _) = tag(";")(input)?;
        Ok((input, Reference::EntityRef(name)))
    }

    //[69] PEReference ::= '%' Name ';'
    fn parse_parameter_reference(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        let (input, _) = tag("%")(input)?;
        let (input, name) = Self::parse_name(input)?;
        let (input, _) = tag(";")(input)?;
        Ok((input, Reference::EntityRef(name)))
    }

    //[66] CharRef ::= '&#' [0-9]+ ';' | '&#x' [0-9a-fA-F]+ ';'
    fn parse_char_reference(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        println!("\n-----\nPARSING CHAR REFERENCE");
        let (input, char_ref) = alt((
            map(
                delimited(tag("&#"), digit1, tag(";")),
                |digits_str: &str| {
                    let (_, decoded) = decode_digit("", digits_str).unwrap();
                    Reference::CharRef {
                        value: Cow::Owned(decoded.into_owned()),
                        state: CharRefState::Decimal,
                    }
                },
            ),
            map(
                delimited(tag("&#x"), hex_digit1, tag(";")),
                |hex_str: &str| {
                    let (_, decoded) = decode_hex("", hex_str).unwrap();
                    Reference::CharRef {
                        value: Cow::Owned(decoded.into_owned()),
                        state: CharRefState::Hexadecimal,
                    }
                },
            ),
        ))(input)?;
        println!("PARSED CHAR REFERENCE: {char_ref:?}");
        Ok((input, char_ref))
    }
}
