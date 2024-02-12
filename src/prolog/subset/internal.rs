use std::{cell::RefCell, collections::HashMap, fs::File, rc::Rc};

use nom::{branch::alt, combinator::map, multi::many0, IResult};

use crate::{
    error::CustomError,
    io::parse_external_entity_file,
    namespaces::ParseNamespace,
    parse::Parse,
    prolog::{external_id::ExternalID, subset::markup_declaration::MarkupDeclaration},
    reference::{ParseReference, Reference},
    Config, ExternalEntityParseConfig, Name,
};

use super::{
    entity_declaration::{EntityDecl, EntityDeclaration},
    entity_definition::EntityDefinition,
    entity_value::EntityValue,
    ParseDeclSep,
};

//TODO handle circular references in all entity replacements
#[derive(Clone, PartialEq)]
pub enum InternalSubset {
    MarkupDecl(MarkupDeclaration),
    DeclSep {
        reference: Reference,
        expansion: Option<Box<InternalSubset>>,
    },
    None,
}

impl InternalSubset {
    pub fn get_entity(&self) -> Option<&EntityDeclaration> {
        match self {
            InternalSubset::MarkupDecl(MarkupDeclaration::Entity(decl)) => match decl {
                EntityDecl::General(general_decl) => Some(general_decl),
                EntityDecl::Parameter(parameter_decl) => Some(parameter_decl),
            },
            _ => None,
        }
    }

    fn get_external_entity(
        entity_declaration: EntityDecl,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
        config: Config,
    ) -> Result<(), CustomError> {
        if let Config {
            external_parse_config:
                ExternalEntityParseConfig {
                    allow_ext_parse: true,
                    base_directory,
                    ..
                },
        } = &config
        {
            if let EntityDecl::Parameter(EntityDeclaration {
                name,
                entity_def:
                    EntityDefinition::External {
                        id: ExternalID::System(ent_file),
                        ..
                    },
            })
            | EntityDecl::General(EntityDeclaration {
                name,
                entity_def:
                    EntityDefinition::External {
                        id: ExternalID::System(ent_file),
                        ..
                    },
            }) = &entity_declaration
            {
                let file_path = match base_directory {
                    Some(base) => format!("{}/{}", base, ent_file),
                    None => ent_file.clone(),
                };
                Self::process_external_entity_file(file_path, name, config, entity_references)
            } else if let EntityDecl::General(EntityDeclaration {
                name,
                entity_def:
                    EntityDefinition::External {
                        id:
                            ExternalID::Public {
                                system_identifier, ..
                            },
                        ..
                    },
            }) = entity_declaration
            {
                if let ExternalID::System(system_identifier) = *system_identifier {
                    let file_path = match base_directory {
                        Some(base) => format!("{}/{}", base, system_identifier),
                        None => system_identifier.clone(),
                    };
                    Self::process_external_entity_file(file_path, &name, config, entity_references)
                } else {
                    Err(nom::Err::Error(nom::error::Error::new(
                        "Failed to match *system_identifier",
                        nom::error::ErrorKind::Fail,
                    ))
                    .into())
                }
            } else {
                Err(nom::Err::Error(nom::error::Error::new(
                    "Failed to match ExternalID::Public",
                    nom::error::ErrorKind::Fail,
                ))
                .into())
            }
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                "Failed to match &entity_declaration",
                nom::error::ErrorKind::Fail,
            ))
            .into())
        }
    }

    fn process_external_entity_file(
        file_path: String,
        name: &Name,
        config: Config,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> Result<(), CustomError> {
        match File::open(file_path) {
            Ok(mut file) => {
                match parse_external_entity_file(&mut file, config, entity_references.clone())
                    .as_deref()
                {
                    Ok([entity]) => {
                        entity_references
                            .borrow_mut()
                            .insert(name.clone(), entity.clone());
                        Ok(())
                    }
                    _ => Err(nom::Err::Error(nom::error::Error::new(
                        "Failed to match [entity] from `parse_external_entity_file`",
                        nom::error::ErrorKind::Fail,
                    ))
                    .into()),
                }
            }
            Err(e) => Err(CustomError::from(e)),
        }
    }
}

