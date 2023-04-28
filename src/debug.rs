// debug.rs
use crate::{declaration::Declaration, Document, Tag, TagState};

use std::fmt::{self, Formatter};

fn fmt_indented(f: &mut String, indent: usize, s: &str) {
    f.push_str(&" ".repeat(indent));
    f.push_str(s);
}

impl<'a> Tag<'a> {
    fn fmt_indented(&self, f: &mut String, indent: usize) {
        match self {
            Tag::Tag {
                name,
                namespace,
                state,
            } => {
                fmt_indented(f, indent, &format!("Tag {{\n"));
                fmt_indented(f, indent + 4, &format!("name: \"{}\",\n", name));
                fmt_indented(f, indent + 4, &format!("namespace: {:?},\n", namespace));
                fmt_indented(f, indent + 4, &format!("state: {:?},\n", state));
                fmt_indented(f, indent, "},\n");
            }
        }
    }
}

impl<'a> Document<'a> {
    fn fmt_indented(&self, f: &mut String, indent: usize) {
        match self {
            Document::Declaration(declaration) => {
                fmt_indented(f, indent, "Declaration {\n");
                match declaration {
                    // Use .as_ref() to get a reference to the inner Declaration
                    Some(Declaration::DocType {
                        name,
                        external_id,
                        int_subset,
                    }) => {
                        fmt_indented(f, indent + 4, "DocType {\n");
                        fmt_indented(f, indent + 8, &format!("name: {:?},\n", name));
                        fmt_indented(f, indent + 8, &format!("external_id: {:?},\n", external_id));
                        fmt_indented(f, indent + 8, &format!("int_subset: {:?},\n", int_subset));
                        fmt_indented(f, indent + 4, "},\n");
                    }
                    Some(Declaration::Element { name, content_spec }) => {
                        fmt_indented(f, indent + 4, "Element {\n");
                        fmt_indented(f, indent + 8, &format!("name: {:?},\n", name));
                        fmt_indented(
                            f,
                            indent + 8,
                            &format!("content_spec: {:?},\n", content_spec),
                        );
                        fmt_indented(f, indent + 4, "},\n");
                    }

                    None => fmt_indented(f, indent + 4, "None,\n"),
                }
                fmt_indented(f, indent, "},\n");
            }
            Document::Element(tag1, document, tag2) => {
                fmt_indented(f, indent, "Element(\n");
                tag1.fmt_indented(f, indent + 4);
                f.push_str("\n");
                document.fmt_indented(f, indent + 4);
                f.push_str("\n");
                tag2.fmt_indented(f, indent + 4);
                f.push_str("\n");
                fmt_indented(f, indent, "),\n");
            }
            Document::Text(text) => {
                fmt_indented(f, indent, &format!("Text(\"{}\"),\n", text));
            }
            Document::Content(content) => {
                fmt_indented(
                    f,
                    indent,
                    &format!(
                        "Content(\"{}\"),\n",
                        content.clone().map_or("".to_string(), |c| c.to_string())
                    ),
                );
            }
            Document::Nested(documents) => {
                fmt_indented(f, indent, "Nested([\n");
                for document in documents.iter() {
                    document.fmt_indented(f, indent + 4);
                }
                f.push_str("\n");
                fmt_indented(f, indent, "]),\n");
            }
            Document::Comment(comment) => {
                fmt_indented(
                    f,
                    indent,
                    &format!(
                        "Comment(\"{}\"),\n",
                        comment.clone().map_or("".to_string(), |c| c.to_string())
                    ),
                );
            }
            Document::Empty => {
                fmt_indented(f, indent, "Empty,\n");
            }
        }
    }
}

impl<'a> fmt::Debug for Document<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented(&mut s, 0);
        write!(f, "{}", s)
    }
}

impl<'a> fmt::Debug for Tag<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented(&mut s, 0);
        write!(f, "{}", s)
    }
}
