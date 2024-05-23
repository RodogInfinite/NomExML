use crate::{
    prolog::{external_id::ExternalID, textdecl::TextDecl},
    Name,
};

use super::entity_value::EntityValue;

#[derive(Clone, PartialEq, Eq)]
pub enum EntityDefinition {
    EntityValue(EntityValue),
    External {
        id: ExternalID,
        n_data: Option<Name>,
        text_decl: Option<TextDecl>,
    },
}

impl EntityDefinition {
    pub fn get_entity_value(&self) -> Option<&EntityValue> {
        if let EntityDefinition::EntityValue(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_external_id(&self) -> Option<&ExternalID> {
        if let EntityDefinition::External { id, .. } = self {
            Some(id)
        } else {
            None
        }
    }
}