impl<'a> ParseNamespace<'a> for InternalSubset {}

impl<'a> Parse<'a> for InternalSubset {
    type Args = (Rc<RefCell<HashMap<Name, EntityValue>>>, Config);
    type Output = IResult<&'a str, Vec<InternalSubset>>;

    //[28b]	intSubset ::= (markupdecl | DeclSep)*
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        let (entity_references, config) = args;
        dbg!("HERE");
        let (input, parsed) = many0(alt((
            |i| {
                let (i, decl_sep) = Self::parse_decl_sep(i, entity_references.clone())?;
                match decl_sep {
                    Some(decl_sep) => Ok((i, Some(decl_sep))),
                    None => Ok((i, None)),
                }
            },
            |i| {
                let (i, result) = MarkupDeclaration::parse(i, entity_references.clone())?;
                match result {
                    Some(markup_declaration) => {
                        Ok((i, Some(InternalSubset::MarkupDecl(markup_declaration))))
                    }
                    None => Err(nom::Err::Error(nom::error::make_error(
                        input,
                        nom::error::ErrorKind::Verify,
                    ))),
                }
            },
        )))(input)?;

        dbg!(&parsed);

        let mut consolidated: Vec<InternalSubset> = vec![];
        for mut internal_subset in parsed {
            if let Some(InternalSubset::MarkupDecl(MarkupDeclaration::Entity(entity))) =
                internal_subset.clone()
            {
                let _ = Self::get_external_entity(
                    entity.clone(),
                    entity_references.clone(),
                    config.clone(),
                );
            };

            if let Some(InternalSubset::DeclSep {
                reference: Reference::EntityRef(name),
                expansion,
            }) = &mut internal_subset
            {
                if let Some(EntityValue::MarkupDecl(inner_expansion)) =
                    entity_references.borrow().get(name)
                {
                    *expansion = Some(Box::new(InternalSubset::MarkupDecl(
                        *inner_expansion.clone(),
                    )));
                }
                if let Some(InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                    name,
                    att_defs: Some(new_defs),
                })) = expansion.as_deref()
                {
                    if let Some(InternalSubset::MarkupDecl(MarkupDeclaration::AttList { att_defs: Some(existing_defs), .. })) = consolidated.iter_mut().find(|i| {
                            matches!(i, InternalSubset::MarkupDecl(MarkupDeclaration::AttList { name: existing_name, .. }) if existing_name == name)
                        }) {
                            existing_defs.extend(new_defs.clone());
                            continue;
                        }
                    consolidated.push(InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                        name: name.clone(),
                        att_defs: Some(new_defs.clone()),
                    }));
                }
            }

            if let Some(InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                name,
                att_defs: Some(new_defs),
            })) = &internal_subset
            {
                if let Some(existing) = consolidated.iter_mut().find(|i| {
                    matches!(i, InternalSubset::MarkupDecl(MarkupDeclaration::AttList { name: existing_name, .. }) if existing_name == name)
                }) {
                    if let InternalSubset::MarkupDecl(MarkupDeclaration::AttList { att_defs: Some(existing_defs), .. }) = existing {
                        existing_defs.extend(new_defs.clone());
                    }
                    continue
                 }
            }

            if let Some(internal_subset) = internal_subset {
                consolidated.push(internal_subset);
            };
        }
        Ok((input, consolidated))
    }
}

impl ParseDeclSep for InternalSubset {
    type Output = Option<InternalSubset>;

    // [28a] DeclSep ::=  PEReference | S
    fn parse_decl_sep(
        input: &str,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> IResult<&str, Self::Output> {
        let (input, decl_sep) = alt((
            map(Reference::parse_parameter_reference, |reference| {
                let expansion = Self::expand_entity(&reference, &entity_references);
                let expanded_internal_subset = match &expansion {
                    Some(EntityValue::MarkupDecl(elem)) => Some(elem.clone()),
                    _ => None,
                };
                Some(Self::DeclSep {
                    reference,
                    expansion: expanded_internal_subset
                        .map(|subset| Box::new(Self::MarkupDecl(*subset))),
                })
            }),
            map(Self::parse_multispace1, |_| None),
        ))(input)?;
        Ok((input, decl_sep))
    }
}
