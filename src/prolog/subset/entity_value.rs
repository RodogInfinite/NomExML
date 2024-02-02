// entity_value.rs

use crate::{reference::Reference, Document};

use super::markup_declaration::MarkupDeclaration;

#[derive(Clone, PartialEq)]
pub enum EntityValue {
    Document(Document),
    Value(String),
    Reference(Reference),
    ParameterReference(Reference),
    MarkupDecl(Box<MarkupDeclaration>),
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
