use crate::parse::Parse;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit1},
    combinator::{map, opt},
    error::ErrorKind,
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use std::{borrow::Cow, str::FromStr};

#[derive(Clone, PartialEq)]
pub enum Standalone {
    Yes,
    No,
}

impl FromStr for Standalone {
    type Err = nom::Err<nom::error::Error<&'static str>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yes" => Ok(Standalone::Yes),
            "no" => Ok(Standalone::No),
            _ => Err(nom::Err::Error(nom::error::Error::new("", ErrorKind::Alt))),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct XmlDecl<'a> {
    pub version: Cow<'a, str>,
    pub encoding: Option<Cow<'a, str>>,
    pub standalone: Option<Standalone>,
}
impl<'a> Parse<'a> for XmlDecl<'a> {
    type Args = ();
    type Output = IResult<&'a str, Self>;
    // [23] XMLDecl	::=  '<?xml' VersionInfo EncodingDecl? SDDecl? S? '?>'
    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
        map(
            tuple((
                tag("<?xml"),
                Self::parse_version_info,
                opt(Self::parse_encoding_decl),
                opt(Self::parse_sd_decl),
                Self::parse_multispace0,
                tag("?>"),
            )),
            |(_start, version, encoding, standalone, _whitespace, _end)| Self {
                version,
                encoding,
                standalone,
            },
        )(input)
    }
}

impl<'a> XmlDecl<'a> {
    // [24] VersionInfo	::= S 'version' Eq ("'" VersionNum "'" | '"' VersionNum '"')
    fn parse_version_info(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
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
    fn parse_version_num(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        map(preceded(tag("1."), digit1), |version| {
            format!("1.{}", version).into()
        })(input)
    }
    // [80] EncodingDecl	::= S 'encoding' Eq ('"' EncName '"' | "'" EncName "'" )
    fn parse_encoding_decl(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
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
    fn parse_enc_name(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        map(
            pair(
                alt((alpha1, tag("-"))),
                many0(alt((alphanumeric1, tag("."), tag("_"), tag("-")))),
            ),
            |(first, rest)| format!("{}{}", first, rest.join("")).into(),
        )(input)
    }

    // [32] SDDecl	::= S 'standalone' Eq (("'" ('yes' | 'no') "'") | ('"' ('yes' | 'no') '"'))
    fn parse_sd_decl(input: &'a str) -> IResult<&'a str, Standalone> {
        map(
            tuple((
                Self::parse_multispace1,
                tag("standalone"),
                Self::parse_eq,
                alt((
                    delimited(tag("'"), alt((tag("yes"), tag("no"))), tag("'")),
                    delimited(tag("\""), alt((tag("yes"), tag("no"))), tag("\"")),
                )),
            )),
            |(_whtiespace, _standalone_literal, _eq, standalone)| {
                Standalone::from_str(standalone).unwrap()
            },
        )(input)
    }
}
