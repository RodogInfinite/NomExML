// debug.rs
use crate::{
    attribute::{Attribute, AttributeValue, Prefix},
    misc::{Misc, MiscState},
    processing_instruction::ProcessingInstruction,
    prolog::{
        content_particle::ContentParticle,
        declaration_content::{DeclarationContent, Mixed},
        doctype::DocType,
        external_id::ExternalID,
        id::ID,
        subset::{
            entity::{
                entity_declaration::{EntityDecl, EntityDeclaration},
                entity_definition::EntityDefinition,
                entity_value::EntityValue,
            },
            markup_declaration::MarkupDeclaration,
            Subset,
        },
        textdecl::TextDecl,
        xmldecl::{Standalone, XmlDecl},
    },
    reference::Reference,
    Document, Name, Tag,
};
use std::fmt::{self, Formatter};

fn fmt_indented(f: &mut String, indent: usize, s: &str) {
    f.push_str(&" ".repeat(indent));
    f.push_str(s);
}
impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_tag(&mut s, 0);
        write!(f, "{}", s)
    }
}
impl Tag {
    fn fmt_indented_tag(&self, f: &mut String, indent: usize) {
        let Tag {
            name,
            attributes,
            state,
        } = self;

        fmt_indented(f, indent, "Tag {\n");
        fmt_indented(
            f,
            indent + 4,
            &format!("name: \n{}\n", name.fmt_qualified_name(indent + 8)),
        );
        fmt_indented(f, indent + 4, "attributes: ");

        match attributes {
            Some(attrs) => {
                let mut s = String::new();
                for attr in attrs {
                    attr.fmt_indented_attribute(&mut s, indent + 8);
                }
                f.push_str(&format!("Some([\n{}", s));
                fmt_indented(f, indent + 4, "]),\n");
            }
            None => f.push_str("None,\n"),
        }

        fmt_indented(f, indent + 4, &format!("state: {:?},\n", state));
        fmt_indented(f, indent, "},\n");
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fmt_qualified_name(0))
    }
}

impl Name {
    fn fmt_qualified_name(&self, indent: usize) -> String {
        let Name { prefix, local_part } = self;
        let mut f = String::new();

        fmt_indented(&mut f, indent, "Name {\n");

        match prefix {
            Some(p) => {
                fmt_indented(&mut f, indent + 4, &format!("prefix: Some(\"{}\"),\n", p));
            }
            None => {
                fmt_indented(&mut f, indent + 4, "prefix: None,\n");
            }
        }
        fmt_indented(
            &mut f,
            indent + 4,
            &format!("local_part: \"{}\",\n", local_part),
        );
        fmt_indented(&mut f, indent, "}");

        f
    }
}

impl fmt::Debug for MiscState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_misc_state(&mut s, 0);
        write!(f, "{}", s)
    }
}

impl MiscState {
    fn fmt_indented_misc_state(&self, f: &mut String, _indent: usize) {
        match self {
            MiscState::BeforeDoctype => {
                f.push_str("BeforeDoctype");
            }
            MiscState::AfterDoctype => {
                f.push_str("AfterDoctype");
            }
        }
    }
}

impl fmt::Debug for Misc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_misc(&mut s, 0);
        write!(f, "{}", s)
    }
}

impl Misc {
    fn fmt_indented_misc(&self, f: &mut String, indent: usize) {
        fmt_indented(f, indent, "Misc {\n");
        fmt_indented(f, indent + 4, &format!("content: {:?}", self.content));
        fmt_indented(f, indent + 4, &format!("state: {:?},\n", self.state));
        fmt_indented(f, indent, "},\n");
    }
}

impl fmt::Debug for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_doc(&mut s, 0);
        writeln!(f, "{}", s)
    }
}

