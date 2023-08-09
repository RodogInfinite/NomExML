use crate::parse::Parse;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit1},
    combinator::opt,
    error::ErrorKind,
    multi::many0,
    sequence::delimited,
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
        let (input, _) = tag("<?xml")(input)?;
        let (input, version) = Self::parse_version_info(input)?;
        let (input, encoding) = opt(Self::parse_encoding_decl)(input)?;
        let (input, standalone) = opt(Self::parse_sd_decl)(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag("?>")(input)?;
        Ok((
            input,
            Self {
                version,
                encoding,
                standalone,
            },
        ))
    }
}

impl<'a> XmlDecl<'a> {
    // [24] VersionInfo	::= S 'version' Eq ("'" VersionNum "'" | '"' VersionNum '"')
    fn parse_version_info(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, _) = tag("version")(input)?;
        let (input, _) = Self::parse_eq(input)?;
        let (input, version) = alt((
            delimited(tag("'"), Self::parse_version_num, tag("'")),
            delimited(tag("\""), Self::parse_version_num, tag("\"")),
        ))(input)?;
        Ok((input, version))
    }

    // [26] VersionNum	::= '1.' [0-9]+
    fn parse_version_num(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, _) = tag("1.")(input)?;
        let (input, version) = digit1(input)?;
        let version_with_prefix = format!("1.{}", version);
        Ok((input, version_with_prefix.into()))
    }
    // [80] EncodingDecl	::= S 'encoding' Eq ('"' EncName '"' | "'" EncName "'" )
    fn parse_encoding_decl(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, _) = tag("encoding")(input)?;
        let (input, _) = Self::parse_eq(input)?;
        let (input, encoding) = alt((
            delimited(tag("'"), Self::parse_enc_name, tag("'")),
            delimited(tag("\""), Self::parse_enc_name, tag("\"")),
        ))(input)?;
        Ok((input, encoding))
    }

    // [81] EncName	::= [A-Za-z] ([A-Za-z0-9._] | '-')*
    fn parse_enc_name(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, first) = alt((alpha1, tag("-")))(input)?;
        let (input, rest) = many0(alt((alphanumeric1, tag("."), tag("_"), tag("-"))))(input)?;
        Ok((input, format!("{}{}", first, rest.join("")).into()))
    }

    // [32] SDDecl	::= S 'standalone' Eq (("'" ('yes' | 'no') "'") | ('"' ('yes' | 'no') '"'))
    fn parse_sd_decl(input: &'a str) -> IResult<&'a str, Standalone> {
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, _) = tag("standalone")(input)?;
        let (input, _) = Self::parse_eq(input)?;
        let (input, standalone) = alt((
            delimited(tag("'"), alt((tag("yes"), tag("no"))), tag("'")),
            delimited(tag("\""), alt((tag("yes"), tag("no"))), tag("\"")),
        ))(input)?;
        let standalone = Standalone::from_str(standalone)?;
        Ok((input, standalone))
    }
}
