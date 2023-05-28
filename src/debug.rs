// debug.rs
use crate::{
    attribute::Attribute,
    prolog::{ContentParticle, Prolog, DeclarationContent, Mixed, InternalSubset, XmlDecl, DocType},
    document::{Document,ProcessingInstruction}, Tag,
};

use std::fmt::{self, Formatter};

fn fmt_indented(f: &mut String, indent: usize, s: &str) {
    f.push_str(&" ".repeat(indent));
    f.push_str(s);
}

impl<'a> Tag<'a> {
    fn fmt_indented_tag(&self, f: &mut String, indent: usize) {
        match self {
            Tag {
                name,
                namespace,
                attributes,
                state,
            } => {
                fmt_indented(f, indent, &format!("Tag {{\n"));
                fmt_indented(f, indent + 4, &format!("name: \"{}\",\n", name));
                fmt_indented(f, indent + 4, &format!("namespace: {:?},\n", namespace));
                fmt_indented(f, indent + 4, &format!("attributes: {:?},\n", attributes));
                fmt_indented(f, indent + 4, &format!("state: {:?},\n", state));
                fmt_indented(f, indent, "},\n");
            }
        }
    }
}

impl<'a> Document<'a> {
    fn fmt_indented_doc(&self, f: &mut String, indent: usize) {
        match self {
            Document::Prolog(prolog) => {
                fmt_indented(f, indent, "Prolog {\n");
                match prolog {
                    Some(d) => {
                        d.fmt_indented_prolog(f, indent + 4);
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
            Document::ProcessingInstruction(ProcessingInstruction { target, data }) => {
                fmt_indented(f, indent, "ProcessingInstruction {\n");
                fmt_indented(f, indent + 4, &format!("target: \"{}\",\n", target));
                fmt_indented(
                    f,
                    indent + 4,
                    &match data {
                        Some(c) => format!("data: \"{}\",\n", c),
                        None => "data: None,\n".to_string(),
                    },
                );
                fmt_indented(f, indent, "},\n");
            }

            Document::CDATA(cdata) => {
                fmt_indented(
                    f,
                    indent,
                    &format!("CDATA(\"{}\"),\n", cdata.clone().as_ref()),
                );
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
                fmt_indented(f, indent + 4, "children:");
                if let Some(children) = children {
                    fmt_indented(f, indent + 4, "[\n");
                    for child in children.iter() {
                        child.fmt_indented_content_particle(f, indent + 8);
                    }
                    fmt_indented(f, indent + 4, "],\n");
                }
                if let None = children {
                    f.push_str(" None,\n")
                }
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
                zero_or_more,
            } => {
                fmt_indented(f, indent, "PCDATA {\n");
                fmt_indented(f, indent + 4, &format!("names: {:?},\n", names));
                fmt_indented(f, indent + 4, &format!("parsed: {:?},\n", parsed));
                fmt_indented(
                    f,
                    indent + 4,
                    &format!("zero_or_more: {:?},\n", zero_or_more),
                );
                fmt_indented(f, indent, "},\n");
            }
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
                fmt_indented(
                    f,
                    indent + 4,
                    &format!("conditional_state: {:?},\n", conditional_state),
                );
                fmt_indented(f, indent, "},\n");
            }
        }
    }
}


impl<'a> XmlDecl<'a> {
    fn fmt_indented_xml_decl(&self, f: &mut String, indent: usize) {
        fmt_indented(f, indent, "XmlDecl {\n");
        fmt_indented(f, indent + 4, &format!("version: {:?},\n", self.version));
        fmt_indented(f, indent + 4, &format!("encoding: {:?},\n", self.encoding));
        fmt_indented(f, indent + 4, &format!("standalone: {:?},\n", self.standalone));
        fmt_indented(f, indent, "},\n");
    }
}

impl<'a> DocType<'a> {
    fn fmt_indented_doc_type(&self, f: &mut String, indent: usize) {
        fmt_indented(f, indent, "DocType {\n");
        fmt_indented(f, indent + 4, &format!("name: {:?},\n", self.name));
        fmt_indented(f, indent + 4, &format!("external_id: {:?},\n", self.external_id));
        fmt_indented(f, indent + 4, "int_subset: Some([\n");
        for element in self.int_subset.as_ref().unwrap_or(&Vec::new()).iter() {
            element.fmt_internal_subset(f, indent + 8); // please ensure fmt_internal_subset method exists
        }
        fmt_indented(f, indent + 4, "]),\n");
        fmt_indented(f, indent, "},\n");
    }
}


impl<'a> Prolog<'a> {
    fn fmt_indented_prolog(&self, f: &mut String, indent: usize) {
        fmt_indented(f, indent, "Prolog {\n");
        if let Some(xml_decl) = self.xml_decl.as_ref() {
            xml_decl.fmt_indented_xml_decl(f, indent + 4);
        }
        if let Some(doc_type) = self.doc_type.as_ref() {
            doc_type.fmt_indented_doc_type(f, indent + 4);
        }
        fmt_indented(f, indent, "},\n");
    }
}

impl<'a> InternalSubset<'a> {
    fn fmt_internal_subset(&self, f: &mut String,indent:usize) {
        match self {
            InternalSubset::Element { name, content_spec } => {
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
            InternalSubset::AttList { name, att_defs } => {
                fmt_indented(f, indent, "AttList {\n");
                fmt_indented(f, indent + 4, &format!("name: {:?},\n", name));
                fmt_indented(f, indent + 4, "att_def: [\n");
                if let Some(def) = att_defs {
                    for def_item in def.iter() {
                        def_item.fmt_indented_attribute(f, indent + 8);
                    }
                }
                fmt_indented(f, indent + 4, "],\n");
                fmt_indented(f, indent, "},\n");
            }
            InternalSubset::DeclSep { name } => {
                fmt_indented(f, indent, "DeclSep {\n");
                fmt_indented(f, indent + 4, &format!("name: {:?},\n", name));
                fmt_indented(f, indent, "},\n");
            }
            InternalSubset::ProcessingInstruction(ProcessingInstruction { target, data }) => {
                fmt_indented(f, indent, "ProcessingInstruction {\n");
                fmt_indented(f, indent + 4, &format!("target: {:?},\n", target));
                fmt_indented(f, indent + 4, &format!("data: {:?},\n", data));
                fmt_indented(f, indent, "},\n");
            }
        }
    }  
}

impl<'a> fmt::Debug for InternalSubset<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_internal_subset(&mut s, 0);
        write!(f, "{}", s)
    }
}

impl<'a> std::fmt::Debug for XmlDecl<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XmlDecl")
            .field("version", &self.version)
            .field("encoding", &self.encoding)
            .field("standalone", &self.standalone)
            .finish()
    }
}

