use std::{borrow::Cow, collections::HashMap};

use crate::{tag::Tag, Document, QualifiedName};

pub enum Extracted<'a> {
    Documents(Vec<Document<'a>>),
    Content(HashMap<Cow<'a, str>, Vec<Document<'a>>>),
    QualifiedContent(HashMap<QualifiedName<'a>, Document<'a>>),
}

pub trait Extract<'a>: Sized {
    fn extract(document: &Document<'a>) -> HashMap<QualifiedName<'a>, Vec<Document<'a>>> {
        let mut result = HashMap::new();

        match document {
            Document::Prolog { .. } => {} // Handle prolog if needed
            Document::Element(start_tag, inner_doc, end_tag) => {
                let mut hashmap: HashMap<Cow<'a, str>, Vec<Document<'a>>> = HashMap::new();
                let mut qualified_hashmap: HashMap<QualifiedName<'a>, Document<'a>> =
                    HashMap::new();

                match start_tag {
                    Tag {
                        name:
                            QualifiedName {
                                prefix: None,
                                local_part,
                            },
                        ..
                    } => {
                        hashmap
                            .entry(local_part.clone())
                            .or_insert_with(Vec::new)
                            .push((**inner_doc).clone());
                        for (key, value) in hashmap {
                            result.insert(
                                QualifiedName {
                                    prefix: None,
                                    local_part: key,
                                },
                                value,
                            );
                        }
                    }
                    Tag { name, .. } => {
                        qualified_hashmap.insert(name.clone(), (**inner_doc).clone());
                        for (key, value) in qualified_hashmap {
                            result.insert(key, vec![value]);
                        }
                    }
                }

                // Recursive call for nested documents
                let nested_result = Self::extract(&*inner_doc);
                for (key, mut value) in nested_result.into_iter() {
                    result
                        .entry(key)
                        .or_insert_with(Vec::new)
                        .append(&mut value);
                }
            }
            Document::Nested(docs) => {
                for doc in docs {
                    let nested_result = Self::extract(doc);
                    for (key, mut value) in nested_result.into_iter() {
                        result
                            .entry(key)
                            .or_insert_with(Vec::new)
                            .append(&mut value);
                    }
                }
            }
            _ => {} // Handle other Document variants if needed
        }

        result
    }

    //pub fn extract_prolog() {}
}
