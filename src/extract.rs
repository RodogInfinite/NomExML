use crate::{document::Document, namespaces::QualifiedName, tag::Tag};

pub enum Extracted<'a> {
    Documents(Vec<Document<'a>>),
    Content(HashMap<Cow<'a, str>, Vec<Document<'a>>>),
    QualifiedContent(QualifiedName<'a>, Vec<Document<'a>>),
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
