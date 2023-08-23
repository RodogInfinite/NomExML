use crate::{prolog::external_id::ExternalID, Name};

use super::{entity_definition::EntityDefinition, entity_value::EntityValue};

#[derive(Clone, PartialEq)]
pub enum EntityDecl<'a> {
    General(GeneralEntityDeclaration<'a>),
    Parameter(ParameterEntityDeclaration<'a>),
}

#[derive(Clone, PartialEq)]
pub struct EntityDeclaration<'a> {
    pub name: Name<'a>,
    pub entity_def: EntityDefinition<'a>,
}
pub type GeneralEntityDeclaration<'a> = EntityDeclaration<'a>;
pub type ParameterEntityDeclaration<'a> = EntityDeclaration<'a>;

impl<'a> EntityDeclaration<'a> {
    pub fn find_name(&self, name: Name<'a>) -> Option<&GeneralEntityDeclaration<'a>> {
        if self.name == name {
            Some(self)
        } else {
            None
        }
    }

    pub fn get_name(&self) -> &Name<'a> {
        &self.name
    }

    pub fn get_entity_def(&self) -> &EntityDefinition<'a> {
        &self.entity_def
    }
}
