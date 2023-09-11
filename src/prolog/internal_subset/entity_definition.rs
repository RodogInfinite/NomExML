use crate::{prolog::external_id::ExternalID, Name};

use super::entity_value::EntityValue;

#[derive(Clone, PartialEq)]
pub enum EntityDefinition {
    EntityValue(EntityValue),
    External {
        id: ExternalID,
        n_data: Option<Name>,
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
