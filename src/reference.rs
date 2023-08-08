// reference.rs

use std::{borrow::Cow, cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1},
    combinator::map,
    sequence::delimited,
    IResult,
};

use crate::{
    decode::{decode_digit, decode_hex},
    Name,
};
use crate::{parse::Parse, prolog::internal_subset::EntityValue};

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
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        alt((Self::parse_entity_ref, Self::parse_char_reference))(input)
    }
}

impl<'a> Reference<'a> {
    pub fn normalize(
        &self,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> Cow<'a, str> {
        match self {
            Reference::EntityRef(name) => {
                let refs_map = entity_references.borrow();
                if let Some(EntityValue::Value(value)) = refs_map.get(name) {
                    return value.clone();
                }
                Cow::Owned(name.local_part.to_string())
            }
            Reference::CharRef { value, .. } => Cow::Owned(value.to_string()),
        }
    }
}

impl<'a> ParseReference<'a> for Reference<'a> {}

#[derive(Clone, PartialEq)]
pub enum CharRefState {
    Decimal,
    Hexadecimal,
}

pub trait ParseReference<'a>: Parse<'a> {
    //[68] EntityRef ::= '&' Name ';'
    fn parse_entity_ref(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        //TODO: decode here?
        println!("\n-----\nPARSING ENTITY REFERENCE");
        let (input, _) = tag("&")(input)?;
        let (input, name) = Self::parse_name(input)?;
        let (input, _) = tag(";")(input)?;
        Ok((input, Reference::EntityRef(name)))
    }

    //[69] PEReference ::= '%' Name ';'
    fn parse_parameter_reference(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        let (input, _) = tag("%")(input)?;
        let (input, name) = Self::parse_name(input)?;
        let (input, _) = tag(";")(input)?;
        Ok((input, Reference::EntityRef(name)))
    }

    //[66] CharRef ::= '&#' [0-9]+ ';' | '&#x' [0-9a-fA-F]+ ';'
    fn parse_char_reference(input: &'a str) -> IResult<&'a str, Reference<'a>> {
        println!("\n-----\nPARSING CHAR REFERENCE");
        let (input, char_ref) = alt((
            map(
                delimited(tag("&#"), digit1, tag(";")),
                |digits_str: &str| {
                    let (_, decoded) = decode_digit("", digits_str).unwrap();
                    Reference::CharRef {
                        value: Cow::Owned(decoded.into_owned()),
                        state: CharRefState::Decimal,
                    }
                },
            ),
            map(
                delimited(tag("&#x"), hex_digit1, tag(";")),
                |hex_str: &str| {
                    let (_, decoded) = decode_hex("", hex_str).unwrap();
                    Reference::CharRef {
                        value: Cow::Owned(decoded.into_owned()),
                        state: CharRefState::Hexadecimal,
                    }
                },
            ),
        ))(input)?;
        println!("PARSED CHAR REFERENCE: {char_ref:?}");
        Ok((input, char_ref))
    }
}
