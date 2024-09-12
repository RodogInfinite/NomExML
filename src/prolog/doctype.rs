use crate::{
    namespaces::ParseNamespace,
    parse::Parse,
    prolog::subset::{
        entity::{
            entity_declaration::EntityDecl, entity_definition::EntityDefinition,
            entity_value::EntityValue,
        },
        Subset,
    },
    Config, IResult, Name,
};
use nom::{
    bytes::complete::tag,
    combinator::opt,
    sequence::{delimited, pair, preceded, tuple},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{
    external_id::ExternalID,
    subset::{entity::EntitySource, markup_declaration::MarkupDeclaration},
};

#[derive(Clone, PartialEq, Eq)]
pub struct DocType {
    pub name: Name,
    pub external_id: Option<ExternalID>,
    pub subset: Option<Vec<Subset>>,
}

impl<'a> Parse<'a> for DocType {
    type Args = (
        Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        &'a Config,
    );

    type Output = IResult<&'a str, Self>;

    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        let (entity_references, config) = args;
        let mut merged_subsets = vec![];
        let (input, (_open_tag, _whitespace1, name, external_id, _whitespace2)) = tuple((
            tag("<!DOCTYPE"),
            Self::parse_multispace1,
            Self::parse_name,
            opt(preceded(Self::parse_multispace1, |i| {
                ExternalID::parse(i, ())
            })),
            Self::parse_multispace0,
        ))(input)?;
        if let Some(external_id) = external_id {
            let mut external_subsets = match external_id.get_external_entity_from_id(
                input,
                entity_references.clone(),
                config,
            ) {
                Ok(subsets) => subsets,
                Err(_) => None,
            };
            let (input, (mut subset, _whitespace3, _close_tag, _whitespace4)) =
                tuple((
                    opt(delimited(
                        pair(tag("["), Self::parse_multispace0),
                        |i| {
                            Subset::parse(
                                i,
                                (entity_references.clone(), config, EntitySource::External),
                            )
                        },
                        pair(Self::parse_multispace0, tag("]")),
                    )),
                    Self::parse_multispace0,
                    tag(">"),
                    Self::parse_multispace0,
                ))(input)?;
            if let Some(subset) = &mut subset {
                subset.iter_mut().for_each(|subset| match subset {
                    Subset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                        entity_decl,
                    )))
                    | Subset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::Parameter(
                        entity_decl,
                    ))) => {
                        if let EntityDefinition::EntityValue(EntityValue::Reference(ref_val)) =
                            &mut entity_decl.entity_def
                        {
                            ref_val.normalize_entity(entity_references.clone());
                        }
                    }

                    _ => {}
                });
                merged_subsets.extend(subset.clone());
            }
            if let Some(subset) = &mut external_subsets {
                subset.iter_mut().for_each(|subset| match subset {
                    Subset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                        entity_decl,
                    )))
                    | Subset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::Parameter(
                        entity_decl,
                    ))) => {
                        if let EntityDefinition::EntityValue(EntityValue::Reference(ref_val)) =
                            &mut entity_decl.entity_def
                        {
                            ref_val.normalize_entity(entity_references.clone());
                        }
                    }
                    _ => {}
                });
                merged_subsets.extend(subset.clone());
            }
            // we need to create a subsets that merges external subsets with subset
            if merged_subsets.is_empty() {
                Ok((
                    input,
                    Self {
                        name,
                        external_id: Some(external_id),
                        subset: None,
                    },
                ))
            } else {
                Ok((
                    input,
                    Self {
                        name,
                        external_id: Some(external_id),
                        subset: Some(merged_subsets),
                    },
                ))
            }
        } else {
            let (input, (mut subset, _whitespace3, _close_tag, _whitespace4)) =
                tuple((
                    opt(delimited(
                        pair(tag("["), Self::parse_multispace0),
                        |i| {
                            Subset::parse(
                                i,
                                (entity_references.clone(), config, EntitySource::Internal),
                            )
                        },
                        pair(Self::parse_multispace0, tag("]")),
                    )),
                    Self::parse_multispace0,
                    tag(">"),
                    Self::parse_multispace0,
                ))(input)?;
            if let Some(subset) = &mut subset {
                subset.iter_mut().for_each(|subset| {
                    match subset {
                        //match internal_subset {
                        Subset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            entity_decl,
                        )))
                        | Subset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::Parameter(
                            entity_decl,
                        ))) => {
                            if let EntityDefinition::EntityValue(EntityValue::Reference(ref_val)) =
                                &mut entity_decl.entity_def
                            {
                                ref_val.normalize_entity(entity_references.clone());
                            }
                        }

                        _ => {}
                    }
                });
            }
            Ok((
                input,
                Self {
                    name,
                    external_id,
                    subset,
                },
            ))
        }
    }
}

//TODO integrate this
impl DocType {
    pub fn extract_entities(&self) -> Option<Vec<Box<Subset>>> {
        let entities: Vec<_> = self
            .subset
            .as_ref()?
            .iter()
            .filter_map(|item| {
                if let Subset::MarkupDecl(MarkupDeclaration::Entity(_)) = item {
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
    //     entity_references: Rc<RefCell<HashMap<(Name,EntityType), EntityValue>>>,
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
