use crate::{
    namespaces::ParseNamespace,
    parse::Parse,
    prolog::subset::{
        entity_declaration::EntityDecl, entity_definition::EntityDefinition,
        entity_value::EntityValue, internal::InternalSubset,
    },
    Config, Name,
};
use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{external_id::ExternalID, subset::markup_declaration::MarkupDeclaration};

#[derive(Clone, PartialEq)]
pub struct DocType {
    pub name: Name,
    pub external_id: Option<ExternalID>,
    pub int_subset: Option<Vec<InternalSubset>>,
}

impl<'a> Parse<'a> for DocType {
    type Args = (Rc<RefCell<HashMap<Name, EntityValue>>>, Config);

    type Output = IResult<&'a str, Self>;

    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        map(
            tuple((
                tag("<!DOCTYPE"),
                Self::parse_multispace1,
                Self::parse_name,
                opt(preceded(Self::parse_multispace1, |i| {
                    ExternalID::parse(i, ())
                })),
                Self::parse_multispace0,
                opt(delimited(
                    pair(tag("["), Self::parse_multispace0),
                    |i| InternalSubset::parse(i, args.clone()),
                    pair(Self::parse_multispace0, tag("]")),
                )),
                Self::parse_multispace0,
                tag(">"),
                Self::parse_multispace0,
            )),
            |(
                _open_tag,
                _whitespace1,
                name,
                external_id,
                _open_bracket_whitespace,
                mut int_subset,
                _whitespace2,
                _close_tag,
                _whitespace3,
            )| {
                if let Some(int_subset) = &mut int_subset {
                    int_subset.iter_mut().for_each(|item| {
                        if let InternalSubset::MarkupDecl(MarkupDeclaration::Entity(
                            EntityDecl::General(entity_decl),
                        ))
                        | InternalSubset::MarkupDecl(MarkupDeclaration::Entity(
                            EntityDecl::Parameter(entity_decl),
                        )) = item
                        {
                            if let EntityDefinition::EntityValue(EntityValue::Reference(ref_val)) =
                                &mut entity_decl.entity_def
                            {
                                ref_val.normalize_entity(args.0.clone());
                            }
                        }
                    })
                };
                Self {
                    name,
                    external_id,
                    int_subset,
                }
            },
        )(input)
    }
}

//TODO integrate this
impl DocType {
    pub fn extract_entities(&self) -> Option<Vec<Box<InternalSubset>>> {
        let entities: Vec<_> = self
            .int_subset
            .as_ref()?
            .iter()
            .filter_map(|item| {
                if let InternalSubset::MarkupDecl(MarkupDeclaration::Entity(_)) = item {
                    Some(Box::new(item.clone()))
                } else {
                    None
                }
            })
            .collect();

        if entities.is_empty() {
            None
        } else {
            Some(entities)
        }
    }
    //TODO: figure out how to integrate this or remove
    // fn _parse_qualified_doctype(
    //     input: &str,
    //     entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    // ) -> IResult<&str, DocType> {
    //     let (input, _) = tag("<!DOCTYPE")(input)?;
    //     let (input, _) = Self::parse_multispace1(input)?;
    //     let (input, name) = Self::parse_qualified_name(input)?;

    //     let (input, external_id) = opt(preceded(Self::parse_multispace1, |i| {
    //         ExternalID::parse(i, ())
    //     }))(input)?;

    //     let (input, _) = Self::parse_multispace0(input)?;

    //     let (input, int_subset) = opt(delimited(
    //         pair(tag("["), Self::parse_multispace0),
    //         |i| InternalSubset::parse(i, entity_references.clone()),
    //         pair(Self::parse_multispace0, tag("]")),
    //     ))(input)?;

    //     let (input, _) = Self::parse_multispace0(input)?;
    //     let (input, _) = tag(">")(input)?;
    //     let (input, _) = Self::parse_multispace0(input)?;

    //     Ok((
    //         input,
    //         Self {
    //             name,
    //             external_id,
    //             int_subset,
    //         },
    //     ))
    // }
}

impl<'a> ParseNamespace<'a> for DocType {}
