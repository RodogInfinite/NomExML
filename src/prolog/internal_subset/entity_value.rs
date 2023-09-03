// entity_value.rs

use std::borrow::Cow;

use crate::{reference::Reference, Document};

use super::InternalSubset;

#[derive(Clone, PartialEq)]
pub enum EntityValue<'a> {
    Document(Document<'a>),
    Value(Cow<'a, str>),
    Reference(Reference<'a>),
    ParameterReference(Reference<'a>),
    InternalSubset(Box<InternalSubset<'a>>),
}

impl<'a> EntityValue<'a> {
    pub fn get_value(&self) -> Option<Cow<'a, str>> {
        match self {
            EntityValue::Value(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_reference(&self) -> Option<&Reference<'a>> {
        if let EntityValue::Reference(reference) = self {
            Some(reference)
        } else {
            None
        }
    }

    pub fn get_perameter_reference(&self) -> Option<&Reference<'a>> {
        if let EntityValue::ParameterReference(reference) = self {
            Some(reference)
        } else {
            None
        }
    }
}
