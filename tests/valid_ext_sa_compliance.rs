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
    let mut file = File::open(format!("tests/xmltest/valid/ext-sa/{file_number}.xml"))?;

    let document = parse_file(&mut file, config)?;
    Ok(document)
}

#[test]
fn test_valid_ext_sa_001() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
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
                    name: Name::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("001.ent".to_string()),
                                    n_data: None,
                                    text_decl: None
                                },
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: Name::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("Data\n".to_string()))),
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
fn test_valid_ext_sa_002() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
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
                    name: Name::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("002.ent".to_string()),
                                    n_data: None,
                                    text_decl: None,
                                },
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: Name::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("Data".to_string()))),
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
fn test_valid_ext_sa_003() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
        "003",
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
                    name: Name::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("003.ent".to_string()),
                                    n_data: None,
                                    text_decl: None,
                                },
                            }
                        ))),
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

#[test]
fn test_valid_ext_sa_004() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
        "004",
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
                    name: Name::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("004.ent".to_string()),
                                    n_data: None,
                                    text_decl: None,
                                },
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: Name::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("Data\n".to_string()))),
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
fn test_valid_ext_sa_005() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
        "005",
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
                    name: Name::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        Name::new(None, "e"),
                                        ConditionalState::ZeroOrMore,
                                    )],
                                    ConditionalState::None,
                                ),
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "e"),
                            content_spec: Some(DeclarationContent::Empty),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("005.ent".to_string()),
                                    n_data: None,
                                    text_decl: None,
                                },
                            },
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: Name::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Nested(vec![
                    Document::EmptyTag(Tag {
                        name: Name::new(None, "e"),
                        attributes: None,
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: Name::new(None, "e"),
                        attributes: None,
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: Name::new(None, "e"),
                        attributes: None,
                        state: TagState::Empty,
                    }),
                ])),
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
fn test_valid_ext_sa_006() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
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
                    name: Name::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::Names(vec!(
                                Name::new(None, "e")
                            )))),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "e"),
                            content_spec: Some(DeclarationContent::Empty),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("006.ent".to_string()),
                                    n_data: None,
                                    text_decl: None,
                                },
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: Name::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Nested(vec![
                    Document::Content(Some("Data\n".to_string())),
                    Document::EmptyTag(Tag {
                        name: Name::new(None, "e"),
                        attributes: None,
                        state: TagState::Empty,
                    }),
                    Document::Content(Some("More data\n".to_string())),
                    Document::EmptyTag(Tag {
                        name: Name::new(None, "e"),
                        attributes: None,
                        state: TagState::Empty,
                    }),
                ])),
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
fn test_valid_ext_sa_007() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
        "007",
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
                    name: Name::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("007.ent".to_string()),
                                    n_data: None,
                                    text_decl: None,
                                },
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: Name::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Nested(vec![
                    Document::Content(Some("X".to_string())),
                    Document::Content(Some("Y".to_string())), // Here's the data from the new .ent file
                    Document::Content(Some("Z".to_string())),
                ])),
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
fn test_valid_ext_sa_008() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
        "008",
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
                    name: Name::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("008.ent".to_string()),
                                    n_data: None,
                                    text_decl: None,
                                },
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: Name::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Nested(vec![
                    Document::Content(Some("X".to_string())),
                    Document::Content(Some("Y".to_string())), // The data from the 008.ent file
                    Document::Content(Some("Z".to_string())),
                ])),
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
fn test_valid_ext_sa_009() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
        "009",
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
                    name: Name::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("009.ent".to_string()),
                                    n_data: None,
                                    text_decl: None,
                                },
                            }
                        ))),
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

#[test]
fn test_valid_ext_sa_010() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
        "010",
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
                    name: Name::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("010.ent".to_string()),
                                    n_data: None,
                                    text_decl: None,
                                },
                            }
                        ))),
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

#[test]
fn test_valid_ext_sa_011() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
        "011",
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
                    name: Name::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::Public {
                                        pubid: "a not very interesting file".to_string(),
                                        system_identifier: Box::new(ExternalID::System(
                                            "011.ent".to_string()
                                        ))
                                    },
                                    n_data: None,
                                    text_decl: None,
                                },
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: Name::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("xyzzy\n".to_string())),),
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
fn test_valid_ext_sa_012() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
        "012",
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
                    name: Name::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e1"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Reference(
                                    Reference::EntityRef(Name::new(None, "e2"))
                                )),
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e2"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Reference(
                                    Reference::EntityRef(Name::new(None, "e3"))
                                )),
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e3"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("012.ent".to_string()),
                                    n_data: None,
                                    text_decl: None,
                                },
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e4"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Reference(
                                    Reference::EntityRef(Name::new(None, "e5"))
                                )),
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e5"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "(e5)".to_string()
                                )),
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
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
                Box::new(Document::Content(Some("(e5)".to_string()))),
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
fn test_valid_ext_sa_013() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
        "013",
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
                    name: Name::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        Name::new(None, "e"),
                                        ConditionalState::None
                                    )],
                                    ConditionalState::None
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "e"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: Name::new(None, "e"),
                            att_defs: Some(vec![
                                Attribute::Definition {
                                    name: Name::new(None, "a1"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("a1 default".to_string()),
                                },
                                Attribute::Definition {
                                    name: Name::new(None, "a2"),
                                    att_type: AttType::Tokenized(TokenizedType::NMTOKENS),
                                    default_decl: DefaultDecl::Value("a2 default".to_string()),
                                },
                            ]),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "x"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("013.ent".to_string()),
                                    n_data: None,
                                    text_decl: None,
                                },
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: Name::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::EmptyTag(Tag {
                    name: Name::new(None, "e"),
                    attributes: Some(vec![
                        Attribute::Instance {
                            name: Name::new(None, "a1"),
                            value: AttributeValue::Value("a1 default".to_string()),
                        },
                        Attribute::Instance {
                            name: Name::new(None, "a2"),
                            value: AttributeValue::Value("a2 default".to_string()),
                        },
                    ]),
                    state: TagState::Empty,
                },)),
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
fn test_valid_ext_sa_014() -> Result<(), Box<dyn Error>> {
    let document = test_valid_ext_sa_file(
        "014",
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
                    name: Name::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: Name::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: Name::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("014.ent".to_string()),
                                    n_data: None,
                                    text_decl: None
                                },
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: Name::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("data".to_string()))),
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
