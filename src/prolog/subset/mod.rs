use std::{cell::RefCell, collections::HashMap, rc::Rc};

use nom::IResult;

use crate::{reference::Reference, Document, Name};

use self::{
    entity::entity_declaration::{EntityDecl, EntityDeclaration},
    entity::entity_value::EntityValue,
    entity::EntitySource,
};

pub mod entity;

pub mod markup_declaration;

use nom::{branch::alt, combinator::map, multi::many0};

use crate::{
    attribute::Attribute, namespaces::ParseNamespace, parse::Parse,
    prolog::subset::markup_declaration::MarkupDeclaration, reference::ParseReference, Config,
};

//TODO handle circular references in all entity replacements
#[derive(Clone, PartialEq)]
pub enum Subset {
    MarkupDecl(MarkupDeclaration),
    DeclSep {
        reference: Reference,
        expansion: Option<Box<Subset>>,
    },
    None,
}

impl Subset {
    pub fn get_entity(&self) -> Option<&EntityDeclaration> {
        match self {
            Subset::MarkupDecl(MarkupDeclaration::Entity(decl)) => match decl {
                EntityDecl::General(general_decl) => Some(general_decl),
                EntityDecl::Parameter(parameter_decl) => Some(parameter_decl),
            },
            _ => None,
        }
    }
}

impl<'a> ParseNamespace<'a> for Subset {}

impl<'a> Parse<'a> for Subset {
    type Args = (
        Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        Config,
        EntitySource,
    );
    type Output = IResult<&'a str, Vec<Subset>>;

    //[28b]	intSubset ::= (markupdecl | DeclSep)*
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        let (entity_references, config, entity_source) = args;

        let (input, parsed) = many0(alt((
            |i| {
                let (i, decl_sep) =
                    Self::parse_decl_sep(i, entity_references.clone(), entity_source.clone())?;

                match decl_sep {
                    Some(decl_sep) => Ok((i, Some(decl_sep))),
                    None => Ok((i, None)),
                }
            },
            |i| {
                let (i, result) = MarkupDeclaration::parse(
                    i,
                    (entity_references.clone(), entity_source.clone()),
                )?;
                match result {
                    Some(markup_declaration) => {
                        Ok((i, Some(Subset::MarkupDecl(markup_declaration))))
                    }
                    None => Err(nom::Err::Error(nom::error::make_error(
                        input,
                        nom::error::ErrorKind::Verify,
                    ))),
                }
            },
        )))(input)?;
        let mut consolidated: Vec<Subset> = vec![];
        for mut subset in parsed {
            if let Some(Subset::MarkupDecl(MarkupDeclaration::Entity(entity))) = subset.clone() {
                let _ = Document::get_external_entity(
                    entity.clone(),
                    entity_references.clone(),
                    config.clone(),
                );
            };
            if let Some(Subset::DeclSep {
                reference: Reference::EntityRef(name),
                expansion,
            }) = &mut subset
            {
                if let Some(EntityValue::MarkupDecl(inner_expansion)) = entity_references
                    .borrow()
                    .get(&(name.clone(), EntitySource::Internal))
                {
                    let mut modified_inner_expansion = *inner_expansion.clone();

                    if let MarkupDeclaration::AttList {
                        ref mut att_defs, ..
                    } = modified_inner_expansion
                    {
                        if let Some(ref mut defs) = att_defs {
                            // Iterate over each attribute definition in att_defs and modify the source to EntitySource::External.
                            for attribute in defs {
                                if let Attribute::Definition { ref mut source, .. } = attribute {
                                    *source = EntitySource::Internal;
                                }
                            }
                        }
                    }

                    *expansion = Some(Box::new(Subset::MarkupDecl(
                        modified_inner_expansion.clone(),
                    )));
                }
                if let Some(EntityValue::MarkupDecl(inner_expansion)) = entity_references
                    .borrow()
                    .get(&(name.clone(), EntitySource::External))
                {
                    let mut modified_inner_expansion = *inner_expansion.clone();

                    if let MarkupDeclaration::AttList {
                        ref mut att_defs, ..
                    } = modified_inner_expansion
                    {
                        if let Some(ref mut defs) = att_defs {
                            // Iterate over each attribute definition in att_defs and modify the source to EntitySource::External.
                            for attribute in defs {
                                if let Attribute::Definition { ref mut source, .. } = attribute {
                                    *source = EntitySource::External;
                                }
                            }
                        }
                    }

                    *expansion = Some(Box::new(Subset::MarkupDecl(
                        modified_inner_expansion.clone(),
                    )));
                }

                if let Some(Subset::MarkupDecl(MarkupDeclaration::AttList {
                    name,
                    att_defs: Some(new_defs),
                })) = expansion.as_deref()
                {
                    if let Some(Subset::MarkupDecl(MarkupDeclaration::AttList { att_defs: Some(existing_defs), .. })) = consolidated.iter_mut().find(|i| {
                            matches!(i, Subset::MarkupDecl(MarkupDeclaration::AttList { name: existing_name, .. }) if existing_name == name)
                        }) {
                            existing_defs.extend(new_defs.clone());
                            continue;
                        }
                    consolidated.push(Subset::MarkupDecl(MarkupDeclaration::AttList {
                        name: name.clone(),
                        att_defs: Some(new_defs.clone()),
                    }));
                }
            }

            if let Some(Subset::MarkupDecl(MarkupDeclaration::AttList {
                name,
                att_defs: Some(new_defs),
            })) = &subset
            {
                if let Some(existing) = consolidated.iter_mut().find(|i| {
                    matches!(i, Subset::MarkupDecl(MarkupDeclaration::AttList { name: existing_name, .. }) if existing_name == name)
                }) {
                    if let Subset::MarkupDecl(MarkupDeclaration::AttList { att_defs: Some(existing_defs), .. }) = existing {
                        existing_defs.extend(new_defs.clone());
                    }
                    continue
                 }
            }

            if let Some(subset) = subset {
                consolidated.push(subset);
            };
        }
        Ok((input, consolidated))
    }
}

impl ParseDeclSep for Subset {
    type Output = Option<Subset>;

    // [28a] DeclSep ::=  PEReference | S
    fn parse_decl_sep(
        input: &str,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> IResult<&str, Self::Output> {
        let (input, decl_sep) = alt((
            map(Reference::parse_parameter_reference, |reference| {
                let expansion =
                    Self::expand_entity(&reference, &entity_references, entity_source.clone());

                let expanded_subset = match &expansion {
                    Some(EntityValue::MarkupDecl(elem)) => Some(elem.clone()),
                    _ => None,
                };

                Some(Subset::DeclSep {
                    reference,
                    expansion: expanded_subset.map(|subset| Box::new(Subset::MarkupDecl(*subset))),
                })
            }),
            map(Self::parse_multispace1, |_| None),
        ))(input)?;
        Ok((input, decl_sep))
    }
}

pub trait ParseDeclSep {
    type Output;
    // [28a] DeclSep ::=  PEReference | S
    fn parse_decl_sep(
        input: &str,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> IResult<&str, Self::Output>;
    fn expand_entity(
        reference: &Reference,
        entity_references: &Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> Option<EntityValue> {
        match reference {
            Reference::EntityRef(name) => {
                let entities = entity_references.borrow();
                entities
                    .get(&(name.clone(), entity_source.clone()))
                    .cloned()
            }
            Reference::CharRef(_) => {
                // Handle character references here if needed
                None
            }
        }
    }
}
