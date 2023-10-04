// reference.rs

use crate::{
    attribute::AttributeValue,
    parse::Parse,

    prolog::subset::entity_value::EntityValue,
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
pub enum Reference {
    EntityRef(Name),
    CharRef(String),
}

impl<'a> Parse<'a> for Reference {
    type Args = Rc<RefCell<HashMap<Name, EntityValue>>>;
    type Output = IResult<&'a str, Self>;
    //[67] Reference ::= EntityRef | CharRef
    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
        alt((Self::parse_entity_ref, Self::parse_char_reference))(input)
    }
}
impl Reference {
    pub fn normalize_entity(
        &self,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> EntityValue {
        match self {
            Reference::EntityRef(name) => {
                dbg!(&*entity_references.borrow());

                let refs_map = entity_references.borrow();
                match refs_map.get(name).cloned() {
                    Some(EntityValue::Value(val))
                        if refs_map.contains_key(&Name {
                            prefix: None,
                            local_part: val.clone(),
                        }) =>
                    {
                        // This value is another reference
                        let reference_name = Name {
                            prefix: None,
                            local_part: val,
                        };
                        Reference::EntityRef(reference_name)
                            .normalize_entity(entity_references.clone())
                    }
                    Some(EntityValue::Reference(Reference::EntityRef(entity))) => {
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
                    None => EntityValue::Value(name.local_part.clone()),
                }
            }
            Reference::CharRef(value) => EntityValue::Value(value.clone()),
        }
    }
    pub fn normalize_attribute(
        &self,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> AttributeValue {
        match self {
            Reference::EntityRef(name) => {
                dbg!(&*entity_references.borrow());

                let refs_map = entity_references.borrow();
                match refs_map.get(name).cloned() {
                    Some(EntityValue::Value(val))
                        if refs_map.contains_key(&Name {
                            prefix: None,
                            local_part: val.clone(),
                        }) =>
                    {
                        let reference_name = Name {
                            prefix: None,
                            local_part: val,
                        };
                        Reference::EntityRef(reference_name)
                            .normalize_attribute(entity_references.clone())
                    }
                    Some(EntityValue::Reference(Reference::EntityRef(entity))) => {
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
                    None => AttributeValue::Value(name.local_part.clone()),
                }
            }
            Reference::CharRef(value) => AttributeValue::Value(value.clone()),
        }
    }
}

impl<'a> ParseReference<'a> for Reference {}
impl Decode for Reference {
    fn as_str(&self) -> &str {
        match self {
            Reference::EntityRef(name) => &name.local_part,
            Reference::CharRef(value) => value,
        }
    }
}

pub trait ParseReference<'a>: Parse<'a> + Decode {
    //[68] EntityRef ::= '&' Name ';'
    fn parse_entity_ref(input: &str) -> IResult<&str, Reference> {
        map(
            tuple((char('&'), Self::parse_name, char(';'))),
            |(_, name, _)| Reference::EntityRef(name),
        )(input)
    }

    //[69] PEReference ::= '%' Name ';'
    fn parse_parameter_reference(input: &str) -> IResult<&str, Reference> {
        map(
            tuple((char('%'), Self::parse_name, char(';'))),
            |(_, name, _)| Reference::EntityRef(name),
        )(input)
    }

    //[66] CharRef ::= '&#' [0-9]+ ';' | '&#x' [0-9a-fA-F]+ ';'
    fn parse_char_reference(input: &str) -> IResult<&str, Reference> {
        //TODO: remove reconstruction if possible
        alt((
            map(
                tuple((tag("&#"), digit1, tag(";"))),
                |(start, digits, end): (&str, &str, &str)| {
                    let reconstructed = format!("{}{}{}", start, digits, end);
                    let decoded = reconstructed.decode().unwrap().into_owned();
                    Reference::CharRef(decoded)
                },
            ),
            map(
                tuple((tag("&#x"), hex_digit1, tag(";"))),
                |(start, hex, end): (&str, &str, &str)| {
                    let reconstructed = format!("{}{}{}", start, hex, end);
                    let decoded = reconstructed.decode().unwrap().into_owned();
                    Reference::CharRef(decoded)
                },
            ),
        ))(input)
    }
}
