use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit1},
    combinator::opt,
    multi::many0,
    sequence::delimited,
    IResult,
};

use crate::parse::Parse;

#[derive(Clone, PartialEq)]
pub struct XmlDecl<'a> {
    pub version: Cow<'a, str>,
    pub encoding: Option<Cow<'a, str>>,
    pub standalone: Option<Cow<'a, str>>,
}
impl<'a> Parse<'a> for XmlDecl<'a> {
    // [23] XMLDecl	::=  '<?xml' VersionInfo EncodingDecl? SDDecl? S? '?>'
    fn parse(input: &'a str) -> IResult<&'a str, XmlDecl<'a>> {
        let (input, _) = tag("<?xml")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
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
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, version) = alt((
            delimited(tag("'"), Self::parse_version_num, tag("'")),
            delimited(tag("\""), Self::parse_version_num, tag("\"")),
        ))(input)?;
        Ok((input, version))
    }

    // [25] Eq	::= S? '=' S?
    fn parse_eq(input: &'a str) -> IResult<&'a str, &'a str> {
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        Ok((input, "="))
    }

    // [26] VersionNum	::= '1.' [0-9]+
    fn parse_version_num(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, _) = tag("1.")(input)?;
        let (input, version) = digit1(input)?;
        Ok((input, version.into()))
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

    // [32] SDDecl	::= S 'standalone' Eq (("'" ('yes' | 'no') "'") | ('"' ('yes' | 'no') '"'))
    fn parse_sd_decl(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, _) = tag("standalone")(input)?;
        let (input, _) = Self::parse_eq(input)?;
        let (input, standalone) = alt((
            delimited(tag("'"), alt((tag("yes"), tag("no"))), tag("'")),
            delimited(tag("\""), alt((tag("yes"), tag("no"))), tag("\"")),
        ))(input)?;
        Ok((input, standalone.into()))
    }
    // [81] EncName	::= [A-Za-z] ([A-Za-z0-9._] | '-')*
    fn parse_enc_name(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, first) = alt((alpha1, tag("-")))(input)?;
        let (input, rest) = many0(alt((alphanumeric1, tag("."), tag("_"), tag("-"))))(input)?;
        Ok((input, format!("{}{}", first, rest.join("")).into()))
    }
}
