use crate::{parse::Parse, Name, QualifiedName};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char},
    combinator::{map, verify},
    sequence::{pair, preceded, tuple},
    IResult,
};
use std::borrow::Cow;

pub trait ParseNamespace<'a>: Parse<'a> + Sized {
    // [1] NSAttName ::=   	PrefixedAttName | DefaultAttName
    fn parse_namespace_attribute_name(input: &'a str) -> IResult<&'a str, Name<'a>> {
        let (input, name) = alt((Self::parse_name, Self::parse_prefixed_attribute_name))(input)?;
        if name.prefix.is_none() && name.local_part != "xmlns" {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }

        Ok((input, name))
    }

    // [2] PrefixedAttName ::=  'xmlns:' NCName
    fn parse_prefixed_attribute_name(input: &'a str) -> IResult<&'a str, QualifiedName> {
        map(
            preceded(tag("xmlns:"), Self::parse_non_colonized_name),
            |local_part| QualifiedName {
                prefix: Some("xmlns".into()),
                local_part,
            },
        )(input)
    }

    // [4] NCName ::= Name - (Char* ':' Char*)  /* An XML Name, minus the ":" */
    fn parse_non_colonized_name(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        map(
            pair(
                Self::parse_name_start_char,
                nom::bytes::complete::take_while1(|c: char| Self::is_name_char(c) && c != ':'),
            ),
            |(start_char, rest_chars)| {
                let mut name = start_char.to_string();
                name.push_str(rest_chars);
                Cow::Owned(name)
            },
        )(input)
    }

    // [5] NCNameChar ::= NameChar - ':' /* An XML NameChar, minus the ":" */
    fn parse_non_colonized_name_char(input: &'a str) -> IResult<&'a str, char> {
        let (input, valid_char) = verify(Self::parse_name_char, |c| *c != ':')(input)?;
        Ok((input, valid_char))
    }

    // [6] NCNameStartChar ::= NCName - ( Char Char Char* ) /* The first letter of an NCName */
    fn parse_non_colonized_name_start_char(input: &'a str) -> IResult<&'a str, char> {
        let (input, valid_char) = verify(anychar, |c| *c != ':')(input)?;
        Ok((input, valid_char))
    }

    // [7] QName ::= PrefixedName | UnprefixedName
    fn parse_qualified_name(input: &'a str) -> IResult<&'a str, QualifiedName> {
        alt((
            Self::parse_prefixed_name,
            map(Self::parse_non_colonized_name, |local_part| QualifiedName {
                prefix: None,
                local_part,
            }),
            Self::parse_name, //unprefixed name
        ))(input)
    }

    // [8] PrefixedName	::= Prefix ':' LocalPart
    fn parse_prefixed_name(input: &'a str) -> IResult<&'a str, QualifiedName> {
        map(
            tuple((
                Self::parse_non_colonized_name,
                char(':'),
                Self::parse_non_colonized_name,
            )),
            |(prefix, _, local_part)| QualifiedName {
                prefix: Some(prefix),
                local_part,
            },
        )(input)
    }

    // [9] UnprefixedName ::= LocalPart
    // [10] Prefix ::= NCName
    // [11] LocalPart ::= NCName
}
