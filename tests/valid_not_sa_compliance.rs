use nom_xml::{
    attribute::{AttType, Attribute, AttributeValue, DefaultDecl, TokenizedType},
    io::parse_file,
    prolog::{
        content_particle::ContentParticle,
        declaration_content::{DeclarationContent, Mixed},
        doctype::DocType,
        external_id::ExternalID,
        subset::{
            entity_declaration::{EntityDecl, GeneralEntityDeclaration},
            entity_definition::EntityDefinition,
            entity_value::EntityValue,
            internal::InternalSubset,
            markup_declaration::MarkupDeclaration,
        },
    },
    reference::Reference,
    tag::{Tag, TagState},
    ConditionalState, Config, Document, ExternalEntityParseConfig, Name,
};
use std::{error::Error, fs::File};
fn test_valid_ext_sa_file(file_number: &str, config: Config) -> Result<Document, Box<dyn Error>> {
    let mut file = File::open(format!("tests/xmltest/valid/not-sa/{file_number}.xml"))?;

    let document = parse_file(&mut file, config)?;
    Ok(document)
}

#[test]
fn test_valid_not_sa_001() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
        "001",
        Config {
            external_parse_config: ExternalEntityParseConfig {
                allow_ext_parse: true,
                ignore_ext_parse_warning: true,
                base_directory: Some("tests/xmltest/valid/not-sa".into()),
            },
        },
    )?;

    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: Name::new(None, "doc"),
                    external_id: Some(ExternalID::System("001.ent".to_string())),
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Empty),
                        }
                    ),]),
                }),
            },
            Document::Element(
                Tag {
                    name: Name::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: Name::new(None, "doc"),
                    attributes: None,
                    state: TagState::End,
                },
            ),
        ]),
    );
    Ok(())
}
