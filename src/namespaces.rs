use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char},
    combinator::{map, verify},
    sequence::tuple,
    IResult,
};

use crate::parse::Parse;

#[derive(Clone, PartialEq)]
pub struct QualifiedName<'a> {
    pub prefix: Option<Cow<'a, str>>,
    pub local_part: Cow<'a, str>,
}

pub enum Namespace {
    Default,
    Prefix(Cow<'static, str>),
    URI(Cow<'static, str>),
}

pub trait ParseNamespace<'a>: Parse<'a> + Sized {
    // [1] NSAttName ::=   	PrefixedAttName | DefaultAttName
    fn parse_namespace_attribute_name(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, name) = alt((
            Self::parse_prefixed_attribute_name,
            map(tag("xmlns"), |s: &'a str| Cow::Borrowed(s)),
        ))(input)?;
        Ok((input, name))
    }

    // [2] PrefixedAttName ::=  'xmlns:' NCName
    fn parse_prefixed_attribute_name(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, _) = tag("xmlns:")(input)?;
        let (input, name) = Self::parse_non_colonized_name(input)?;
        Ok((input, name))
    }

    // [4] NCName ::= Name - (Char* ':' Char*)  /* An XML Name, minus the ":" */
    fn parse_non_colonized_name(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, valid_name) =
            verify(Self::parse_name, |name: &Cow<str>| !name.contains(':'))(input)?;
        Ok((input, valid_name))
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
        ))(input)
    }

    // [8] PrefixedName	::= Prefix ':' LocalPart
    fn parse_prefixed_name(input: &'a str) -> IResult<&'a str, QualifiedName<'a>> {
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
