fn get_external_entity(doc: Document) -> Option<String> {
    match doc {
        Document::Prolog {
            doc_type: Some(doctype),
            ..
        } => {
            if let Some(subsets) = doctype.int_subset {
                for subset in subsets {
                    if let InternalSubset::Entity(entity_decl) = subset {
                        if let EntityDefinition::External {
                            id: ExternalID::System(value),
                            ..
                        } = entity_decl.entity_def
                        {
                            return Some(value);
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
                            if let InternalSubset::Entity(entity_decl) = subset {
                                if let EntityDefinition::External {
                                    id: ExternalID::System(value),
                                    ..
                                } = entity_decl.entity_def
                                {
                                    return Some(value);
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
