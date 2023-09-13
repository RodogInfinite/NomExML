use nom_xml::{
    attribute::{AttType, Attribute, AttributeValue, DefaultDecl, TokenizedType},
    io::parse_file,
    misc::{Misc, MiscState},
    processing_instruction::ProcessingInstruction,
    prolog::{
        content_particle::ContentParticle,
        declaration_content::{DeclarationContent, Mixed},
        doctype::DocType,
        external_id::ExternalID,
        id::ID,
        internal_subset::{
            entity_declaration::{
                EntityDecl, EntityDeclaration, GeneralEntityDeclaration, ParameterEntityDeclaration,
            },
            entity_definition::EntityDefinition,
            entity_value::EntityValue,
            InternalSubset,
        },
        xmldecl::{Standalone, XmlDecl},
    },
    reference::Reference,
    tag::{Tag, TagState},
    ConditionalState, Config, Document, ExternalEntityParseConfig, QualifiedName,
};
use std::{error::Error, fs::File};
fn test_valid_sa_file(file_number: &str, config: Config) -> Result<Document, Box<dyn Error>> {
    let mut file = File::open(format!("tests/xmltest/valid/ext-sa/{file_number}.xml"))?;

    let document = parse_file(&mut file, config)?;
    Ok(document)
}

#[test]
fn test_valid_sa_001() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file(
        "001",
        Config {
            external_parse_config: ExternalEntityParseConfig {
                allow_ext_parse: true,
                ignore_ext_parse_warning: true,
                base_directory: Some("tests/xmltest/valid/ext-sa".into()),
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
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::External {
                                id: ExternalID::System("001.ent".to_string()),
                                n_data: None,
                            },
                        })),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("Data\n".to_string()))),
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::End,
                },
            ),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_002() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file(
        "002",
        Config {
            external_parse_config: ExternalEntityParseConfig {
                allow_ext_parse: true,
                ignore_ext_parse_warning: true,
                base_directory: Some("tests/xmltest/valid/ext-sa".into()),
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
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::External {
                                id: ExternalID::System("002.ent".to_string()),
                                n_data: None,
                            },
                        })),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("Data".to_string()))),
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::End,
                },
            ),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_003() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file(
        "003", // adjusted for the new test
        Config {
            external_parse_config: ExternalEntityParseConfig {
                allow_ext_parse: true,
                ignore_ext_parse_warning: true,
                base_directory: Some("tests/xmltest/valid/ext-sa".into()),
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
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::External {
                                id: ExternalID::System("003.ent".to_string()),
                                n_data: None,
                            },
                        })),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::End,
                },
            ),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_004() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file(
        "004", // adjusted for the new test
        Config {
            external_parse_config: ExternalEntityParseConfig {
                allow_ext_parse: true,
                ignore_ext_parse_warning: true,
                base_directory: Some("tests/xmltest/valid/ext-sa".into()),
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
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::External {
                                id: ExternalID::System("004.ent".to_string()),
                                n_data: None,
                            },
                        })),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("Data\n".to_string()))),
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::End,
                },
            ),
        ]),
    );

    Ok(())
}

#[test]
fn test_valid_sa_005() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file(
        "005", // adjusted for the new test
        Config {
            external_parse_config: ExternalEntityParseConfig {
                allow_ext_parse: true,
                ignore_ext_parse_warning: true,
                base_directory: Some("tests/xmltest/valid/ext-sa".into()),
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
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        QualifiedName::new(None, "e"),
                                        ConditionalState::ZeroOrMore
                                    )],
                                    ConditionalState::None
                                )
                            )),
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "e"),
                            content_spec: Some(DeclarationContent::Empty),
                        },
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::External {
                                id: ExternalID::System("005.ent".to_string()),
                                n_data: None,
                            },
                        })),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Nested(vec![
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "e"),
                        attributes: None,
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "e"),
                        attributes: None,
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "e"),
                        attributes: None,
                        state: TagState::Empty,
                    }),
                ])),
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::End,
                },
            ),
        ]),
    );

    Ok(())
}

#[test]
fn test_valid_sa_006() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file(
        "006",
        Config {
            external_parse_config: ExternalEntityParseConfig {
                allow_ext_parse: true,
                ignore_ext_parse_warning: true,
                base_directory: Some("tests/xmltest/valid/ext-sa".into()),
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
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Choice(
                                    vec![
                                        ContentParticle::Name(
                                            QualifiedName::new(None, "#PCDATA"),
                                            ConditionalState::None
                                        ),
                                        ContentParticle::Name(
                                            QualifiedName::new(None, "e"),
                                            ConditionalState::ZeroOrMore
                                        ),
                                    ],
                                    ConditionalState::None
                                )
                            )),
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "e"),
                            content_spec: Some(DeclarationContent::Empty),
                        },
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::External {
                                id: ExternalID::System("006.ent".to_string()),
                                n_data: None,
                            },
                        })),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Nested(vec![
                    Document::Content(Some("Data\n".to_string())),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "e"),
                        attributes: None,
                        state: TagState::Empty,
                    }),
                    Document::Content(Some("More data\n".to_string())),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "e"),
                        attributes: None,
                        state: TagState::Empty,
                    }),
                ])),
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::End,
                },
            ),
        ]),
    );

    Ok(())
}
