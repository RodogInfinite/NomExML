use crate::{error::Error, parse::Parse, IResult};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char},
    combinator::{map, verify},
    sequence::{pair, preceded, tuple},
};

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Name {
    pub prefix: Option<String>,
    pub local_part: String,
}

impl Name {
    pub fn new(prefix: Option<&str>, local_part: &str) -> Self {
        Self {
            prefix: prefix.map(|p| p.to_string()),
            local_part: local_part.to_string(),
        }
    }
}

pub trait ParseNamespace<'a>: Parse<'a> + Sized {
    // [1] NSAttName ::=   	PrefixedAttName | DefaultAttName
    fn parse_namespace_attribute_name(input: &'static str) -> IResult<&'static str, Name> {
        let (input, name) = alt((Self::parse_name, Self::parse_prefixed_attribute_name))(input)?;
        if name.prefix.is_none() && name.local_part != "xmlns" {
            return Err(nom::Err::Error(Error::NomError(nom::error::Error::new(
                input.into(),
                nom::error::ErrorKind::Verify,
            ))));
        }

        Ok((input, name))
    }

    // [2] PrefixedAttName ::=  'xmlns:' NCName
    fn parse_prefixed_attribute_name(input: &'static str) -> IResult<&'static str, Name> {
        map(
            preceded(tag("xmlns:"), Self::parse_non_colonized_name),
            |local_part| Name {
                prefix: Some("xmlns".into()),
                local_part,
            },
        )(input)
    }

    // [4] NCName ::= Name - (Char* ':' Char*)  /* An XML Name, minus the ":" */
    fn parse_non_colonized_name(input: &'static str) -> IResult<&'static str, String> {
        map(
            pair(
                Self::parse_name_start_char,
                nom::bytes::complete::take_while1(|c: char| Self::is_name_char(c) && c != ':'),
            ),
            |(start_char, rest_chars)| {
                let mut name = start_char.to_string();
                name.push_str(rest_chars);
                name
            },
        )(input)
    }

    // [5] NCNameChar ::= NameChar - ':' /* An XML NameChar, minus the ":" */
    fn parse_non_colonized_name_char(input: &'static str) -> IResult<&'static str, char> {
        verify(Self::parse_name_char, |c| *c != ':')(input)
    }

    // [6] NCNameStartChar ::= NCName - ( Char Char Char* ) /* The first letter of an NCName */
    fn parse_non_colonized_name_start_char(input: &'static str) -> IResult<&'static str, char> {
        verify(anychar, |c| *c != ':')(input)
    }

    // [7] QName ::= PrefixedName | UnprefixedName
    fn parse_qualified_name(input: &'static str) -> IResult<&'static str, Name> {
        alt((
            Self::parse_prefixed_name,
            map(Self::parse_non_colonized_name, |local_part| Name {
                prefix: None,
                local_part,
            }),
            Self::parse_name, //unprefixed name
        ))(input)
    }

    // [8] PrefixedName	::= Prefix ':' LocalPart
    fn parse_prefixed_name(input: &'static str) -> IResult<&'static str, Name> {
        map(
            tuple((
                Self::parse_non_colonized_name,
                char(':'),
                Self::parse_non_colonized_name,
            )),
            |(prefix, _colon_literal, local_part)| Name {
                prefix: Some(prefix),
                local_part,
            },
        )(input)
    }

    // [9] UnprefixedName ::= LocalPart
    // [10] Prefix ::= NCName
    // [11] LocalPart ::= NCName
}
