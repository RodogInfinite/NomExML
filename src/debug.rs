use crate::{Element, Namespace, Tag};
use std::fmt::{self, Formatter};

fn fmt_indented(f: &mut String, indent: usize, s: &str) {
    f.push_str(&" ".repeat(indent));
    f.push_str(s);
}

impl<'a> Tag<'a> {
    fn fmt_indented(&self, f: &mut String, indent: usize) {
        match self {
            Tag::Open(tag) => {
                fmt_indented(f, indent, &format!("Open(\"{}\")", tag));
            }
            Tag::Close(tag) => {
                fmt_indented(f, indent, &format!("Close(\"{}\")", tag));
            }
            Tag::NS(namespace, tag) => {
                fmt_indented(f, indent, "NS(\n");
                namespace.fmt_indented(f, indent + 4);
                f.push_str(",\n");
                tag.fmt_indented(f, indent + 4);
                f.push_str("\n");
                fmt_indented(f, indent, "),"); // Aligns the closing parenthesis with "NS("
            }
        }
    }
}

impl<'a> Namespace<'a> {
    fn fmt_indented(&self, f: &mut String, indent: usize) {
        match self {
            Namespace::Prefix(prefix) => {
                fmt_indented(f, indent, &format!("Prefix(\"{}\")", prefix));
            }
            Namespace::URI(uri) => {
                fmt_indented(f, indent, &format!("URI(\"{}\")", uri));
            }
        }
    }
}

impl<'a> Element<'a> {
    fn fmt_indented(&self, f: &mut String, indent: usize) {
        match self {
            Element::Node(tag1, element, tag2) => {
                fmt_indented(f, indent, "Node(\n");
                tag1.fmt_indented(f, indent + 4);
                f.push_str("\n");
                element.fmt_indented(f, indent + 4);
                //f.push_str("\n");
                tag2.fmt_indented(f, indent + 4);
                f.push_str("\n");
                fmt_indented(f, indent, "),\n");
            }
            Element::Content(content) => {
                fmt_indented(f, indent, "Content(\"");
                f.push_str(&content);
                f.push_str("\"),\n");
            }
            Element::Nested(elements) => {
                fmt_indented(f, indent, "Nested([\n");
                for (_, element) in elements.iter().enumerate() {
                    element.fmt_indented(f, indent + 4);
                }
                f.push_str("\n");
                fmt_indented(f, indent, "]),\n");
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
