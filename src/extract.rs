use std::{borrow::Cow, collections::HashMap};

use crate::{namespaces::ParseNamespace, tag::Tag, Document, QualifiedName};

pub enum Extracted<'a> {
    Documents(Vec<Document<'a>>),
    Content(HashMap<Cow<'a, str>, Vec<Document<'a>>>),
    QualifiedContent(HashMap<QualifiedName<'a>, Document<'a>>),
}
