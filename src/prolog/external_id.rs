use crate::parse::Parse;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};
use std::borrow::Cow;

use super::id::ID;

#[derive(Clone, Debug, PartialEq)]
pub enum ExternalID {
    System(String),
    Public {
        pubid: String,
        system_identifier: Box<ExternalID>, // Box<ExternalID::System>
    },
}

impl<'a> Parse<'a> for ExternalID {
    type Args = ();
    type Output = IResult<&'a str, Self>;
    //[75] ExternalID ::= 'SYSTEM' S SystemLiteral | 'PUBLIC' S PubidLiteral S SystemLiteral
    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
        alt((Self::parse_system, Self::parse_public))(input)
    }
}

impl ExternalID {
    fn parse_system(input: &str) -> IResult<&str, ExternalID> {
        map(
            tuple((
                tag("SYSTEM"),
                Self::parse_multispace1,
                Self::parse_system_literal,
            )),
            |(_system, _whitespace, system_literal)| ExternalID::System(system_literal),
        )(input)
    }

    fn parse_public(input: &str) -> IResult<&str, ExternalID> {
        map(
            tuple((
                tag("PUBLIC"),
                Self::parse_multispace1,
                ID::parse_public_id_literal,
                Self::parse_multispace1,
                Self::parse_system_literal,
            )),
            |(_public, _whitespace1, pubid_literal, _whitespace2, system_literal)| {
                ExternalID::Public {
                    pubid: pubid_literal,
                    system_identifier: Box::new(ExternalID::System(system_literal)),
                }
            },
        )(input)
    }

    // [11] SystemLiteral ::= ('"' [^"]* '"') | ("'" [^']* "'")
    fn parse_system_literal(input: &str) -> IResult<&str, String> {
        map(
            alt((
                delimited(tag("\""), is_not("\""), tag("\"")),
                delimited(tag("'"), is_not("'"), tag("'")),
            )),
            |s: &str| s.to_string(),
        )(input)
    }
}
