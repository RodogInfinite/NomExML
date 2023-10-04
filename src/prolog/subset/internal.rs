
use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc, fs::File};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    combinator::{map, opt, map_res, verify, map_opt},
    multi::{fold_many0, many0, many1, fold_many1, separated_list0},
    sequence::{delimited, tuple},
    IResult, Parser,
};

use crate::{
    io::read_file,
    attribute::Attribute,
    namespaces::ParseNamespace,
    parse::Parse,
    processing_instruction::ProcessingInstruction,
    prolog::{
        declaration_content::DeclarationContent, external_id::ExternalID, id::ID,
        subset::{markup_declaration::MarkupDeclaration,entity_declaration::GeneralEntityDeclaration},
    },
    reference::{ParseReference, Reference},
    Document, Name, QualifiedName, ExternalEntityParseConfig, io::parse_external_ent_file, Config,
};

use super::{entity_declaration::{EntityDeclaration, EntityDecl}, entity_value::EntityValue, ParseDeclSep, entity_definition::EntityDefinition};






//TODO handle circular references in all entity replacements
#[derive(Clone, PartialEq)]
pub enum InternalSubset {
    MarkupDecl(MarkupDeclaration),
    DeclSep {
        reference: Reference,
        expansion: Option<Box<InternalSubset>>,
    },
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
    fn get_external_entity<'a>(
        entity_decl: EntityDecl,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
        config: Config,
    ) -> Result<(), nom::Err<nom::error::Error<&'a str>>> {
        if let Config {external_parse_config: ExternalEntityParseConfig { allow_ext_parse: true, base_directory, .. }} = &config {
            if let EntityDecl::Parameter(EntityDeclaration {
                name,
                entity_def: EntityDefinition::External {
                    id: ExternalID::System(ent_file),
                    ..
                },
            })| EntityDecl::General(EntityDeclaration {
                name,
                entity_def: EntityDefinition::External {
                    id: ExternalID::System(ent_file),
                    ..
                },
            }) = &entity_decl
            {
                let file_path = match base_directory {
                    Some(base) => format!("{}/{}", base, ent_file),
                    None => ent_file.clone(),
                };
                dbg!(&file_path);
                match File::open(file_path) {
                    Ok(mut file) => {
                        
                        match parse_external_ent_file(&mut file,  config.clone(),entity_references.clone()) {
                            Ok(parsed_entity_value) => {
                                dbg!(&parsed_entity_value);
                                match parsed_entity_value.as_slice() {
                                    [entity] => {
                                        dbg!(&entity);
                                        entity_references.borrow_mut().insert(name.clone(), entity.clone());
                                        dbg!(entity_references);
                                        Ok(())
                                    },
                                    _ => {dbg!("HERE0");Err(nom::Err::Error(nom::error::Error::new("", nom::error::ErrorKind::Fail)))}, 
                                }
                            },
                            Err(_) => {dbg!("HERE1");Err(nom::Err::Error(nom::error::Error::new("", nom::error::ErrorKind::Fail)))}, 
                        }
                    },
                    Err(_) => {dbg!("HERE2");Err(nom::Err::Error(nom::error::Error::new("", nom::error::ErrorKind::Fail)))}, 
                }
            } else {
                {dbg!("HERE3");Err(nom::Err::Error(nom::error::Error::new("", nom::error::ErrorKind::Fail)))} 
            }
        } else {
            Err(nom::Err::Error(nom::error::Error::new("", nom::error::ErrorKind::Fail))) 
        }
    }
}

impl<'a> ParseNamespace<'a> for InternalSubset {}

impl<'a> Parse<'a> for  InternalSubset {
    type Args = (Rc<RefCell<HashMap<Name, EntityValue>>>,Config);
    type Output = IResult<&'a str, Vec<InternalSubset>>;

    //[28b]	intSubset ::= (markupdecl | DeclSep)*
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        let(entity_references,config) = args;
        let (input, parsed) = separated_list0(
            |i| Self::parse_decl_sep(i, entity_references.clone()), 
            |i| {
                let (next_input, result) = MarkupDeclaration::parse(i, entity_references.clone())?;
                match result {
                    Some(markup_declaration) => Ok((next_input, InternalSubset::MarkupDecl(markup_declaration))),
                    None => Err(nom::Err::Error(nom::error::make_error(i, nom::error::ErrorKind::Verify))),
                }
            }
        )(input)?;
        
        
        
        dbg!(&parsed);
                                
        let mut consolidated: Vec<InternalSubset> = vec![];

        for mut internal_subset in parsed {

            if let InternalSubset::MarkupDecl(MarkupDeclaration::Entity(entity)) = internal_subset.clone() {
                
                let _ = Self::get_external_entity(entity.clone(), entity_references.clone(), config.clone());
                dbg!(&entity_references);    
                
                
               
            }; 
            
            //dbg!(&expanded_entities);

            if let InternalSubset::DeclSep { reference, expansion } = &mut internal_subset {
                dbg!(&reference);
                dbg!(&entity_references);
                if let Reference::EntityRef(name) = reference {
                    if let Some(EntityValue::MarkupDecl(inner_expansion)) = entity_references.borrow().get(name) {
                        *expansion = Some(Box::new(InternalSubset::MarkupDecl(*inner_expansion.clone())));
                    }
                }
            }
            
                
            

            if let InternalSubset::MarkupDecl(MarkupDeclaration::AttList { name, att_defs: Some(new_defs) }) = &internal_subset {
                if let Some(existing) = consolidated.iter_mut().find(|i| {
                    matches!(i, InternalSubset::MarkupDecl(MarkupDeclaration::AttList { name: existing_name, .. }) if existing_name == name)
                }) {
                    if let InternalSubset::MarkupDecl(MarkupDeclaration::AttList { att_defs: Some(existing_defs), .. }) = existing {
                        existing_defs.extend(new_defs.clone()); 
                    }
                    continue;
                }
            }

            consolidated.push(internal_subset.clone());

        }

        Ok((input, consolidated))
    }
}

impl ParseDeclSep for InternalSubset {
    type Output = Option<InternalSubset>;
    
    // [28a] DeclSep ::=  PEReference | S 
    fn parse_decl_sep(input: &str, entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>) -> IResult<&str, Self::Output> {
        alt((
            map(Reference::parse_parameter_reference, |reference| {
                let expansion = Self::expand_entity(&reference, &entity_references);
                let expanded_internal_subset = match &expansion {
                    Some(EntityValue::MarkupDecl(elem)) => Some(elem.clone()),
                    _ => None,
                };
                Some(InternalSubset::DeclSep {
                    reference,
                    expansion: expanded_internal_subset.map(|subset| Box::new(InternalSubset::MarkupDecl(*subset)))
                })
            }),
            map(Self::parse_multispace1, |_| None)
        ))(input)

    }        
}
