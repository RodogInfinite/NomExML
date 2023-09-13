use crate::{
    namespaces::ParseNamespace,
    parse::Parse,
    prolog::internal_subset::{
        entity_declaration::{EntityDecl, EntityDeclaration},
        entity_definition::EntityDefinition,
    },
    Config, ExternalEntityParseConfig, Name,
};
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
pub struct DocType {
    pub name: Name,
    pub external_id: Option<ExternalID>,
    pub int_subset: Option<Vec<InternalSubset>>,
}

impl<'a> Parse<'a> for DocType {
    type Args = (Rc<RefCell<HashMap<Name, EntityValue>>>, Config);

    type Output = IResult<&'a str, Self>;

    // [28] doctypedecl ::= '<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>'
    // Namespaces (Third Edition) [16] doctypedecl ::= '<!DOCTYPE' S QName (S ExternalID)? S? ('[' (markupdecl | PEReference | S)* ']' S?)? '>'
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        //let (entity_references, external_parse_config) = args;
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
                // Iterate over the internal subset and normalize any references
                if !int_subset.is_empty() {
                    for item in &mut int_subset {
                        match item {
                            InternalSubset::Entity(EntityDecl::General(EntityDeclaration {
                                entity_def,
                                ..
                            }))
                            | InternalSubset::Entity(EntityDecl::Parameter(EntityDeclaration {
                                entity_def,
                                ..
                            })) => {
                                if let EntityDefinition::EntityValue(ev) = entity_def {
                                    if let EntityValue::Reference(ref ref_val) = *ev {
                                        let x = ref_val.normalize_entity(args.0.clone());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }

                Self {
                    name,
                    external_id,
                    int_subset: if int_subset.is_empty() {
                        None
                    } else {
                        Some(int_subset)
                    },
                }
            },
        )(input)
    }
}

//TODO integrate this
impl DocType {
    pub fn get_entities(&self) -> InternalSubset {
        let entities: Vec<_> = self.int_subset.as_ref().map_or(Vec::new(), |subset| {
            subset
                .iter()
                .filter_map(|item| {
                    if let InternalSubset::Entity(_) = item {
                        Some(Box::new(item.clone()))
                    } else {
                        None
                    }
                })
                .collect()
        });

        InternalSubset::Entities(entities)
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
