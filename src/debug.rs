// debug.rs
use crate::{declaration::{Declaration, DeclarationContent, Mixed, ContentParticle}, Document, Tag, TagState};

use std::fmt::{self, Formatter};

fn fmt_indented(f: &mut String, indent: usize, s: &str) {
    f.push_str(&" ".repeat(indent));
    f.push_str(s);
}

impl<'a> Tag<'a> {
    fn fmt_indented_tag(&self, f: &mut String, indent: usize) {
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
    fn fmt_indented_doc(&self, f: &mut String, indent: usize) {
        match self {
            Document::Declaration(declaration) => {
                fmt_indented(f, indent, "Declaration {\n");
                match declaration {
                    Some(d) => {
                        d.fmt_indented_declaration(f, indent + 4);
                    }
                    None => fmt_indented(f, indent + 4, "None,\n"),
                }
                fmt_indented(f, indent, "},\n");
            }
            Document::Element(tag1, document, tag2) => {
                fmt_indented(f, indent, "Element(\n");
                tag1.fmt_indented_tag(f, indent + 4);
                document.fmt_indented_doc(f, indent + 4);
                tag2.fmt_indented_tag(f, indent + 4);
                fmt_indented(f, indent, "),\n");
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
                    document.fmt_indented_doc(f, indent + 4);
                }
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
        self.fmt_indented_doc(&mut s, 0);
        write!(f, "{}\n", s)
    }
}

impl<'a> fmt::Debug for Tag<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_tag(&mut s, 0);
        write!(f, "{}", s)
    }
}


impl<'a> DeclarationContent<'a> {
    fn fmt_indented_dec_content(&self, f: &mut String, indent: usize) {
        match self {
            DeclarationContent::Spec { mixed, children } => {
                fmt_indented(f, indent, "Spec {\n");
                mixed.fmt_indented_mixed(f, indent + 4);
                fmt_indented(f, indent + 4, "children: [\n");
                for child in children.as_ref().unwrap_or(&Vec::new()).iter() {
                    child.fmt_indented_content_particle(f, indent + 8);
                }
                fmt_indented(f, indent + 4, "],\n");
                fmt_indented(f, indent, "},");
            }
        }
    }
}


impl<'a> fmt::Debug for DeclarationContent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_dec_content(&mut s, 0);
        write!(f, "{}\n", s)
    }
}


impl<'a> Mixed<'a> {
    fn fmt_indented_mixed(&self, f: &mut String, indent: usize) {
        match self {
            Mixed::PCDATA {
                names,
                parsed,
                conditional_state,
            } => {
                fmt_indented(f, indent, "PCDATA {\n");
                fmt_indented(f, indent + 4, &format!("names: {:?},\n", names));
                fmt_indented(f, indent + 4, &format!("parsed: {:?},\n", parsed));
                fmt_indented(f, indent + 4, &format!("conditional_state: {:?},\n", conditional_state));
                fmt_indented(f, indent, "},\n");
            },
        }
    }
}

impl<'a> fmt::Debug for Mixed<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_mixed(&mut s, 0);
        write!(f, "{}", s)
    }
}
impl<'a> ContentParticle<'a> {
    fn fmt_indented_content_particle(&self, f: &mut String, indent: usize) {
        match self {
            ContentParticle::Particle {
                names,
                choice,
                sequence,
                conditional_state,
            } => {
                fmt_indented(f, indent, "Particle {\n");
                if let Some(names) = names {
                    fmt_indented(f, indent + 4, &format!("names: {:?},\n", names));
                }
                if let Some(choice) = choice {
                    fmt_indented(f, indent + 4, "choice: [\n");
                    for item in choice {
                        item.fmt_indented_content_particle(f, indent + 8);
                    }
                    fmt_indented(f, indent + 4, "],\n");
                }
                if let Some(sequence) = sequence {
                    fmt_indented(f, indent + 4, "sequence: [\n");
                    for item in sequence {
                        item.fmt_indented_content_particle(f, indent + 8);
                    }
                    fmt_indented(f, indent + 4, "],\n");
                }
                fmt_indented(f, indent + 4, &format!("conditional_state: {:?},\n", conditional_state));
                fmt_indented(f, indent, "},\n");
            }
        }
    }
}

impl<'a> Declaration<'a> {
    fn fmt_indented_declaration(&self, f: &mut String, indent: usize) {
        match self {
            Declaration::DocType {
                name,
                external_id,
                int_subset,
            } => {
                fmt_indented(f, indent, "DocType {\n");
                fmt_indented(f, indent + 4, &format!("name: {:?},\n", name));
                fmt_indented(f, indent + 4, &format!("external_id: {:?},\n", external_id));
                fmt_indented(f, indent + 4, "int_subset: Some([\n");
                for element in int_subset.as_ref().unwrap_or(&Vec::new()).iter() {
                    element.fmt_indented_declaration(f, indent + 8);
                }
                fmt_indented(f, indent + 4, "]),\n");
                fmt_indented(f, indent, "},\n");
            }
            Declaration::Element { name, content_spec } => {
                fmt_indented(f, indent, "Element {\n");
                fmt_indented(f, indent + 4, &format!("name: {:?},\n", name));
                fmt_indented(f, indent + 4, "content_spec: ");
                match content_spec {
                    Some(spec) => {
                        let mut s = String::new();
                        spec.fmt_indented_dec_content(&mut s, indent + 8);
                        f.push_str(&format!("Some(\n{}\n", s));
                        fmt_indented(f, indent + 4, "),\n");
                    }
                    None => f.push_str("None,\n"),
                }
                fmt_indented(f, indent, "},\n");
            }
        }
    }
}
