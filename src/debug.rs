// debug.rs
use crate::{
    attribute::{Attribute, Prefix},
    misc::{Misc, MiscState},
    processing_instruction::ProcessingInstruction,
    prolog::{
        content_particle::ContentParticle,
        declaration_content::{DeclarationContent, Mixed},
        doctype::DocType,
        external_id::ExternalID,
        internal_subset::{
            EntityDeclaration, EntityDefinition, EntityValue, GeneralEntityDeclaration,
            InternalSubset, ID,
        },
        xmldecl::{Standalone, XmlDecl},
    },
    reference::{CharRefState, Reference},
    Document, QualifiedName, Tag,
};
use std::fmt::{self, Formatter};

fn fmt_indented(f: &mut String, indent: usize, s: &str) {
    f.push_str(&" ".repeat(indent));
    f.push_str(s);
}
impl<'a> fmt::Debug for Tag<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_tag(&mut s, 0);
        write!(f, "{}", s)
    }
}
impl<'a> Tag<'a> {
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
        // Removed the extra line here

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

impl<'a> fmt::Debug for QualifiedName<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fmt_qualified_name(0))
    }
}

impl<'a> QualifiedName<'a> {
    fn fmt_qualified_name(&self, indent: usize) -> String {
        let QualifiedName { prefix, local_part } = self;
        let mut f = String::new();

        fmt_indented(&mut f, indent, "QualifiedName {\n");

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
        fmt_indented(&mut f, indent, "},");

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

impl<'a> fmt::Debug for Misc<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_misc(&mut s, 0);
        write!(f, "{}", s)
    }
}

impl<'a> Misc<'a> {
    fn fmt_indented_misc(&self, f: &mut String, indent: usize) {
        fmt_indented(f, indent, "Misc {\n");
        fmt_indented(f, indent + 4, &format!("content: {:?}", self.content));
        fmt_indented(f, indent + 4, &format!("state: {:?},\n", self.state));
        fmt_indented(f, indent, "},\n");
    }
}

impl<'a> Document<'a> {
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
                    &format!("Comment(\"{}\"),\n", comment.to_string()),
                );
            }
            Document::Empty => {
                fmt_indented(f, indent, "Empty,\n");
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
        writeln!(f, "{}", s)
    }
}

impl<'a> DeclarationContent<'a> {
    fn fmt_indented_dec_content(&self, f: &mut String, indent: usize) {
        match self {
            DeclarationContent::Mixed(mixed) => {
                fmt_indented(f, indent, "Mixed {\n");
                mixed.fmt_indented_mixed(f, indent + 4);
                fmt_indented(f, indent, "},");
            }
            DeclarationContent::Children(children) => {
                fmt_indented(f, indent, "Children {\n");
                let mut s = String::new();
                children.fmt_indented_content_particle(&mut s, indent + 4);
                f.push_str(&format!("[\n{}\n", s));
                fmt_indented(f, indent, "},");
            }
            DeclarationContent::Empty => {
                fmt_indented(f, indent, "Empty,");
            }
            DeclarationContent::Any => {
                fmt_indented(f, indent, "Any,");
            }
        }
    }
}

impl<'a> fmt::Debug for DeclarationContent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_indented_dec_content(&mut s, 0);
        writeln!(f, "{}", s)
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
            ContentParticle::Name(name, conditional_state) => {
                fmt_indented(f, indent, "Name {\n");
                fmt_indented(f, indent + 4, &format!("name: {:?}\n", name));
                fmt_indented(
                    f,
                    indent + 4,
                    &format!("conditional_state: {:?},\n", conditional_state),
                );
                fmt_indented(f, indent, "},\n");
            }
            ContentParticle::Choice(particles, conditional_state) => {
                fmt_indented(f, indent, "Choice {\n");
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
                fmt_indented(f, indent, "Seq {\n");
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

impl<'a> XmlDecl<'a> {
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

impl<'a> std::fmt::Debug for DocType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DocType")
            .field("name", &self.name)
            .field("external_id", &self.external_id)
            .field("int_subset", &self.int_subset)
            .finish()
    }
}

