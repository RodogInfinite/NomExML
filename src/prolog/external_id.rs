use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag},
    character::complete::alphanumeric1,
    multi::many1,
    sequence::delimited,
    IResult,
};
use std::borrow::Cow;

use crate::parse::Parse;

#[derive(Clone, Debug, PartialEq)]
pub enum ExternalID<'a> {
    System(Cow<'a, str>),
    Public {
        pubid: Cow<'a, str>,
        system_identifier: Box<ExternalID<'a>>, // Box<ExternalID::System>
    },
}

impl<'a> Parse<'a> for ExternalID<'a> {
    //[75] ExternalID ::= 'SYSTEM' S SystemLiteral | 'PUBLIC' S PubidLiteral S SystemLiteral
    fn parse(input: &'a str) -> IResult<&'a str, ExternalID<'a>> {
        alt((Self::parse_system, Self::parse_public))(input)
    }
}

impl<'a> ExternalID<'a> {
    fn parse_system(input: &'a str) -> IResult<&'a str, ExternalID<'a>> {
        let (input, _) = tag("SYSTEM")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, system_literal) = Self::parse_system_literal(input)?;
        Ok((input, ExternalID::System(system_literal)))
    }

    fn parse_public(input: &'a str) -> IResult<&'a str, ExternalID<'a>> {
        let (input, _) = tag("PUBLIC")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, pubid_literal) = Self::parse_public_id_literal(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, system_literal) = Self::parse_system_literal(input)?;
        Ok((
            input,
            ExternalID::Public {
                pubid: pubid_literal,
                system_identifier: Box::new(ExternalID::System(system_literal)),
            },
        ))
    }

    // [11] SystemLiteral ::= ('"' [^"]* '"') | ("'" [^']* "'")
    fn parse_system_literal(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, system_literal) = alt((
            delimited(tag("\""), is_not("\""), tag("\"")),
            delimited(tag("'"), is_not("'"), tag("'")),
        ))(input)?;
        Ok((input, Cow::Borrowed(system_literal)))
    }

    // [12] PubidLiteral ::= '"' PubidChar* '"' | "'" (PubidChar - "'")* "'"
    fn parse_public_id_literal(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, pubid_literal) = alt((
            delimited(tag("\""), many1(Self::parse_pubid_char), tag("\"")),
            delimited(tag("'"), many1(Self::parse_pubid_char), tag("'")),
        ))(input)?;
        Ok((input, Cow::Owned(pubid_literal.join(""))))
    }

    // [13] PubidChar ::= #x20 | #xD | #xA | [a-zA-Z0-9] | [-'()+,./:=?;!*#@$_%]
    fn parse_pubid_char(input: &'a str) -> IResult<&'a str, &'a str> {
        alt((alphanumeric1, is_a(" \r\n-'()+,./:=?;!*#@$_%")))(input)
    }
}
