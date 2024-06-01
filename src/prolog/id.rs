use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::alphanumeric1,
    combinator::map,
    multi::many0,
    sequence::{delimited, pair, preceded},
};

use crate::{parse::Parse, IResult};

use super::external_id::ExternalID;

#[derive(Clone, PartialEq, Eq)]
pub enum ID {
    ExternalID(ExternalID),
    PublicID(String),
}

impl<'a: 'static> Parse<'a> for ID {
    type Args = ();
    type Output = IResult<&'a str, Self>;
    // [83] PublicID ::= 'PUBLIC' S PubidLiteral
    fn parse(input: &'static str, _args: Self::Args) -> Self::Output {
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

impl ID {
    // [12] PubidLiteral ::= '"' PubidChar* '"' | "'" (PubidChar - "'")* "'"
    pub fn parse_public_id_literal(input: &'static str) -> IResult<&'static str, String> {
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
            |pubid_literal: Vec<&str>| pubid_literal.concat(),
        )(input)
    }

    // [13] PubidChar ::= #x20 | #xD | #xA | [a-zA-Z0-9] | [-'()+,./:=?;!*#@$_%]
    pub fn parse_pubid_char(
        input: &'static str,
        exclude_single_quote: bool,
    ) -> IResult<&'static str, &str> {
        let chars = if exclude_single_quote {
            " \r\n-()+,./:=?;!*#@$_%"
        } else {
            " \r\n-'()+,./:=?;!*#@$_%"
        };

        alt((tag(" "), tag("\r"), tag("\n"), alphanumeric1, is_a(chars)))(input)
    }
}
