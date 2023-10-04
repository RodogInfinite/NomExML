use crate::{
    attribute::Attribute, namespaces::ParseNamespace, parse::Parse,
    prolog::subset::entity_value::EntityValue, Name,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt},
    multi::{many0, many1},
    sequence::{delimited, pair, tuple},
    IResult,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug, PartialEq)]

pub enum TagState {
    Start,
    End,
    Empty,
}

#[derive(Clone, PartialEq)]
pub struct Tag {
    pub name: Name,
    pub attributes: Option<Vec<Attribute>>, // Attribute::Instance
    pub state: TagState,
}

impl<'a> Parse<'a> for Tag {
    type Args = ();
    type Output = IResult<&'a str, Self>;
}
impl<'a> ParseNamespace<'a> for Tag {}

// TODO: Investigate. The hardcoded bracket codes is kind of a hack to get reference element parsing to work. Unsure of how this is going to impact invalid XML.
// Tried to use decode, but having some lifetime issues
impl Tag {
    // [40] STag ::= '<' Name (S Attribute)* S? '>'
    // Namespaces (Third Edition) [12] STag ::= '<' QName (S Attribute)* S? '>'
    pub fn parse_start_tag(
        input: &str,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> IResult<&str, Self> {
        map(
            tuple((
                alt((tag("&#60;"), tag("&#x3C;"), tag("<"))),
                alt((Self::parse_name, Self::parse_qualified_name)),
                many0(pair(
                    Self::parse_multispace1,
                    |i| Attribute::parse_qualified_attribute(i, entity_references.clone()), //TODO merge behavior with parse_attribute
                )),
                Self::parse_multispace0,
                alt((tag("&#62;"), tag("&#x3E;"), tag(">"))),
            )),
            |(_open_char, name, attributes, _whitespace, _close_char)| {
                let attributes: Vec<_> = attributes
                    .into_iter()
                    .map(|(_whitespace, attr)| attr)
                    .collect();
                Self {
                    name,
                    attributes: if attributes.is_empty() {
                        None
                    } else {
                        Some(attributes)
                    },
                    state: TagState::Start,
                }
            },
        )(input)
    }

    // [42] ETag ::= '</' Name S? '>'
    // Namespaces (Third Edition) [13] ETag ::= '</' QName S? '>'
    pub fn parse_end_tag(input: &str) -> IResult<&str, Self> {
        delimited(
            alt((tag("&#60;/"), tag("&#x3C;/"), tag("</"))),
            map(
                tuple((
                    Self::parse_multispace0,
                    alt((Self::parse_name, Self::parse_qualified_name)),
                    Self::parse_multispace0,
                )),
                |(_open_tag, name, _close_tag)| Self {
                    name,
                    attributes: None, // Attributes are not parsed for end tags
                    state: TagState::End,
                },
            ),
            alt((tag("&#62;"), tag("&#x3E;"), tag(">"))),
        )(input)
    }

    // [44] EmptyElemTag ::= '<' Name (S Attribute)* S? '/>'
    // Namespaces (Third Edition) [14] EmptyElemTag ::= '<' QName (S Attribute)* S? '/>'
    pub fn parse_empty_element_tag(
        input: &str,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> IResult<&str, Tag> {
        map(
            tuple((
                alt((tag("&#60;"), tag("&#x3C;"), tag("<"))),
                alt((Self::parse_name, Self::parse_qualified_name)),
                opt(many1(pair(Self::parse_multispace1, |i| {
                    Attribute::parse(i, entity_references.clone())
                }))),
                Self::parse_multispace0,
                alt((tag("/&#62;"), tag("/&#x3E;"), tag("/>"))),
            )),
            |(_open_tag, name, attributes, _whitespace, _close_tag)| Self {
                name,
                attributes: attributes
                    .map(|attr| attr.into_iter().map(|(_whitespace, attr)| attr).collect()),
                state: TagState::Empty,
            },
        )(input)
    }
}
