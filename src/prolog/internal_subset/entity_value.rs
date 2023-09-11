// entity_value.rs

use std::borrow::Cow;

use crate::{reference::Reference, Document};

use super::InternalSubset;

#[derive(Clone, PartialEq)]
pub enum EntityValue {
    Document(Document),
    Value(String),
    Reference(Reference),
    ParameterReference(Reference),
    InternalSubset(Box<InternalSubset>),
}

impl EntityValue {
    pub fn get_value(&self) -> Option<String> {
        match self {
            EntityValue::Value(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_reference(&self) -> Option<&Reference> {
        if let EntityValue::Reference(reference) = self {
            Some(reference)
        } else {
            None
        }
    }

    pub fn get_perameter_reference(&self) -> Option<&Reference> {
        if let EntityValue::ParameterReference(reference) = self {
            Some(reference)
        } else {
            None
        }
    }
}
