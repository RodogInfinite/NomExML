use crate::{prolog::external_id::ExternalID, Name};

use super::{entity_definition::EntityDefinition, entity_value::EntityValue};

#[derive(Clone, PartialEq)]
pub enum EntityDecl {
    General(GeneralEntityDeclaration),
    Parameter(ParameterEntityDeclaration),
}

#[derive(Clone, PartialEq)]
pub struct EntityDeclaration {
    pub name: Name,
    pub entity_def: EntityDefinition,
}
pub type GeneralEntityDeclaration = EntityDeclaration;
pub type ParameterEntityDeclaration = EntityDeclaration;

impl EntityDeclaration {
    pub fn find_name(&self, name: Name) -> Option<&GeneralEntityDeclaration> {
        if self.name == name {
            Some(self)
        } else {
            None
        }
    }

    pub fn get_name(&self) -> &Name {
        &self.name
    }

    pub fn get_entity_def(&self) -> &EntityDefinition {
        &self.entity_def
    }
}
