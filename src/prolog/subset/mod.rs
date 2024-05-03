use std::{cell::RefCell, collections::HashMap, fs::File, rc::Rc};

use nom::{branch::alt, combinator::map, multi::many0, IResult};

use crate::{
    error::CustomError,
    io::parse_external_entity_file,
    namespaces::ParseNamespace,
    parse::Parse,
    reference::{ParseReference, Reference},
    Config, ExternalEntityParseConfig, Name,
};

use self::{
    entity::entity_declaration::{EntityDecl, EntityDeclaration, GeneralEntityDeclaration},
    entity::entity_definition::EntityDefinition,
    entity::entity_value::EntityValue,
    entity::EntitySource,
    markup_declaration::MarkupDeclaration,
    subset::Subset,
};

use super::external_id::ExternalID;

pub mod entity;

pub mod markup_declaration;
pub mod subset;

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