impl<'a> DocType<'a> {
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
        fmt_indented(f, indent + 4, "int_subset: Some([\n");
        for element in self.int_subset.as_ref().unwrap_or(&Vec::new()).iter() {
            element.fmt_internal_subset(f, indent + 8);
        }
        fmt_indented(f, indent + 4, "]),\n");
        fmt_indented(f, indent, "},\n");
    }
}
impl<'a> ExternalID<'a> {
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

impl<'a> std::fmt::Debug for ID<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ID::ExternalID(external_id) => f.debug_tuple("ExternalID").field(&external_id).finish(),
            ID::PublicID(pubid_literal) => f.debug_tuple("PublicID").field(&pubid_literal).finish(),
        }
    }
}

impl<'a> ID<'a> {
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

impl<'a> InternalSubset<'a> {
    fn fmt_internal_subset(&self, f: &mut String, indent: usize) {
        match self {
            InternalSubset::Element { name, content_spec } => {
                fmt_indented(f, indent, "Element {\n");
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

            InternalSubset::AttList { name, att_defs } => {
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
            InternalSubset::Notation { name, id } => {
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

            InternalSubset::Entity(entity_declaration) => {
                match entity_declaration {
                    EntityDeclaration::General(general_declaration) => {
                        fmt_indented(f, indent, "Entity::General {\n");
                        let mut s = String::new();
                        general_declaration
                            .fmt_indented_general_entity_declaration(&mut s, indent + 4);
                        f.push_str(&format!("{}\n", s));
                        fmt_indented(f, indent, "},\n");
                    }
                    EntityDeclaration::Parameter(parameter_declaration) => {
                        fmt_indented(f, indent, "Entity::Parameter {\n");
                        // This part depends on how you want to format ParameterEntityDefinition
                        fmt_indented(f, indent + 4, &format!("{:?},\n", parameter_declaration));
                        fmt_indented(f, indent, "},\n");
                    }
                }
            }
            InternalSubset::DeclSep(name) => {
                fmt_indented(f, indent, &format!("\nDeclSep({}", format!("{:?}", name)));
                f.push_str("),\n");
            }
            InternalSubset::ProcessingInstruction(ProcessingInstruction { target, data }) => {
                fmt_indented(f, indent, "ProcessingInstruction {\n");
                fmt_indented(f, indent + 4, &format!("target: {:?},\n", target));
                fmt_indented(f, indent + 4, &format!("data: {:?},\n", data));
                fmt_indented(f, indent, "},\n");
            }
            InternalSubset::Comment(comment) => {
                fmt_indented(f, indent, "Comment(\n");
                match comment {
                    Document::Comment(comment_str) => {
                        fmt_indented(f, indent + 4, &format!("{:?}\n", comment_str));
                    }
                    // If `Document` has other variants, you can match them here
                    _ => {
                        fmt_indented(f, indent + 4, "Unsupported comment variant,\n");
                    }
                }
                fmt_indented(f, indent, "),\n");
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

impl<'a> Attribute<'a> {
    fn fmt_indented_attribute(&self, f: &mut String, indent: usize) {
        match self {
            Attribute::Definition {
                name,
                att_type,
                default_decl,
            } => {
                fmt_indented(f, indent, "Definition {\n");
                fmt_indented(
                    f,
                    indent + 4,
                    &format!("name: \n{}\n", name.fmt_qualified_name(indent + 8)),
                );
                fmt_indented(f, indent + 4, &format!("att_type: {:?},\n", att_type));
                fmt_indented(
                    f,
                    indent + 4,
                    &format!("default_decl: {:?},\n", default_decl),
                );
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
                fmt_indented(f, indent + 4, &format!("name: {:?}\n", name));
                fmt_indented(f, indent + 4, &format!("value: {:?},\n", value));
                fmt_indented(f, indent, "},\n");
            }
            Attribute::Namespace { prefix, uri } => {
                fmt_indented(f, indent, "Namespace {\n");
                fmt_indented(f, indent + 4, &format!("prefix: {:?},\n", prefix));
                fmt_indented(f, indent + 4, &format!("uri: {:?},\n", uri));
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

impl<'a> fmt::Debug for Prefix<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prefix::Default => write!(f, "Default"),
            Prefix::Prefix(p) => write!(f, "Prefix({:?})", p),
        }
    }
}

impl<'a> fmt::Debug for Reference<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reference::EntityRef(entity) => {
                f.debug_struct("EntityRef").field("entity", entity).finish()
            }
            Reference::CharRef { value, state } => f
                .debug_struct("CharRef")
                .field("value", value)
                .field("state", state)
                .finish(),
        }
    }
}

impl fmt::Debug for CharRefState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CharRefState::Decimal => f.write_str("Decimal"),
            CharRefState::Hexadecimal => f.write_str("Hex"),
        }
    }
}

impl<'a> std::fmt::Debug for GeneralEntityDeclaration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeneralEntityDeclaration")
            .field("name", &self.name)
            .field("entity_def", &self.entity_def)
            .finish()
    }
}
impl<'a> GeneralEntityDeclaration<'a> {
    fn fmt_indented_general_entity_declaration(&self, f: &mut String, indent: usize) {
        fmt_indented(f, indent, "GeneralEntityDeclaration {\n");
        fmt_indented(f, indent + 4, &format!("name: {:?},\n", self.name));
        fmt_indented(f, indent + 4, "entity_def: ");
        let mut s = String::new();
        self.entity_def
            .fmt_indented_entity_definition(&mut s, indent + 8);
        f.push_str(&format!("{}\n", s));
        fmt_indented(f, indent, "},");
    }
}

