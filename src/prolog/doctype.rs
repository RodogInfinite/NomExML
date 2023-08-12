use crate::{namespaces::ParseNamespace, parse::Parse, Name};
use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{
    external_id::ExternalID,
    internal_subset::{entity_value::EntityValue, InternalSubset},
};

#[derive(Clone, PartialEq)]
pub struct DocType<'a> {
    pub name: Name<'a>,
    pub external_id: Option<ExternalID<'a>>,
    pub int_subset: Option<Vec<InternalSubset<'a>>>,
}

impl<'a> Parse<'a> for DocType<'a> {
    type Args = Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>;

    type Output = IResult<&'a str, Self>;

    // [28] doctypedecl ::= '<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>'
    // Namespaces (Third Edition) [16] doctypedecl ::= '<!DOCTYPE' S QName (S ExternalID)? S? ('[' (markupdecl | PEReference | S)* ']' S?)? '>'
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        dbg!(&input, "DocType::parse input");
        map(
            tuple((
                tag("<!DOCTYPE"),
                Self::parse_multispace1,
                Self::parse_name,
                opt(preceded(Self::parse_multispace1, |i| {
                    ExternalID::parse(i, ())
                })),
                Self::parse_multispace0,
                delimited(
                    pair(tag("["), Self::parse_multispace0),
                    |i| InternalSubset::parse(i, args.clone()),
                    pair(Self::parse_multispace0, tag("]")),
                ),
                Self::parse_multispace0,
                tag(">"),
                Self::parse_multispace0,
            )),
            |(_, _, name, external_id, _, int_subset, _, _, _)| Self {
                name,
                external_id,
                int_subset: if int_subset.is_empty() {
                    None
                } else {
                    Some(int_subset)
                },
            },
        )(input)
    }
}

//TODO integrate this
impl<'a> DocType<'a> {
    //TODO: figure out how to integrate this or remove
    fn _parse_qualified_doctype(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, DocType<'a>> {
        let (input, _) = tag("<!DOCTYPE")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_qualified_name(input)?;

        let (input, external_id) = opt(preceded(Self::parse_multispace1, |i| {
            ExternalID::parse(i, ())
        }))(input)?;

        let (input, _) = Self::parse_multispace0(input)?;

        let (input, int_subset) = opt(delimited(
            pair(tag("["), Self::parse_multispace0),
            |i| InternalSubset::parse(i, entity_references.clone()),
            pair(Self::parse_multispace0, tag("]")),
        ))(input)?;

        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag(">")(input)?;
        let (input, _) = Self::parse_multispace0(input)?;

        Ok((
            input,
            Self {
                name,
                external_id,
                int_subset,
            },
        ))
    }
}

impl<'a> ParseNamespace<'a> for DocType<'a> {}
