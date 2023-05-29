use std::borrow::Cow;

use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::map,
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};


use crate::{attribute::Attribute, document::Document, utils::Parse};

#[derive(Clone, Debug, PartialEq)]
pub struct Namespace<'a> {
    pub declaration: Option<Cow<'a, str>>,
    pub prefix: Cow<'a, str>,
    pub uri: Option<Cow<'a, str>>,
}

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
}

#[derive(Clone, PartialEq)]
pub struct Tag<'a> {
    pub name: Cow<'a, str>,
    pub namespace: Option<Namespace<'a>>,
    pub attributes: Option<Vec<Attribute<'a>>>, // Attribute::Instance
    pub state: TagState,
}

impl<'a> Tag<'a> {
    pub fn parse_attributes(input: &'a str) -> IResult<&'a str, Option<Vec<Attribute<'a>>>> {
        let mut parser = many0(Attribute::parse_attribute_instance);
        let (input, attributes) = parser(input)?;
        if attributes.is_empty() {
            Ok((input, None))
        } else {
            Ok((input, Some(attributes)))
        }
    }

    pub fn parse_start_tag(input: &'a str) -> IResult<&'a str, Self> {
        map(
            delimited(
                tag("<"),
                tuple((
                    delimited(multispace0, Document::parse_tag_and_namespace, multispace0),
                    Self::parse_attributes,
                )),
                tag(">"),
            ),
            |((name, namespace), attributes)| Self {
                name,
                namespace,
                attributes,
                state: TagState::Start,
            },
        )(input)
    }

    pub fn parse_end_tag(input: &'a str) -> IResult<&'a str, Self> {
        map(
            delimited(
                preceded(tag("</"), Self::parse_multispace0),
                Document::parse_tag_and_namespace,
                preceded(Self::parse_multispace0, tag(">")),
            ),
            |(name, namespace)| Self {
                name,
                namespace,
                attributes: None, // Attributes are not parsed for end tags
                state: TagState::End,
            },
        )(input)
    }
}

impl<'a> Parse<'a> for Tag<'a> {}