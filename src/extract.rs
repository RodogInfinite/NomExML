use crate::{document::Document, namespaces::QualifiedName, tag::Tag};

pub enum Extracted<'a> { // Document::Nested([Document::Element])
    Documents(Vec<Document<'a>>),
    
}

or pub struct Extracted<'a> { // Document::Nested([Document::Element])

}

pub trait Extract<'a>: Sized {
    fn extract_content(&'a self) -> Option<&'a str> {
        unimplemented!()
    }

    fn get_tags(&'a self, tag_name: &'a str) -> Self {
        unimplemented!()
    }

    fn get_internal_tags(&'a self, tag_name: &str, results: &mut Vec<&'a Self>) {
        unimplemented!()
    }
}
