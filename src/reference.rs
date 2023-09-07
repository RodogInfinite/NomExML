// reference.rs

use crate::{
    attribute::AttributeValue,
    parse::Parse,

    prolog::internal_subset::entity_value::EntityValue,
    //transcode::{decode_digit, decode_hex},
    transcode::Decode,
    Name,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, hex_digit1},
    combinator::{map, recognize},
    sequence::tuple,
    IResult,
};
use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, PartialEq)]
pub enum Reference<'a> {
    EntityRef(Name<'a>),
    CharRef(Cow<'a, str>),
}

impl<'a> Parse<'a> for Reference<'a> {
    type Args = Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>;
    type Output = IResult<&'a str, Self>;
    //[67] Reference ::= EntityRef | CharRef
    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
        dbg!("Reference::parse");
        dbg!(&input);
        alt((Self::parse_entity_ref, Self::parse_char_reference))(input)
    }
}
impl<'a> Reference<'a> {
    pub fn normalize_entity(
        &self,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> EntityValue<'a> {
        dbg!("NORMALIZE");
        dbg!(&self);

        match self {
            Reference::EntityRef(name) => {
                dbg!("ENTITYREF NAME");
                dbg!(&name);
                dbg!(&*entity_references.borrow());

                let refs_map = entity_references.borrow();
                match refs_map.get(name).cloned() {
                    Some(EntityValue::Value(val))
                        if refs_map.contains_key(&Name {
                            prefix: None,
                            local_part: Cow::Borrowed(val.as_ref()),
                        }) =>
                    {
                        // This value is another reference
                        let reference_name = Name {
                            prefix: None,
                            local_part: Cow::Owned(val.into_owned()),
                        };
                        Reference::EntityRef(reference_name)
                            .normalize_entity(entity_references.clone())
                    }
                    Some(EntityValue::Reference(Reference::EntityRef(entity))) => {
                        dbg!("DOING WORK");
                        dbg!(&entity);
                        if let Some(EntityValue::Value(val)) = refs_map.get(&entity).cloned() {
                            EntityValue::Value(val)
                        } else {
                            Reference::EntityRef(entity).normalize_entity(entity_references.clone())
                        }
                    }
                    Some(EntityValue::Reference(Reference::CharRef(value))) => {
                        EntityValue::Value(value)
                    }
                    Some(entity_value) => entity_value,
                    None => EntityValue::Value(Cow::Owned(name.local_part.to_string())),
                }
            }
            Reference::CharRef(value) => EntityValue::Value(Cow::Owned(value.to_string())),
        }
    }
    pub fn normalize_attribute(
        &self,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> AttributeValue<'a> {
        dbg!("NORMALIZE ATTRIBUTE");
        dbg!(&self);
        match self {
            Reference::EntityRef(name) => {
                dbg!("ATT REF NAME");
                dbg!(&name);
                dbg!(&*entity_references.borrow());

                let refs_map = entity_references.borrow();
                match refs_map.get(name).cloned() {
                    Some(EntityValue::Value(val))
                        if refs_map.contains_key(&Name {
                            prefix: None,
                            local_part: Cow::Borrowed(val.as_ref()),
                        }) =>
                    {
                        let reference_name = Name {
                            prefix: None,
                            local_part: Cow::Owned(val.into_owned()),
                        };
                        Reference::EntityRef(reference_name)
                            .normalize_attribute(entity_references.clone())
                    }
                    Some(EntityValue::Reference(Reference::EntityRef(entity))) => {
                        dbg!("DOING WORK2");
                        dbg!(&entity);
                        if let Some(EntityValue::Value(val)) = refs_map.get(&entity).cloned() {
                            AttributeValue::Value(val)
                        } else {
                            Reference::EntityRef(entity.clone())
                                .normalize_attribute(entity_references.clone())
                        }
                    }
                    Some(entity_value) => {
                        // Convert EntityValue to AttributeValue
                        match entity_value {
                            EntityValue::Value(val) => AttributeValue::Value(val),
                            EntityValue::Reference(reference) => {
                                reference.normalize_attribute(entity_references.clone())
                            }
                            _ => panic!("Unexpected EntityValue variant"),
                        }
                    }
                    None => AttributeValue::Value(Cow::Owned(name.local_part.to_string())),
                }
            }
            Reference::CharRef(value) => {
                dbg!("CHARREF HERE");
                dbg!(&value);
                AttributeValue::Value(Cow::Owned(value.to_string()))
            }
        }
    }
}

impl<'a> ParseReference<'a> for Reference<'a> {}
impl<'a> Decode for Reference<'a> {
    fn as_str(&self) -> &str {
        match self {
            Reference::EntityRef(name) => &name.local_part,
            Reference::CharRef(value) => value,
        }
    }
}

pub trait ParseReference<'a>: Parse<'a> + Decode {
    //[68] EntityRef ::= '&' Name ';'
    fn parse_entity_ref(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        dbg!("PARSE ENTITY REF");
        dbg!(&input);
        map(
            tuple((char('&'), Self::parse_name, char(';'))),
            |(_, name, _)| {
                dbg!(&name);
                Reference::EntityRef(name)
            },
        )(input)
    }

    //[69] PEReference ::= '%' Name ';'
    fn parse_parameter_reference(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        map(
            tuple((char('%'), Self::parse_name, char(';'))),
            |(_, name, _)| Reference::EntityRef(name),
        )(input)
    }

    //[66] CharRef ::= '&#' [0-9]+ ';' | '&#x' [0-9a-fA-F]+ ';'
    fn parse_char_reference(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        //TODO: remove reconstruction if possible
        dbg!("parse_char_reference");
        dbg!(&input);
        alt((
            map(
                tuple((tag("&#"), digit1, tag(";"))),
                |(start, digits, end): (&str, &str, &str)| {
                    let reconstructed = format!("{}{}{}", start, digits, end);
                    let decoded = reconstructed.decode().unwrap().into_owned();
                    dbg!(&decoded);
                    Reference::CharRef(Cow::Owned(decoded))
                },
            ),
            map(
                tuple((tag("&#x"), hex_digit1, tag(";"))),
                |(start, hex, end): (&str, &str, &str)| {
                    let reconstructed = format!("{}{}{}", start, hex, end);
                    let decoded = reconstructed.decode().unwrap().into_owned();
                    Reference::CharRef(Cow::Owned(decoded))
                },
            ),
        ))(input)
    }
}
