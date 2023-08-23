// reference.rs

use crate::{
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
    CharRef {
        value: Cow<'a, str>,
        state: CharRefState,
    },
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
    pub fn normalize(
        &self,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> EntityValue<'a> {
        match self {
            Reference::EntityRef(name) => {
                let refs_map = entity_references.borrow();
                dbg!(&name);

                if let Some(entity_value) = refs_map.get(name) {
                    match entity_value {
                        EntityValue::Document(doc) => {
                            return EntityValue::Document(doc.clone());
                        }
                        EntityValue::Value(val) => {
                            return EntityValue::Value(val.clone());
                        }
                        EntityValue::Reference(ref_val) => {
                            return EntityValue::Reference(ref_val.clone());
                        }
                        EntityValue::ParameterReference(param_ref_val) => {
                            return EntityValue::ParameterReference(param_ref_val.clone());
                        }
                    }
                }

                // If we can't find a matching entity value in the map, you can default to some behavior.
                // Here, I'm returning the name as a Value.
                EntityValue::Value(Cow::Owned(name.local_part.to_string()))
            }
            Reference::CharRef { value, .. } => EntityValue::Value(Cow::Owned(value.to_string())),
        }
    }
}

impl<'a> ParseReference<'a> for Reference<'a> {}
impl<'a> Decode for Reference<'a> {
    fn as_str(&self) -> &str {
        match self {
            Reference::EntityRef(name) => &name.local_part,
            Reference::CharRef { value, .. } => value,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum CharRefState {
    Decimal,
    Hexadecimal,
}

pub trait ParseReference<'a>: Parse<'a> + Decode {
    //[68] EntityRef ::= '&' Name ';'
    fn parse_entity_ref(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        map(
            tuple((char('&'), Self::parse_name, char(';'))),
            |(_, name, _)| Reference::EntityRef(name),
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
                    Reference::CharRef {
                        value: Cow::Owned(decoded),
                        state: CharRefState::Decimal,
                    }
                },
            ),
            map(
                tuple((tag("&#x"), hex_digit1, tag(";"))),
                |(start, hex, end): (&str, &str, &str)| {
                    let reconstructed = format!("{}{}{}", start, hex, end);
                    let decoded = reconstructed.decode().unwrap().into_owned();
                    Reference::CharRef {
                        value: Cow::Owned(decoded),
                        state: CharRefState::Hexadecimal,
                    }
                },
            ),
        ))(input)
    }
}
