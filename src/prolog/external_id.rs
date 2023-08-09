use super::internal_subset::ID;
use crate::parse::Parse;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    sequence::delimited,
    IResult,
};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub enum ExternalID<'a> {
    System(Cow<'a, str>),
    Public {
        pubid: Cow<'a, str>,
        system_identifier: Box<ExternalID<'a>>, // Box<ExternalID::System>
    },
}

impl<'a> Parse<'a> for ExternalID<'a> {
    type Args = ();
    type Output = IResult<&'a str, Self>;
    //[75] ExternalID ::= 'SYSTEM' S SystemLiteral | 'PUBLIC' S PubidLiteral S SystemLiteral
    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
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
        let (input, pubid_literal) = ID::parse_public_id_literal(input)?;
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
}