impl<'a> std::fmt::Debug for DocType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DocType")
            .field("name", &self.name)
            .field("external_id", &self.external_id)
            .field("int_subset", &self.int_subset)
            .finish()
    }
}

impl<'a> std::fmt::Debug for Prolog<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Prolog")
            .field("xml_decl", &self.xml_decl)
            .field("doc_type", &self.doc_type)
            .finish()
    }
}


impl<'a> Attribute<'a> {
    fn fmt_indented_attribute(&self, f: &mut String, indent: usize) {
        match self {
            Attribute::Definition {
                name,
                att_type,
                default_decl,
            } => {
                fmt_indented(f, indent, "Definition {\n");
                fmt_indented(f, indent + 4, &format!("name: {:?},\n", name));
                fmt_indented(f, indent + 4, &format!("att_type: {:?},\n", att_type));
                fmt_indented(
                    f,
                    indent + 4,
                    &format!("default_decl: {:?},\n", default_decl),
                );
                fmt_indented(f, indent, "},\n");
            }
            Attribute::Reference { entity, char } => {
                fmt_indented(f, indent, "Reference {\n");
                fmt_indented(f, indent + 4, &format!("entity: {:?},\n", entity));
                fmt_indented(f, indent + 4, &format!("char: {:?},\n", char));
                fmt_indented(f, indent, "},\n");
            }
            Attribute::Required => {
                fmt_indented(f, indent, "REQUIRED,\n");
            }
            Attribute::Implied => {
                fmt_indented(f, indent, "IMPLIED,\n");
            }
            Attribute::Instance { name, value } => {
                fmt_indented(f, indent, "Instance {\n");
                fmt_indented(f, indent + 4, &format!("name: {:?},\n", name));
                fmt_indented(f, indent + 4, &format!("value: {:?},\n", value));
                fmt_indented(f, indent, "},\n");
            }
        }
    }
}

impl<'a> fmt::Debug for Attribute<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_attribute(&mut s, 0);
        write!(f, "{}", s)
    }
}
