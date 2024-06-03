use crate::{
    attribute::{Attribute, AttributeValue, DefaultDecl},
    error::Error,
    namespaces::ParseNamespace,
    parse::Parse,
    prolog::subset::entity::{entity_value::EntityValue, EntitySource},
    IResult, Name,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, map_res, opt},
    multi::{many0, many1},
    sequence::{delimited, pair, tuple},
};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]

pub enum TagState {
    Start,
    End,
    Empty,
}

#[derive(Clone, PartialEq, Eq)]
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
    pub fn new(name: Name, attributes: Option<Vec<Attribute>>, state: TagState) -> Self {
        Self {
            name,
            attributes,
            state,
        }
    }

    // [40] STag ::= '<' Name (S Attribute)* S? '>'
    // Namespaces (Third Edition) [12] STag ::= '<' QName (S Attribute)* S? '>'
    pub fn parse_start_tag(
        input: &str,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> IResult<&str, Self> {
        map(
            tuple((
                alt((tag("&#60;"), tag("&#x3C;"), tag("<"))),
                alt((Self::parse_qualified_name, Self::parse_name)),
                many0(pair(Self::parse_multispace1, |i| {
                    Attribute::parse_attribute(i, entity_references.clone(), entity_source.clone())
                })),
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
                        // check doctype here, if within that, add them to the tag else, None
                        None
                    } else {
                        Some(attributes)
                    },
                    state: TagState::Start,
                }
            },
        )(input)
    }

    pub fn parse_start_tag_by_name<'a>(
        input: &'a str,
        tag_name: &'a str,
        attributes: &Option<Vec<Attribute>>,
        entity_references: &Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> IResult<&'a str, Self> {
        let mut current_input = input;

        loop {
            let result: IResult<&'a str, Self> = map_res(
                tuple((
                    alt((tag("&#60;"), tag("&#x3C;"), tag("<"))),
                    map_res(
                        alt((Self::parse_qualified_name, Self::parse_name)),
                        |name| {
                            if name.local_part == tag_name {
                                Ok(name)
                            } else {
                                Err(nom::Err::Error(nom::error::Error::new(
                                    "Start Tag doesn't match",
                                    nom::error::ErrorKind::Tag,
                                )))
                            }
                        },
                    ),
                    many0(pair(Self::parse_multispace1, |i| {
                        Attribute::parse_attribute(i, entity_references.clone(), entity_source.clone())
                    })),
                    Self::parse_multispace0,
                    alt((tag("&#62;"), tag("&#x3E;"), tag(">"))),
                )),
                |(_open_char, name, attributes_vec, _whitespace, _close_char)| -> Result<Self, nom::Err<Error>> {
                    let parsed_attributes: Vec<_> = attributes_vec
                        .into_iter()
                        .map(|(_whitespace, attr)| attr)
                        .collect();
                    if let Some(expected_attributes) = attributes {
                        if expected_attributes == &parsed_attributes {
                            Ok(Self {
                                name,
                                attributes: if parsed_attributes.is_empty() {
                                    None
                                } else {
                                    Some(parsed_attributes)
                                },
                                state: TagState::Start,
                            })
                        } else {
                            Err(nom::Err::Error(nom::error::Error::new(
                                "Attributes do not match",
                                nom::error::ErrorKind::Tag,
                            ).into()))
                        }
                    } else {
                        Ok(Self {
                            name,
                            attributes: if parsed_attributes.is_empty() {
                                None
                            } else {
                                Some(parsed_attributes)
                            },
                            state: TagState::Start,
                        })
                    }
                },
            )(current_input);

            match result {
                Ok((next_input, tag)) => return Ok((next_input, tag)),
                Err(nom::Err::Error(_)) => {
                    if current_input.is_empty() {
                        return Err(nom::Err::Error(
                            nom::error::Error::new(current_input, nom::error::ErrorKind::Tag)
                                .into(),
                        ));
                    }
                    // Move forward in the input string to avoid infinite loop
                    current_input = &current_input[1..];
                }
                Err(e) => return Err(e),
            }
        }
    }
    // [42] ETag ::= '</' Name S? '>'
    // Namespaces (Third Edition) [13] ETag ::= '</' QName S? '>'
    pub fn parse_end_tag(input: &str) -> IResult<&str, Self> {
        delimited(
            alt((tag("&#60;/"), tag("&#x3C;/"), tag("</"))),
            map(
                tuple((
                    Self::parse_multispace0,
                    alt((Self::parse_qualified_name, Self::parse_name)),
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
    // [42] ETag ::= '</' Name S? '>'
    // Namespaces (Third Edition) [13] ETag ::= '</' QName S? '>'
    pub fn parse_end_tag_by_name<'a>(input: &'a str, tag_name: &'a str) -> IResult<&'a str, Self> {
        delimited(
            alt((tag("&#60;/"), tag("&#x3C;/"), tag("</"))),
            map(
                tuple((
                    Self::parse_multispace0,
                    map_res(
                        alt((Self::parse_qualified_name, Self::parse_name)),
                        |name| {
                            if name.local_part == tag_name {
                                Ok(name)
                            } else {
                                Err(nom::Err::Error(nom::error::Error::new(
                                    "END TAG FAILING",
                                    nom::error::ErrorKind::Tag,
                                )))
                            }
                        },
                    ),
                    Self::parse_multispace0,
                )),
                |(_open_tag, name, _close_tag)| Self {
                    name,             //: Name::new(None, name),
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
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> IResult<&str, Self> {
        map(
            tuple((
                alt((tag("&#60;"), tag("&#x3C;"), tag("<"))),
                alt((Self::parse_qualified_name, Self::parse_name)),
                opt(many1(pair(Self::parse_multispace1, |i| {
                    Attribute::parse(i, (entity_references.clone(), entity_source.clone()))
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

    // [44] EmptyElemTag ::= '<' Name (S Attribute)* S? '/>'
    // Namespaces (Third Edition) [14] EmptyElemTag ::= '<' QName (S Attribute)* S? '/>'
    pub fn parse_empty_element_tag_by_name<'a>(
        input: &'a str,
        tag_name: &'a str,
        _attributes: &Option<Vec<Attribute>>, //TODO: implement empty tag attribute matching
        entity_references: &Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> IResult<&'a str, Self> {
        map(
            tuple((
                alt((tag("&#60;"), tag("&#x3C;"), tag("<"))),
                tag(tag_name),
                opt(many1(pair(Self::parse_multispace1, |i| {
                    Attribute::parse(i, (entity_references.clone(), entity_source.clone()))
                }))),
                Self::parse_multispace0,
                alt((tag("/&#62;"), tag("/&#x3E;"), tag("/>"))),
            )),
            |(_open_tag, name, attributes, _whitespace, _close_tag)| Self {
                name: Name::new(None, name),
                attributes: attributes
                    .map(|attr| attr.into_iter().map(|(_whitespace, attr)| attr).collect()),
                state: TagState::Empty,
            },
        )(input)
    }
    pub fn merge_default_attributes(&mut self, default_attributes: &[Attribute]) {
        let existing_attributes = self.attributes.get_or_insert_with(Vec::new);

        let mut seen_names = std::collections::HashSet::new();
        for default_attr in default_attributes {
            if let Attribute::Definition {
                name, default_decl, ..
            } = default_attr
            {
                if seen_names.contains(name) {
                    // Skip if this name has already been processed.
                    continue;
                }
                seen_names.insert(name.clone());

                // Only add the attribute if it doesn't already exist and has a default value
                let exists = existing_attributes.iter().any(|attr| matches!(attr, Attribute::Instance { name: existing_name, .. } if existing_name == name));
                if !exists {
                    if let DefaultDecl::Value(val) = default_decl {
                        existing_attributes.push(Attribute::Instance {
                            name: name.clone(),
                            value: AttributeValue::Value(val.clone()),
                        });
                    }
                }
            }
        }

        // If no attributes were added (and none were already present), set attributes to None
        if existing_attributes.is_empty() {
            self.attributes = None;
        }
    }

    pub fn add_attributes(&mut self, new_attributes: Vec<Attribute>) {
        self.attributes = if new_attributes.is_empty() {
            None
        } else {
            Some(new_attributes)
        };
    }
}