impl Document {
    fn fmt_indented_doc(&self, f: &mut String, indent: usize) {
        match self {
            Document::Prolog {
                xml_decl,
                misc,
                doc_type,
            } => {
                if xml_decl.is_some() || misc.is_some() || doc_type.is_some() {
                    fmt_indented(f, indent, "Prolog {\n");
                    if let Some(xml_decl) = xml_decl {
                        xml_decl.fmt_indented_xml_decl(f, indent + 4);
                    }

                    if let Some(misc_vec) = misc {
                        for misc in misc_vec {
                            misc.fmt_indented_misc(f, indent + 4);
                        }
                    }
                    if let Some(doc_type) = doc_type {
                        doc_type.fmt_indented_doc_type(f, indent + 4);
                    }
                    fmt_indented(f, indent, "},\n");
                }
            }
            Document::Element(tag1, document, tag2) => {
                fmt_indented(f, indent, "Document::Element(\n");
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
                        content.clone().map_or("".to_string(), |c| c)
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
                fmt_indented(f, indent, &format!("Comment(\"{}\"),\n", comment));
            }
            Document::Empty => {
                fmt_indented(f, indent, "Empty");
            }
            Document::EmptyTag(tag) => {
                fmt_indented(f, indent, "EmptyTag(\n");
                tag.fmt_indented_tag(f, indent + 4);
                fmt_indented(f, indent, "),\n");
            }

            Document::ProcessingInstruction(ProcessingInstruction { target, data }) => {
                fmt_indented(f, indent, "ProcessingInstruction {\n");
                fmt_indented(f, indent + 4, &format!("target: \"{:?}\",\n", target));
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
                fmt_indented(f, indent, &format!("CDATA(\"{}\"),\n", cdata.clone()));
            }
        }
    }
}

impl DeclarationContent {
    fn fmt_indented_dec_content(&self, f: &mut String, indent: usize) {
        match self {
            DeclarationContent::Mixed(mixed) => {
                fmt_indented(f, indent, " DeclarationContent::Mixed(\n");
                mixed.fmt_indented_mixed(f, indent + 4);
                fmt_indented(f, indent, "),\n");
            }
            DeclarationContent::Children(children) => {
                fmt_indented(f, indent, " DeclarationContent::Children (\n");
                let mut s = String::new();
                children.fmt_indented_content_particle(&mut s, indent);
                fmt_indented(f, indent, &format!("{}\n", s));
                fmt_indented(f, indent, "),\n");
            }
            DeclarationContent::Empty => {
                fmt_indented(f, indent, "DeclarationContent::Empty,\n");
            }
            DeclarationContent::Any => {
                fmt_indented(f, indent, "DeclarationContent::Any,\n");
            }
        }
    }
}

impl fmt::Debug for DeclarationContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_dec_content(&mut s, 0);
        writeln!(f, "{}", s)
    }
}

impl Mixed {
    fn fmt_indented_mixed(&self, f: &mut String, indent: usize) {
        match self {
            Mixed::PCDATA => {
                fmt_indented(f, indent, "PCDATA,\n");
            }
            Mixed::Names(names) => {
                fmt_indented(f, indent, "Names([\n");
                for name in names {
                    let formatted_name = name.fmt_qualified_name(indent + 4); // Assuming you have this function
                    f.push_str(&format!("{},\n", formatted_name));
                }
                fmt_indented(f, indent, "]),\n");
            }
        }
    }
}

impl fmt::Debug for Mixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_mixed(&mut s, 0);
        write!(f, "{}", s)
    }
}

impl ContentParticle {
    fn fmt_indented_content_particle(&self, f: &mut String, indent: usize) {
        match self {
            ContentParticle::Name(name, conditional_state) => {
                fmt_indented(f, indent, "ContentParticle::Name {\n");
                fmt_indented(f, indent + 4, "name: \n");
                let formatted_name = name.fmt_qualified_name(indent + 8);
                f.push_str(&formatted_name);
                f.push('\n');
                fmt_indented(
                    f,
                    indent + 4,
                    &format!("conditional_state: {:?},\n", conditional_state),
                );
                fmt_indented(f, indent, "},\n");
            }
            ContentParticle::Choice(particles, conditional_state) => {
                fmt_indented(f, indent, "ContentParticle::Choice {\n");
                fmt_indented(f, indent + 4, "particles: [\n");
                for item in particles {
                    item.fmt_indented_content_particle(f, indent + 8);
                }
                fmt_indented(f, indent + 4, "],\n");
                fmt_indented(
                    f,
                    indent + 4,
                    &format!("conditional_state: {:?},\n", conditional_state),
                );
                fmt_indented(f, indent, "},\n");
            }
            ContentParticle::Sequence(particles, conditional_state) => {
                fmt_indented(f, 4, "ContentParticle::Sequence {\n");
                fmt_indented(f, indent + 8, "particles: [\n");
                for item in particles {
                    item.fmt_indented_content_particle(f, indent + 12);
                }
                fmt_indented(f, indent + 8, "],\n");
                fmt_indented(
                    f,
                    indent + 4,
                    &format!("conditional_state: {:?},\n", conditional_state),
                );
                fmt_indented(f, indent + 4, "},\n");
            }
        }
    }
}

