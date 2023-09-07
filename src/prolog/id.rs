use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::alphanumeric1,
    combinator::map,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded},
    IResult,
};

use crate::parse::Parse;

use super::external_id::ExternalID;

#[derive(Clone, PartialEq)]
pub enum ID<'a> {
    ExternalID(ExternalID<'a>),
    PublicID(Cow<'a, str>),
}

impl<'a> Parse<'a> for ID<'a> {
    type Args = ();
    type Output = IResult<&'a str, Self>;
    // [83] PublicID ::= 'PUBLIC' S PubidLiteral
    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
        alt((
            map(
                preceded(
                    pair(tag("PUBLIC"), Self::parse_multispace1),
                    Self::parse_public_id_literal,
                ),
                ID::PublicID,
            ),
            map(|i| ExternalID::parse(i, ()), ID::ExternalID),
        ))(input)
    }
}

impl<'a> ID<'a> {
    // [12] PubidLiteral ::= '"' PubidChar* '"' | "'" (PubidChar - "'")* "'"
    pub fn parse_public_id_literal(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        map(
            alt((
                delimited(
                    tag("\""),
                    many0(|i| Self::parse_pubid_char(i, false)),
                    tag("\""),
                ),
                delimited(
                    tag("'"),
                    many0(|i| Self::parse_pubid_char(i, true)),
                    tag("'"),
                ),
            )),
            |pubid_literal: Vec<&'a str>| Cow::Owned(pubid_literal.concat()),
        )(input)
    }

    // [13] PubidChar ::= #x20 | #xD | #xA | [a-zA-Z0-9] | [-'()+,./:=?;!*#@$_%]
    pub fn parse_pubid_char(
        input: &'a str,
        exclude_single_quote: bool,
    ) -> IResult<&'a str, &'a str> {
        let chars = if exclude_single_quote {
            " \r\n-()+,./:=?;!*#@$_%"
        } else {
            " \r\n-'()+,./:=?;!*#@$_%"
        };

        alt((tag(" "), tag("\r"), tag("\n"), alphanumeric1, is_a(chars)))(input)
    }
}
