use nom_xml::{
    attribute::{AttType, Attribute, DefaultDecl},
    io::parse_entire_file,
    prolog::{
        declaration_content::DeclarationContent,
        doctype::DocType,
        external_id::ExternalID,
        subset::{entity::EntitySource, markup_declaration::MarkupDeclaration, Subset},
    },
    tag::{Tag, TagState},
    Config, Document, ExternalEntityParseConfig, Name,
};
use std::{error::Error, fs::File};
fn test_valid_ext_sa_file(file_number: &str, config: Config) -> Result<Document, Box<dyn Error>> {
    let mut file = File::open(format!("tests/xmltest/valid/not-sa/{file_number}.xml"))?;

    let document = parse_entire_file(&mut file, config)?;
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
                    subset: Some(vec![Subset::MarkupDecl(MarkupDeclaration::Element {
                        name: Name::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Empty),
                    }),]),
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

#[test]
fn test_valid_not_sa_002() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
        "002",
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
                    external_id: Some(ExternalID::System("002.ent".to_string())),
                    subset: Some(vec![Subset::MarkupDecl(MarkupDeclaration::Element {
                        name: Name::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Empty),
                    }),]),
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

#[test]
#[ignore]
fn test_valid_not_sa_003() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
        "003",
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
                    external_id: Some(ExternalID::System("003-1.ent".to_string())),
                    subset: Some(vec![
                        Subset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Empty),
                        }),
                        Subset::MarkupDecl(MarkupDeclaration::AttList {
                            name: Name::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: Name::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                                source: EntitySource::External,
                            },]),
                        }),
                    ]),
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