impl fmt::Debug for Standalone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Standalone::Yes => write!(f, "Yes"),
            Standalone::No => write!(f, "No"),
        }
    }
}
impl TextDecl {
    fn _fmt_indented_text_decl(&self, f: &mut String, indent: usize) {
        fmt_indented(f, indent, "TextDecl {\n");
        fmt_indented(f, indent + 4, &format!("version: {:?},\n", self.version));
        fmt_indented(f, indent + 4, &format!("encoding: {:?},\n", self.encoding));
        fmt_indented(f, indent, "},\n");
    }
}

impl XmlDecl {
    fn fmt_indented_xml_decl(&self, f: &mut String, indent: usize) {
        fmt_indented(f, indent, "XmlDecl {\n");
        fmt_indented(f, indent + 4, &format!("version: {:?},\n", self.version));
        fmt_indented(f, indent + 4, &format!("encoding: {:?},\n", self.encoding));
        fmt_indented(
            f,
            indent + 4,
            &format!("standalone: {:?},\n", self.standalone),
        );
        fmt_indented(f, indent, "},\n");
    }
}

impl std::fmt::Debug for DocType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DocType")
            .field("name", &self.name)
            .field("external_id", &self.external_id)
            .field("subset", &self.subset)
            .finish()
    }
}

impl DocType {
    fn fmt_indented_doc_type(&self, f: &mut String, indent: usize) {
        fmt_indented(f, indent, "DocType {\n");
        fmt_indented(
            f,
            indent + 4,
            &format!("name: \n{}\n", self.name.fmt_qualified_name(indent + 8)),
        );
        fmt_indented(
            f,
            indent + 4,
            &format!("external_id: {:?},\n", self.external_id),
        );
        fmt_indented(f, indent + 4, "subset: Some([\n");
        for element in self.subset.as_ref().unwrap_or(&Vec::new()).iter() {
            element.fmt_subset(f, indent + 8);
        }
        fmt_indented(f, indent + 4, "]),\n");
        fmt_indented(f, indent, "},\n");
    }
}
impl ExternalID {
    fn fmt_indented_external_id(&self, f: &mut String, indent: usize) {
        match self {
            ExternalID::System(system) => {
                fmt_indented(f, indent, &format!("System({:?}),\n", system));
            }
            ExternalID::Public {
                pubid,
                system_identifier,
            } => {
                fmt_indented(f, indent, "Public {\n");
                fmt_indented(f, indent + 4, &format!("pubid: {:?},\n", pubid));
                fmt_indented(f, indent + 4, "system_identifier: ");
                system_identifier.fmt_indented_external_id(f, indent + 8);
                fmt_indented(f, indent, "},\n");
            }
        }
    }
}

impl std::fmt::Debug for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ID::ExternalID(external_id) => f.debug_tuple("ExternalID").field(&external_id).finish(),
            ID::PublicID(pubid_literal) => f.debug_tuple("PublicID").field(&pubid_literal).finish(),
        }
    }
}

impl ID {
    fn fmt_indented_id(&self, f: &mut String, indent: usize) {
        match self {
            ID::ExternalID(external_id) => {
                fmt_indented(f, indent, "ExternalID {\n");
                external_id.fmt_indented_external_id(f, indent + 4); // Assumes a function like `fmt_indented_external_id` exists for `ExternalID`
                fmt_indented(f, indent, "},\n");
            }
            ID::PublicID(pubid_literal) => {
                fmt_indented(f, indent, &format!("PublicID({:?}),\n", pubid_literal));
            }
        }
    }
}

