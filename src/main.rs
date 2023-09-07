use nom_xml::{
    attribute::{AttType, Attribute, DefaultDecl, TokenizedType},
    io::parse_file,
    misc::{Misc, MiscState},
    processing_instruction::ProcessingInstruction,
    prolog::{
        content_particle::ContentParticle,
        declaration_content::{DeclarationContent, Mixed},
        doctype::DocType,
        external_id::ExternalID,
        xmldecl::{Standalone, XmlDecl},
    },
    tag::{Tag, TagState},
    ConditionalState, Document, QualifiedName,
};
use std::{borrow::Cow, error::Error, fs::File};
fn main() {
    let mut buffer = String::new();
    fn test_file<'a>(
        file_number: &str,
        buffer: &'a mut String,
    ) -> Result<Document<'a>, Box<dyn Error>> {
        let mut file = File::open(format!("data/{file_number}.xml"))?;

        let document = parse_file(&mut file, buffer)?;

        Ok(document)
    }

    let document = test_file("65", &mut buffer).unwrap();

    println!("\n\n\n==========\n\n\n{:#?}", document);
}
