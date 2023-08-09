use crate::{
    attribute::Attribute, namespaces::ParseNamespace, parse::Parse,
    prolog::internal_subset::EntityValue, Name,
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
pub struct Tag<'a> {
    pub name: Name<'a>,
    pub attributes: Option<Vec<Attribute<'a>>>, // Attribute::Instance
    pub state: TagState,
}

impl<'a> Parse<'a> for Tag<'a> {
    type Args = ();
    type Output = IResult<&'a str, Self>;
}
impl<'a> ParseNamespace<'a> for Tag<'a> {}

impl<'a> Tag<'a> {
    // [40] STag ::= '<' Name (S Attribute)* S? '>'
    // Namespaces (Third Edition) [12] STag ::= '<' QName (S Attribute)* S? '>'
    pub fn parse_start_tag(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, Self> {
        let (input, tag) = map(
            tuple((
                char('<'),
                alt((Self::parse_name, Self::parse_qualified_name)),
                many0(pair(
                    Self::parse_multispace1,
                    |i| Attribute::parse_qualified_attribute(i, entity_references.clone()), //TODO merge behavior with parse_attribute
                )),
                Self::parse_multispace0,
                char('>'),
            )),
            |(_open_char, name, attributes, _whitespace, _close_char)| {
                println!("ATTRIBUTE WITHIN TAG: {attributes:?}");
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
        )(input)?;
        Ok((input, tag))
    }

    // [42] ETag ::= '</' Name S? '>'
    // Namespaces (Third Edition) [13] ETag ::= '</' QName S? '>'
    pub fn parse_end_tag(input: &'a str) -> IResult<&'a str, Self> {
        println!("PARSING END TAG: {input}");
        delimited(
            tag("</"),
            map(
                tuple((
                    Self::parse_multispace0,
                    alt((Self::parse_name, Self::parse_qualified_name)),
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
    // [44] EmptyElemTag ::= '<' Name (S Attribute)* S? '/>'
    // Namespaces (Third Edition) [14] EmptyElemTag ::= '<' QName (S Attribute)* S? '/>'
    pub fn parse_empty_element_tag(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, Tag<'a>> {
        println!("PARSING EMPTY ELEMENT TAG: {input}");
        map(
            tuple((
                char('<'),
                alt((Self::parse_name, Self::parse_qualified_name)),
                opt(many1(pair(Self::parse_multispace1, |i| {
                    Attribute::parse(i, entity_references.clone())
                }))),
                Self::parse_multispace0,
                tag("/>"),
            )),
            |(_, name, attributes, _, _)| Self {
                name,
                attributes: attributes.map(|attr| attr.into_iter().map(|(_, attr)| attr).collect()),
                state: TagState::Empty,
            },
        )(input)
    }
}