impl fmt::Debug for Subset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_subset(&mut s, 0);
        write!(f, "{}", s)
    }
}

impl Subset {
    fn fmt_subset(&self, f: &mut String, indent: usize) {
        match self {
            Subset::MarkupDecl(markup_declaration) => {
                markup_declaration.fmt_markup_decl(f, indent);
            }
            Subset::DeclSep {
                reference,
                expansion,
            } => {
                fmt_indented(f, indent, "DeclSep {\n");
                fmt_indented(f, indent + 4, &format!("reference: {:?},\n", reference));
                fmt_indented(f, indent + 4, "expansion: ");
                if let Some(inner) = expansion.as_deref() {
                    let mut s = String::new();
                    inner.fmt_subset(&mut s, indent + 8);
                    f.push_str(&format!("Some(\n{}\n", s));
                    fmt_indented(f, indent + 4, "),\n");
                } else {
                    f.push_str("None,\n");
                }
                fmt_indented(f, indent, "},\n");
            }
            Subset::None => {
                fmt_indented(f, indent, "None");
            }
        }
    }
}

impl MarkupDeclaration {
    fn fmt_markup_decl(&self, f: &mut String, indent: usize) {
        match self {
            MarkupDeclaration::Element { name, content_spec } => {
                fmt_indented(f, indent, "MarkupDeclaration::Element {\n");
                fmt_indented(
                    f,
                    indent + 4,
                    &format!("name: \n{}\n", name.fmt_qualified_name(indent + 8)),
                );
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

            MarkupDeclaration::AttList { name, att_defs } => {
                fmt_indented(f, indent, "AttList {\n");
                fmt_indented(
                    f,
                    indent + 4,
                    &format!("name: \n{}\n", name.fmt_qualified_name(indent + 8)),
                );
                fmt_indented(f, indent + 4, "att_defs: [\n");
                if let Some(def) = att_defs {
                    for def_item in def.iter() {
                        def_item.fmt_indented_attribute(f, indent + 8);
                    }
                }
                fmt_indented(f, indent + 4, "],\n");
                fmt_indented(f, indent, "},\n");
            }
            MarkupDeclaration::Notation { name, id } => {
                fmt_indented(f, indent, "Notation {\n");
                fmt_indented(
                    f,
                    indent + 4,
                    &format!("name: \n{}\n", name.fmt_qualified_name(indent + 8)),
                );
                fmt_indented(f, indent + 4, "id: ");

                // Use the fmt_indented_id function here
                id.fmt_indented_id(f, indent + 8);

                fmt_indented(f, indent, "},\n");
            }

            MarkupDeclaration::Entity(entity_declaration) => match entity_declaration {
                EntityDecl::General(general_declaration) => {
                    fmt_indented(f, indent, "Entity::General {\n");
                    let mut s = String::new();
                    general_declaration.fmt_indented_entity_declaration(&mut s, indent + 4);
                    f.push_str(&format!("{}\n", s));
                    fmt_indented(f, indent, "},\n");
                }
                EntityDecl::Parameter(parameter_declaration) => {
                    fmt_indented(f, indent, "Entity::Parameter {\n");
                    fmt_indented(f, indent + 4, &format!("{parameter_declaration:?}\n"));
                    fmt_indented(f, indent, "},\n");
                }
            },

            MarkupDeclaration::ProcessingInstruction(ProcessingInstruction { target, data }) => {
                fmt_indented(f, indent, "ProcessingInstruction {\n");
                fmt_indented(f, indent + 4, &format!("target: {:?},\n", target));
                fmt_indented(f, indent + 4, &format!("data: {:?},\n", data));
                fmt_indented(f, indent, "},\n");
            }
            MarkupDeclaration::Comment(comment) => {
                fmt_indented(f, indent, "Comment(\n");
                match comment {
                    Document::Comment(comment_str) => {
                        fmt_indented(f, indent + 4, &format!("{:?}\n", comment_str));
                    }
                    _ => {
                        fmt_indented(f, indent + 4, "Unsupported comment variant,\n");
                    }
                }
                fmt_indented(f, indent, "),\n");
            }
        }
    }
}

impl fmt::Debug for MarkupDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_markup_decl(&mut s, 0);
        write!(f, "{}", s)
    }
}

