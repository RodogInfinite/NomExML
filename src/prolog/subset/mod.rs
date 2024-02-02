use std::{cell::RefCell, collections::HashMap, rc::Rc};

use nom::IResult;

use crate::{reference::Reference, Name};

use self::entity_value::EntityValue;

pub mod entity_declaration;
pub mod entity_definition;
pub mod entity_value;
pub mod external_entity;
pub mod internal;
pub mod markup_declaration;

pub trait ParseDeclSep {
    type Output;
    // [28a] DeclSep ::=  PEReference | S
    fn parse_decl_sep(
        input: &str,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> IResult<&str, Self::Output>;
    fn expand_entity(
        reference: &Reference,
        entity_references: &Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> Option<EntityValue> {
        match reference {
            Reference::EntityRef(name) => {
                let entities = entity_references.borrow();
                entities.get(name).cloned()
            }
            Reference::CharRef(_) => {
                // Handle character references here if needed
                None
            }
        }
    }
}
