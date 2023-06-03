use crate::{document::Document, namespaces::QualifiedName, tag::Tag};

pub enum Extracted<'a> {
    Documents(Vec<Document<'a>>),
    Content(HashMap<Cow<'a, str>, Vec<Document<'a>>>),
    QualifiedContent(Hashmap<QualifiedName<'a>, Document<'a>>),
}

pub trait Extract<'a>: Sized {
    fn extract_content(
        &'a mut self,
        tag_name: QualifiedName,
        content: Document<'a>,
    ) -> Extracted<'a> {
        let hashmap: Hashmap<Cow<'a, str>, Document<'a>> = HashMap::new();
        let qualified_hashmap: HashMap<QualifiedName<'a>, Document<'a>> = HashMap::new();
        // walk inner tags if they exist and insert them into the hashmap
        match tag_name {
            QualifiedName(None, local_part) => {
                hashmap.insert(local_part, content);
            }
            QualifiedName(Some(qualified_name), local_part) => {
                hashmap.insert(name, content);
            }
        }
    }
}
