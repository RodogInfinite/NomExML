pub fn extract_content(&'a self) -> Option<&'a str> {
    match self {
        Document::Element(_, content, _) => content.extract_content(),
        Document::Content(Some(content)) => Some(content),
        _ => None,
    }
}

pub fn get_tags(&'a self, tag_name: &'a str) -> Elements<'a> {
    let mut results = Vec::new();
    self.get_internal_tags(tag_name, &mut results);
    Elements { tags: results }
}

pub fn get_internal_tags(&'a self, tag_name: &str, results: &mut Vec<&'a Self>) {
    match self {
        Document::Element(
            Tag {
                name, namespace, ..
            },
            content,
            _,
        ) => {
            if let Some(namespace) = namespace {
                if tag_name == (namespace.prefix.to_string() + ":" + name) {
                    results.push(self);
                }
            } else if name == tag_name {
                results.push(self);
            }
            content.get_internal_tags(tag_name, results);
        }
        Document::Nested(docs) => {
            let mut docs_iter = docs.iter();
            while let Some(doc) = docs_iter.next() {
                doc.get_internal_tags(tag_name, results);
            }
        }
        _ => (),
    }
}

impl<'a> Elements<'a> {
    pub fn extract_content(&self) -> Vec<Option<&'a str>> {
        self.tags.iter().map(|tag| tag.extract_content()).collect()
    }
}
