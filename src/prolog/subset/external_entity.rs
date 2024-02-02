use crate::{
    prolog::{
        external_id::ExternalID,
        subset::entity_declaration::{EntityDecl, EntityDeclaration},
    },
    Document,
};

use super::{
    entity_definition::EntityDefinition, internal::InternalSubset,
    markup_declaration::MarkupDeclaration,
};
pub fn get_external_entity(doc: Document) -> Option<String> {
    fn extract_external_id(entity_decl: &EntityDeclaration) -> Option<String> {
        if let EntityDefinition::External {
            id: ExternalID::System(value),
            ..
        } = &entity_decl.entity_def
        {
            Some(value.clone())
        } else {
            None
        }
    }

    match doc {
        Document::Prolog {
            doc_type: Some(doctype),
            ..
        } => {
            if let Some(subsets) = doctype.int_subset {
                for subset in subsets {
                    if let InternalSubset::MarkupDecl(MarkupDeclaration::Entity(entity_enum)) =
                        subset
                    {
                        match entity_enum {
                            EntityDecl::General(entity_decl)
                            | EntityDecl::Parameter(entity_decl) => {
                                if let Some(value) = extract_external_id(&entity_decl) {
                                    return Some(value);
                                }
                            }
                        }
                    }
                }
            }
        }
        Document::Nested(docs) => {
            for inner_doc in docs {
                if let Document::Prolog {
                    doc_type: Some(doctype),
                    ..
                } = inner_doc
                {
                    if let Some(subsets) = doctype.int_subset {
                        for subset in subsets {
                            if let InternalSubset::MarkupDecl(MarkupDeclaration::Entity(
                                entity_enum,
                            )) = subset
                            {
                                match entity_enum {
                                    EntityDecl::General(entity_decl)
                                    | EntityDecl::Parameter(entity_decl) => {
                                        if let Some(value) = extract_external_id(&entity_decl) {
                                            return Some(value);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    None
}
