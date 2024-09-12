// reference.rs

use crate::{
    attribute::AttributeValue,
    parse::Parse,
    prolog::subset::entity::{entity_value::EntityValue, EntitySource},
    transcode::Decode,
    Document, IResult, Name,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, hex_digit1},
    combinator::map,
    sequence::tuple,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, PartialEq, Eq)]
pub enum Reference {
    EntityRef(Name),
    CharRef(String),
}

impl<'a> Parse<'a> for Reference {
    type Args = EntitySource;
    //);
    type Output = IResult<&'a str, Self>;
    //[67] Reference ::= EntityRef | CharRef
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        alt((
            move |i| Self::parse_entity_ref(i, args.clone()),
            Self::parse_char_reference,
        ))(input)
    }
}
impl Reference {
    pub(crate) fn normalize_entity(
        &self,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
    ) -> EntityValue {
        match self {
            Reference::EntityRef(name) => {
                let refs_map = entity_references.borrow();

                // Try to find the most appropriate source based on available references
                let possible_sources = [EntitySource::External, EntitySource::Internal];
                let entity_value = possible_sources
                    .iter()
                    .filter_map(|source| refs_map.get(&(name.clone(), source.clone())).cloned())
                    .next()
                    .unwrap_or_else(|| EntityValue::Value(name.local_part.clone())); // Default to just returning the name if no entity is found

                match entity_value {
                    EntityValue::Value(val) => {
                        if refs_map.contains_key(&(
                            Name {
                                prefix: None,
                                local_part: val.clone(),
                            },
                            EntitySource::Internal,
                        )) {
                            // This value is another reference, recurse
                            let reference_name = Name {
                                prefix: None,
                                local_part: val,
                            };
                            Reference::EntityRef(reference_name)
                                .normalize_entity(entity_references.clone())
                        } else {
                            EntityValue::Value(val)
                        }
                    }
                    EntityValue::Reference(ref next_ref) => {
                        // Recursively resolve the next reference
                        next_ref.normalize_entity(entity_references.clone())
                    }
                    _ => entity_value,
                }
            }
            Reference::CharRef(value) => EntityValue::Value(value.clone()),
        }
    }

    pub(crate) fn normalize_attribute(
        &self,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> AttributeValue {
        match self {
            Reference::EntityRef(name) => {
                let refs_map = entity_references.borrow();
                match refs_map
                    .get(&(name.clone(), entity_source.clone()))
                    .cloned()
                {
                    Some(EntityValue::Value(val))
                        if refs_map.contains_key(&(
                            Name {
                                prefix: None,
                                local_part: val.clone(),
                            },
                            entity_source.clone(),
                        )) =>
                    {
                        let reference_name = Name {
                            prefix: None,
                            local_part: val,
                        };
                        Reference::EntityRef(reference_name)
                            .normalize_attribute(entity_references.clone(), entity_source.clone())
                    }
                    Some(EntityValue::Reference(Reference::EntityRef(entity))) => {
                        if let Some(EntityValue::Value(val)) = refs_map
                            .get(&(entity.clone(), EntitySource::Internal))
                            .cloned()
                        {
                            AttributeValue::Value(val)
                        } else {
                            Reference::EntityRef(entity.clone()).normalize_attribute(
                                entity_references.clone(),
                                EntitySource::External,
                            )
                        }
                    }
                    Some(entity_value) => {
                        // Convert EntityValue to AttributeValue
                        match entity_value {
                            EntityValue::Value(val) => AttributeValue::Value(val),
                            EntityValue::Reference(reference) => reference.normalize_attribute(
                                entity_references.clone(),
                                entity_source.clone(),
                            ),
                            EntityValue::Document(doc) => {
                                if let Document::Empty = doc {
                                    AttributeValue::EmptyExternalReference
                                } else {
                                    unimplemented!(
                                        "Unexpected Document variant to convert to AttributeValue"
                                    )
                                }
                            }
                            _ => panic!("Unexpected EntityValue variant"),
                        }
                    }
                    None => {
                        if entity_source == EntitySource::External {
                            if let Reference::EntityRef(_name) = &self {
                                AttributeValue::Reference(self.clone())
                            } else {
                                AttributeValue::Value(name.local_part.clone())
                            }
                        } else {
                            AttributeValue::Value(name.local_part.clone())
                        }
                    }
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
    fn parse_entity_ref(input: &str, _entity_source: EntitySource) -> IResult<&str, Reference> {
        let (input, reference) = map(
            tuple((char('&'), Self::parse_name, char(';'))),
            |(_, name, _)| Reference::EntityRef(name),
        )(input)?;
        Ok((input, reference))
    }

    //[69] PEReference ::= '%' Name ';'
    fn parse_parameter_reference(input: &str) -> IResult<&str, Reference> {
        let (input, output) = map(
            tuple((char('%'), Self::parse_name, char(';'))),
            |(_, name, _)| Reference::EntityRef(name),
        )(input)?;
        Ok((input, output))
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
