use crate::{prolog::external_id::ExternalID, Name};

use super::{entity_definition::EntityDefinition, entity_value::EntityValue};

#[derive(Clone, PartialEq)]
pub enum EntityDeclaration<'a> {
    General(GeneralEntityDeclaration<'a>),
    Parameter(ParameterEntityDefinition<'a>),
}

impl<'a> EntityDeclaration<'a> {
    pub fn get_general(&self) -> Option<&GeneralEntityDeclaration<'a>> {
        match self {
            EntityDeclaration::General(decl) => Some(decl),
            _ => None,
        }
    }

    pub fn get_parameter(&self) -> Option<&ParameterEntityDefinition<'a>> {
        match self {
            EntityDeclaration::Parameter(decl) => Some(decl),
            _ => None,
        }
    }

    pub fn find_general_entity_value(&self, name: Name<'a>) -> Option<&EntityValue<'a>> {
        if let EntityDeclaration::General(decl) = self {
            if decl.name == name {
                return decl.entity_def.get_entity_value();
            }
        }
        None
    }
}

#[derive(Clone, PartialEq)]
pub struct GeneralEntityDeclaration<'a> {
    pub name: Name<'a>,
    pub entity_def: EntityDefinition<'a>,
}

impl<'a> GeneralEntityDeclaration<'a> {
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

#[derive(Clone, Debug, PartialEq)]
pub enum ParameterEntityDefinition<'a> {
    EntityValue(EntityValue<'a>),
    ExternalID(ExternalID<'a>),
}