impl<'a> std::fmt::Debug for EntityDefinition<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.fmt_indented_entity_definition(&mut s, 0);
        write!(f, "{}", s)
    }
}

impl<'a> EntityDefinition<'a> {
    fn fmt_indented_entity_definition(&self, f: &mut String, indent: usize) {
        match self {
            EntityDefinition::EntityValue(value) => {
                f.push_str("EntityValue {\n");
                let mut s = String::new();
                value.fmt_indented_entity_value(&mut s, indent + 4);
                f.push_str(&format!("{}", s));
                fmt_indented(f, indent - 4, "},");
            }
            EntityDefinition::External { id, n_data } => {
                fmt_indented(f, indent, "External {\n");
                fmt_indented(f, indent + 4, &format!("id: {:?},\n", id));
                fmt_indented(f, indent + 4, &format!("n_data: {:?},\n", n_data));
                fmt_indented(f, indent, "},");
            }
        }
    }
}

impl<'a> std::fmt::Debug for EntityValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityValue::Value(value) => {
                f.debug_struct("EntityValue").field("Value", value).finish()
            }
            EntityValue::Reference(reference) => f
                .debug_struct("EntityValue")
                .field("Reference", reference)
                .finish(),
            EntityValue::PerameterReference(reference) => f
                .debug_struct("EntityValue")
                .field("PerameterReference", reference)
                .finish(),
        }
    }
}

impl<'a> EntityValue<'a> {
    fn fmt_indented_entity_value(&self, f: &mut String, indent: usize) {
        match self {
            EntityValue::Value(value) => {
                fmt_indented(f, indent - 4, "Value(\n");
                fmt_indented(f, indent, &format!("{:?}\n", value));
                fmt_indented(f, indent - 4, "),\n");
            }
            EntityValue::Reference(reference) => {
                fmt_indented(f, indent, "Reference(\n");
                fmt_indented(f, indent + 4, &format!("{:?},\n", reference));
                fmt_indented(f, indent, "),\n");
            }
            EntityValue::PerameterReference(reference) => {
                fmt_indented(f, indent, "PerameterReference(\n");
                fmt_indented(f, indent + 4, &format!("{:?},\n", reference));
                fmt_indented(f, indent, "),\n");
            }
        }
    }
}
