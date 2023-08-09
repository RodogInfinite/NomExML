use std::{cell::RefCell, collections::HashMap, rc::Rc};

use nom::{
    bytes::complete::tag,
    combinator::opt,
    sequence::{delimited, pair, preceded},
    IResult,
};

use crate::{namespaces::ParseNamespace, parse::Parse, Name};

use super::{
    external_id::ExternalID,
    internal_subset::{EntityValue, InternalSubset},
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
        println!("PARSING DOCTYPE");
        let (input, _) = tag("<!DOCTYPE")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_name(input)?;
        println!("PARSED NAME: {name:?}");
        let (input, external_id) = opt(preceded(Self::parse_multispace1, |i| {
            ExternalID::parse(i, ())
        }))(input)?;
        let (input, _) = Self::parse_multispace0(input)?;

        let (input, int_subset) = delimited(
            pair(tag("["), Self::parse_multispace0),
            |i| InternalSubset::parse(i, args.clone()), // Passing a None as there are no initial entity references
            pair(Self::parse_multispace0, tag("]")),
        )(input)
        .map(|(next_input, subset)| {
            (
                next_input,
                if subset.is_empty() {
                    None
                } else {
                    Some(subset)
                },
            )
        })?;

        println!("DOCTYPE INPUT AFTER PARSED INTERNAL SUBSET: {input}");
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag(">")(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        println!("PARSED DOCTYPE");
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
