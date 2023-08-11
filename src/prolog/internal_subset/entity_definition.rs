use crate::{prolog::external_id::ExternalID, Name};

use super::entity_value::EntityValue;

#[derive(Clone, PartialEq)]
pub enum EntityDefinition<'a> {
    EntityValue(EntityValue<'a>),
    External {
        id: ExternalID<'a>,
        n_data: Option<Name<'a>>,
    },
}

impl<'a> EntityDefinition<'a> {
    pub fn get_entity_value(&self) -> Option<&EntityValue<'a>> {
        if let EntityDefinition::EntityValue(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_external_id(&self) -> Option<&ExternalID<'a>> {
        if let EntityDefinition::External { id, .. } = self {
            Some(id)
        } else {
            None
        }
    }
}