impl std::fmt::Debug for XmlDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XmlDecl")
            .field("version", &self.version)
            .field("encoding", &self.encoding)
            .field("standalone", &self.standalone)
            .finish()
    }
}
impl std::fmt::Debug for TextDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextDecl")
            .field("version", &self.version)
            .field("encoding", &self.encoding)
            .finish()
    }
}

impl std::fmt::Debug for AttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_string = String::new();
        self.fmt_indented_attribute_value(&mut debug_string, 4);
        write!(f, "{}", debug_string)
    }
}

impl AttributeValue {
    fn fmt_indented_attribute_value(&self, f: &mut String, indent: usize) {
        match self {
            AttributeValue::Value(value) => {
                fmt_indented(f, indent - 4, "Value(\n");
                fmt_indented(f, indent, &format!("{:?}\n", value));
                fmt_indented(f, indent - 4, "),\n");
            }
            AttributeValue::Values(values) => {
                fmt_indented(f, indent - 4, "Values(\n");
                for value in values {
                    value.fmt_indented_attribute_value(f, indent);
                }
                fmt_indented(f, indent - 4, "),\n");
            }
            AttributeValue::Reference(reference) => {
                fmt_indented(f, indent, "Reference(\n");
                fmt_indented(f, indent + 4, &format!("{:?},\n", reference));
                fmt_indented(f, indent, "),\n");
            }
            AttributeValue::EmptyExternalReference => {
                fmt_indented(f, indent, "EmptyExternalReference\n");
            }
        }
    }
}

impl Attribute {
    fn fmt_indented_attribute(&self, f: &mut String, indent: usize) {
        match self {
            Attribute::Definition {
                name,
                att_type,
                default_decl,
                source,
            } => {
                fmt_indented(f, indent, "Definition {\n");
                fmt_indented(f, indent + 4, "name: \n");
                let formatted_name = name.fmt_qualified_name(indent + 8);
                f.push_str(&formatted_name);
                f.push('\n');
                fmt_indented(f, indent + 4, &format!("att_type: {:?},\n", att_type));
                fmt_indented(
                    f,
                    indent + 4,
                    &format!("default_decl: {:?},\n", default_decl),
                );
                fmt_indented(f, indent + 4, &format!("source: {:?},\n", source));
                fmt_indented(f, indent, "},\n");
            }
            Attribute::Reference(reference) => {
                fmt_indented(f, indent, &format!("Reference: {:?},\n", reference));
            }
            Attribute::Required => {
                fmt_indented(f, indent, "REQUIRED,\n");
            }
            Attribute::Implied => {
                fmt_indented(f, indent, "IMPLIED,\n");
            }
            Attribute::Instance { name, value } => {
                fmt_indented(f, indent, "Instance {\n");
                fmt_indented(f, indent + 4, "name: \n");
                let formatted_name = name.fmt_qualified_name(indent + 8);
                f.push_str(&formatted_name);
                f.push('\n');
                fmt_indented(f, indent + 4, "value:\n");
                value.fmt_indented_attribute_value(f, indent + 12);
                fmt_indented(f, indent, "},\n");
            }
            Attribute::Namespace { prefix, uri } => {
                fmt_indented(f, indent, "Namespace {\n");
                fmt_indented(f, indent + 4, &format!("prefix: {:?},\n", prefix));
                uri.fmt_indented_attribute_value(f, indent + 4);
                fmt_indented(f, indent, "},\n");
            }
        }
    }
}

impl fmt::Debug for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_attribute(&mut s, 0);
        write!(f, "{}", s)
    }
}

impl fmt::Debug for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prefix::Default => write!(f, "Default"),
            Prefix::Prefix(p) => write!(f, "Prefix({:?})", p),
        }
    }
}

// impl fmt::Debug for Reference {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Reference::EntityRef(name) => f
//                 .debug_struct("EntityRef")
//                 .field("name", &format_args!("\n{}", name.fmt_qualified_name(12)))
//                 .finish(),

