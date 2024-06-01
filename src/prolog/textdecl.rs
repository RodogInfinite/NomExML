use nom::{
    branch::alt,
    character::complete::{alpha1, alphanumeric1, digit1},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
};

use crate::{parse::Parse, tag, IResult};

#[derive(Clone, PartialEq, Eq)]
pub struct TextDecl {
    pub version: Option<String>,
    pub encoding: String,
}
impl<'a: 'static> Parse<'a> for TextDecl {
    type Args = ();
    type Output = IResult<&'a str, Self>;
    //[77] TextDecl ::='<?xml' VersionInfo? EncodingDecl S? '?>'
    fn parse(input: &'static str, _args: Self::Args) -> Self::Output {
        map(
            tuple((
                tag("<?xml"),
                opt(Self::parse_version_info),
                Self::parse_encoding_decl,
                Self::parse_multispace0,
                tag("?>"),
            )),
            |(_start, version, encoding, _whitespace, _end)| Self { version, encoding },
        )(input)
    }
}

impl TextDecl {
    // [24] VersionInfo	::= S 'version' Eq ("'" VersionNum "'" | '"' VersionNum '"')
    fn parse_version_info(input: &'static str) -> IResult<&'static str, String> {
        map(
            tuple((
                Self::parse_multispace1,
                tag("version"),
                Self::parse_eq,
                alt((
                    delimited(tag("'"), Self::parse_version_num, tag("'")),
                    delimited(tag("\""), Self::parse_version_num, tag("\"")),
                )),
            )),
            |(_whitespace, _version_literal, _eq, version)| version,
        )(input)
    }

    // [26] VersionNum	::= '1.' [0-9]+
    fn parse_version_num(input: &'static str) -> IResult<&'static str, String> {
        map(preceded(tag("1."), digit1), |version| {
            format!("1.{}", version)
        })(input)
    }
    // [80] EncodingDecl	::= S 'encoding' Eq ('"' EncName '"' | "'" EncName "'" )
    fn parse_encoding_decl(input: &'static str) -> IResult<&'static str, String> {
        map(
            tuple((
                Self::parse_multispace1,
                tag("encoding"),
                Self::parse_eq,
                alt((
                    delimited(tag("'"), Self::parse_enc_name, tag("'")),
                    delimited(tag("\""), Self::parse_enc_name, tag("\"")),
                )),
            )),
            |(_whitespace, _encoding_literal, _eq, encoding)| encoding,
        )(input)
    }

    // [81] EncName	::= [A-Za-z] ([A-Za-z0-9._] | '-')*
    fn parse_enc_name(input: &'static str) -> IResult<&'static str, String> {
        map(
            pair(
                alt((alpha1, tag("-"))),
                many0(alt((alphanumeric1, tag("."), tag("_"), tag("-")))),
            ),
            |(first, rest)| format!("{}{}", first, rest.join("")),
        )(input)
    }
}
