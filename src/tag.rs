use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    multi::many0,
    sequence::{delimited, pair, tuple},
    IResult,
};

use crate::{attribute::Attribute, namespaces::ParseNamespace, parse::Parse, QualifiedName};

#[derive(Clone, Debug, PartialEq)]
pub enum ConditionalState {
    None,
    Optional,
    ZeroOrMore,
    OneOrMore,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TagState {
    Start,
    End,
    Empty,
}

#[derive(Clone, PartialEq)]
pub struct Tag<'a> {
    pub name: QualifiedName<'a>,
    pub attributes: Option<Vec<Attribute<'a>>>, // Attribute::Instance
    pub state: TagState,
}

impl<'a> Parse<'a> for Tag<'a> {}
impl<'a> ParseNamespace<'a> for Tag<'a> {}

impl<'a> Tag<'a> {
    // [44] EmptyElemTag ::= '<' Name (S Attribute)* S? '/>'
    pub fn parse_empty_element_tag(input: &'a str) -> IResult<&'a str, Tag<'a>> {
        map(
            tuple((
                char('<'),
                Self::parse_name,
                many0(pair(Self::parse_multispace1, Attribute::parse)),
                Self::parse_multispace0,
                tag("/>"),
            )),
            |(_, name, attributes, _, _)| Self {
                name: QualifiedName {
                    prefix: None,
                    local_part: name,
                },
                attributes: Some(attributes.into_iter().map(|(_, attr)| attr).collect()),
                state: TagState::Empty,
            },
        )(input)
    }

    // [40] STag ::= '<' Name (S Attribute)* S? '>'
    pub fn parse_start_tag(input: &'a str) -> IResult<&'a str, Self> {
        map(
            tuple((
                char('<'),
                Self::parse_name,
                many0(pair(
                    Self::parse_multispace1,
                    alt((Attribute::parse_qualified_attribute, Attribute::parse)),
                )),
                Self::parse_multispace0,
                char('>'),
            )),
            |(_, name, attributes, _, _)| {
                let attr = if attributes.is_empty() {
                    None
                } else {
                    Some(attributes.into_iter().map(|(_, attr)| attr).collect())
                };

                Self {
                    name: QualifiedName {
                        prefix: None,
                        local_part: name,
                    },
                    attributes: attr,
                    state: TagState::Start,
                }
            },
        )(input)
    }

    // [42] ETag ::= '</' Name S? '>'
    pub fn parse_end_tag(input: &'a str) -> IResult<&'a str, Self> {
        delimited(
            tag("</"),
            map(
                tuple((
                    Self::parse_multispace0,
                    Self::parse_name,
                    Self::parse_multispace0,
                )),
                |(_, name, _)| Self {
                    name: QualifiedName {
                        prefix: None,
                        local_part: name,
                    },
                    attributes: None, // Attributes are not parsed for end tags
                    state: TagState::End,
                },
            ),
            char('>'),
        )(input)
    }
    // Namespaces (Third Edition) [12] STag ::= '<' QName (S Attribute)* S? '>'
    pub fn parse_qualified_start_tag(input: &'a str) -> IResult<&'a str, Self> {
        map(
            tuple((
                char('<'),
                Self::parse_qualified_name,
                many0(pair(
                    Self::parse_multispace1,
                    Attribute::parse_qualified_attribute,
                )),
                Self::parse_multispace0,
                char('>'),
            )),
            |(_, name, attributes, _, _)| Self {
                name,
                attributes: Some(attributes.into_iter().map(|(_, attr)| attr).collect()),
                state: TagState::Start,
            },
        )(input)
    }

    // Namespaces (Third Edition) [13] ETag ::= '</' QName S? '>'
    pub fn parse_qualified_end_tag(input: &'a str) -> IResult<&'a str, Self> {
        delimited(
            tag("</"),
            map(
                tuple((
                    Self::parse_multispace0,
                    Self::parse_qualified_name,
                    Self::parse_multispace0,
                )),
                |(_, name, _)| Self {
                    name,
                    attributes: None, // Attributes are not parsed for end tags
                    state: TagState::End,
                },
            ),
            char('>'),
        )(input)
    }
    // Namespaces (Third Edition) [14] EmptyElemTag ::= '<' QName (S Attribute)* S? '/>'
    pub fn parse_empty_qualified_element_tag(input: &'a str) -> IResult<&'a str, Tag<'a>> {
        map(
            tuple((
                char('<'),
                Self::parse_qualified_name,
                many0(pair(Self::parse_multispace1, Attribute::parse)),
                Self::parse_multispace0,
                tag("/>"),
            )),
            |(_, name, attributes, _, _)| Self {
                name,
                attributes: Some(attributes.into_iter().map(|(_, attr)| attr).collect()),
                state: TagState::Empty,
            },
        )(input)
    }
}
