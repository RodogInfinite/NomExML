// debug.rs
use crate::Element;
use std::fmt::{self, Formatter};

fn fmt_indented(f: &mut String, indent: usize, s: &str) {
    f.push_str(&" ".repeat(indent));
    f.push_str(s);
}

impl<'a> Element<'a> {
    fn fmt_indented(&self, f: &mut String, indent: usize) {
        match self {
            Element::DocType {
                name,
                external_id,
                int_subset,
            } => {
                fmt_indented(f, indent, "DocType {\n");
                fmt_indented(f, indent + 4, &format!("name: \"{}\",\n", name));
                fmt_indented(f, indent + 4, &format!("external_id: {:?},\n", external_id));
                fmt_indented(f, indent + 4, &format!("int_subset: {:?},\n", int_subset));
                fmt_indented(f, indent, "},\n");
            }
            Element::Tag {
                open,
                close,
                name,
                namespace,
            } => {
                fmt_indented(f, indent, "Tag {\n");
                fmt_indented(f, indent + 4, &format!("open: {},\n", open));
                fmt_indented(f, indent + 4, &format!("close: {},\n", close));
                fmt_indented(f, indent + 4, &format!("name: \"{}\",\n", name));
                fmt_indented(f, indent + 4, &format!("namespace: {:?},\n", namespace));
                fmt_indented(f, indent, "},\n");
            }
            Element::Node(tag1, element, tag2) => {
                fmt_indented(f, indent, "Node(\n");
                tag1.fmt_indented(f, indent + 4);
                f.push_str("\n");
                element.fmt_indented(f, indent + 4);
                f.push_str("\n");
                tag2.fmt_indented(f, indent + 4);
                f.push_str("\n");
                fmt_indented(f, indent, "),\n");
            }
            Element::Content(content) => {
                fmt_indented(f, indent, "Content(\"");
                f.push_str(content.unwrap_or(""));
                f.push_str("\"),\n");
            }
            Element::Nested(elements) => {
                fmt_indented(f, indent, "Nested([\n");
                for element in elements.iter() {
                    element.fmt_indented(f, indent + 4);
                }
                f.push_str("\n");
                fmt_indented(f, indent, "]),\n");
            }
            Element::Comment(comment) => {
                fmt_indented(f, indent, "Comment(\"");
                f.push_str(comment.unwrap_or(""));
                f.push_str("\"),\n");
            }
        }
    }
}

impl<'a> fmt::Debug for Element<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented(&mut s, 0);
        write!(f, "{}", s)
    }
}