//             Reference::CharRef(value) => f.debug_struct("CharRef").field("value", value).finish(),
//         }
//     }
// }

impl fmt::Debug for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_reference(&mut s, 0);
        write!(f, "{}", s)
    }
}

impl Reference {
    fn fmt_indented_reference(&self, f: &mut String, indent: usize) {
        match self {
            Reference::EntityRef(name) => {
                fmt_indented(f, indent, "EntityRef {\n");
                fmt_indented(f, indent + 4, "name:\n");
                let name_str = name.fmt_qualified_name(indent + 10);
                f.push_str(&name_str);
                f.push('\n');
                fmt_indented(f, indent, "},\n");
            }
            Reference::CharRef(value) => {
                fmt_indented(f, indent, "CharRef {\n");
                fmt_indented(f, indent + 4, &format!("value: {:?},\n", value));
                fmt_indented(f, indent, "},\n");
            }
        }
    }
}

impl fmt::Debug for EntityDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_entity_declaration(&mut s, 0);
        write!(f, "{}", s.trim_end())
    }
}

impl EntityDeclaration {
    fn fmt_indented_entity_declaration(&self, f: &mut String, indent: usize) {
        fmt_indented(f, indent, "EntityDeclaration {\n");
        fmt_indented(
            f,
            indent + 8,
            &format!("name: \n{}\n", self.name.fmt_qualified_name(indent + 12)),
        );
        fmt_indented(f, indent + 4, "entity_def:\n");
        let mut s = String::new();
        self.entity_def
            .fmt_indented_entity_definition(&mut s, indent + 8);
        f.push_str(&s.to_string());
        fmt_indented(f, indent + 4, "},");
    }
}

impl std::fmt::Debug for EntityDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.fmt_indented_entity_definition(&mut s, 0);
        write!(f, "{}", s)
    }
}

impl EntityDefinition {
    fn fmt_indented_entity_definition(&self, f: &mut String, indent: usize) {
        match self {
            EntityDefinition::EntityValue(value) => {
                fmt_indented(f, indent, "EntityDefinition::EntityValue(\n");
                value.fmt_indented_entity_value(f, indent + 4);
                fmt_indented(f, indent, ")\n");
            }
            EntityDefinition::External {
                id,
                n_data,
                text_decl,
            } => {
                fmt_indented(f, indent, "EntityDefinition::External {\n");
                fmt_indented(f, indent + 4, &format!("id: {:?},\n", id));
                fmt_indented(f, indent + 4, &format!("n_data: {:?},\n", n_data));
                fmt_indented(f, indent + 4, &format!("text_decl: {:?},\n", text_decl));
                fmt_indented(f, indent, "},\n");
            }
        }
    }
}

impl std::fmt::Debug for EntityValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.fmt_indented_entity_value(&mut s, 0);
        write!(f, "{}", s)
    }
}

impl EntityValue {
    fn fmt_indented_entity_value(&self, f: &mut String, indent: usize) {
        match self {
            EntityValue::Value(value) => {
                fmt_indented(f, indent, "Value(\n");
                fmt_indented(f, indent, &format!("{value:?}\n"));
                fmt_indented(f, indent + 4, "),\n");
            }
            EntityValue::Reference(reference) => {
                fmt_indented(f, indent, "Reference(\n");
                fmt_indented(f, indent + 4, &format!("{reference:?},\n"));
                fmt_indented(f, indent, "),\n");
            }
            EntityValue::ParameterReference(reference) => {
                fmt_indented(f, indent, "PerameterReference(\n");
                fmt_indented(f, indent + 4, &format!("{reference:?},\n"));
                fmt_indented(f, indent, "),\n");
            }
            EntityValue::Document(document) => {
                fmt_indented(f, indent, "Document(\n");
                fmt_indented(f, indent + 4, &format!("{document:?}"));
                fmt_indented(f, indent, ")");
            }
            EntityValue::MarkupDecl(subset) => {
                // Handle the new variant here
                fmt_indented(f, indent, "MarkupDecl(\n");
                fmt_indented(f, indent + 4, &format!("{subset:?},\n"));
                fmt_indented(f, indent, "),\n");
            }
        }
    }
}
