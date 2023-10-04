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
        subset::{
            entity_declaration::{
                EntityDecl, EntityDeclaration, GeneralEntityDeclaration, ParameterEntityDeclaration,
            },
            entity_definition::EntityDefinition,
            entity_value::EntityValue,
            internal::InternalSubset,
            markup_declaration::MarkupDeclaration,
        },
        xmldecl::{Standalone, XmlDecl},
    },
    reference::Reference,
    tag::{Tag, TagState},
    ConditionalState, Config, Document, ExternalEntityParseConfig, QualifiedName,
};
use std::{error::Error, fs::File};

fn test_valid_sa_file(file_number: &str, config: Config) -> Result<Document, Box<dyn Error>> {
    let mut file = File::open(format!("tests/xmltest/valid/sa/{file_number}.xml"))?;

    let document = parse_file(&mut file, config)?;
    Ok(document)
}

#[test]
fn test_valid_sa_001() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("001", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
fn test_valid_sa_002() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("002", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
fn test_valid_sa_003() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("003", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
    let document = test_valid_sa_file("004", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: AttributeValue::Value("v1".to_string()),
                    }]),
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
fn test_valid_sa_005() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("005", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: AttributeValue::Value("v1".to_string()),
                    }]),
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
fn test_valid_sa_006() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("006", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: AttributeValue::Value("v1".to_string()),
                    }]),
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
fn test_valid_sa_007() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("007", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some(" ".to_string()))),
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
fn test_valid_sa_008() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("008", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("&<>\"'".to_string()))),
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
fn test_valid_sa_009() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("009", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some(" ".to_string()))),
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
fn test_valid_sa_010() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("010", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: AttributeValue::Value("v1".to_string()),
                    }]),
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
fn test_valid_sa_011() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("011", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a1"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Implied,
                                },
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a2"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Implied,
                                },
                            ]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![
                        Attribute::Instance {
                            name: QualifiedName::new(None, "a1"),
                            value: AttributeValue::Value("v1".to_string()),
                        },
                        Attribute::Instance {
                            name: QualifiedName::new(None, "a2"),
                            value: AttributeValue::Value("v2".to_string()),
                        },
                    ]),
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
fn test_valid_sa_012() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("012", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, ":"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, ":"), // TODO: confirm this is correct
                        value: AttributeValue::Value("v1".to_string()),
                    }]),
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
fn test_valid_sa_013() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("013", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "_.-0123456789"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "_.-0123456789"),
                        value: AttributeValue::Value("v1".to_string()),
                    }]),
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
fn test_valid_sa_014() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("014", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "abcdefghijklmnopqrstuvwxyz"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "abcdefghijklmnopqrstuvwxyz"),
                        value: AttributeValue::Value("v1".to_string()),
                    }]),
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
fn test_valid_sa_015() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("015", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
                        value: AttributeValue::Value("v1".to_string()),
                    }]),
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
fn test_valid_sa_016() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("016", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::ProcessingInstruction(ProcessingInstruction {
                    target: QualifiedName::new(None, "pi"),
                    data: None,
                })),
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
fn test_valid_sa_017() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("017", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Nested(vec![
                    Document::ProcessingInstruction(ProcessingInstruction {
                        target: QualifiedName::new(None, "pi"),
                        data: Some("some data ".to_string()),
                    }),
                    Document::ProcessingInstruction(ProcessingInstruction {
                        target: QualifiedName::new(None, "x"),
                        data: None,
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
fn test_valid_sa_017a() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("017a", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::ProcessingInstruction(ProcessingInstruction {
                    target: QualifiedName::new(None, "pi"),
                    data: Some("some data ? > <?".to_string()),
                })),
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
fn test_valid_sa_018() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("018", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::CDATA("<foo>".to_string())),
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
fn test_valid_sa_019() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("019", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::CDATA("<&".to_string())),
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
fn test_valid_sa_020() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("020", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::CDATA("<&]>]".to_string())),
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
fn test_valid_sa_021() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("021", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Comment(" a comment ".to_string())),
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
fn test_valid_sa_022() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("022", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Comment(" a comment ->".to_string())),
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
fn test_valid_sa_023() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("023", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "".to_string()
                                )),
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("".to_string()))),
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
fn test_valid_sa_024() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("024", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        QualifiedName::new(None, "foo"),
                                        ConditionalState::None
                                    )],
                                    ConditionalState::None
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "foo"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "&#60;foo></foo>".to_string()
                                )),
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Nested(vec![Document::Element(
                    Tag {
                        name: QualifiedName::new(None, "foo"),
                        attributes: None,
                        state: TagState::Start,
                    },
                    Box::new(Document::Empty),
                    Tag {
                        name: QualifiedName::new(None, "foo"),
                        attributes: None,
                        state: TagState::End,
                    },
                )])),
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
fn test_valid_sa_025() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("025", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        QualifiedName::new(None, "foo"),
                                        ConditionalState::ZeroOrMore
                                    ),],
                                    ConditionalState::None
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "foo"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
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
                        name: QualifiedName::new(None, "foo"),
                        attributes: None,
                        state: TagState::Empty,
                    },),
                    Document::Element(
                        Tag {
                            name: QualifiedName::new(None, "foo"),
                            attributes: None,
                            state: TagState::Start,
                        },
                        Box::new(Document::Empty),
                        Tag {
                            name: QualifiedName::new(None, "foo"),
                            attributes: None,
                            state: TagState::End,
                        },
                    ),
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
fn test_valid_sa_026() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("026", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        QualifiedName::new(None, "foo"),
                                        ConditionalState::ZeroOrMore
                                    )],
                                    ConditionalState::None
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "foo"),
                            content_spec: Some(DeclarationContent::Empty),
                        }),
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
                        name: QualifiedName::new(None, "foo"),
                        attributes: None,
                        state: TagState::Empty,
                    },),
                    Document::Element(
                        Tag {
                            name: QualifiedName::new(None, "foo"),
                            attributes: None,
                            state: TagState::Start,
                        },
                        Box::new(Document::Empty),
                        Tag {
                            name: QualifiedName::new(None, "foo"),
                            attributes: None,
                            state: TagState::End,
                        },
                    ),
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
fn test_valid_sa_027() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("027", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        QualifiedName::new(None, "foo"),
                                        ConditionalState::ZeroOrMore
                                    )],
                                    ConditionalState::None
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "foo"),
                            content_spec: Some(DeclarationContent::Any),
                        }),
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
                        name: QualifiedName::new(None, "foo"),
                        attributes: None,
                        state: TagState::Empty,
                    },),
                    Document::Element(
                        Tag {
                            name: QualifiedName::new(None, "foo"),
                            attributes: None,
                            state: TagState::Start,
                        },
                        Box::new(Document::Empty),
                        Tag {
                            name: QualifiedName::new(None, "foo"),
                            attributes: None,
                            state: TagState::End,
                        },
                    ),
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
fn test_valid_sa_028() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("028", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: Some(XmlDecl {
                    version: "1.0".to_string(),
                    encoding: None,
                    standalone: None,
                }),
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
fn test_valid_sa_029() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("029", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: Some(XmlDecl {
                    version: "1.0".to_string(),
                    encoding: None,
                    standalone: None,
                }),
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
fn test_valid_sa_030() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("030", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: Some(XmlDecl {
                    version: "1.0".to_string(),
                    encoding: None,
                    standalone: None,
                }),
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
fn test_valid_sa_031() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("031", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: Some(XmlDecl {
                    version: "1.0".to_string(),
                    encoding: Some("UTF-8".to_string()),
                    standalone: None,
                }),
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
fn test_valid_sa_032() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("032", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: Some(XmlDecl {
                    version: "1.0".to_string(),
                    encoding: None,
                    standalone: Some(Standalone::Yes),
                }),
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
fn test_valid_sa_033() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("033", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: Some(XmlDecl {
                    version: "1.0".to_string(),
                    encoding: Some("UTF-8".to_string()),
                    standalone: Some(Standalone::Yes),
                }),
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
fn test_valid_sa_034() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("034", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::EmptyTag(Tag {
                name: QualifiedName::new(None, "doc"),
                attributes: None,
                state: TagState::Empty,
            },),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_035() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("035", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::EmptyTag(Tag {
                name: QualifiedName::new(None, "doc"),
                attributes: None,
                state: TagState::Empty,
            },),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_036() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("036", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
                }
            ),
            Document::ProcessingInstruction(ProcessingInstruction {
                target: QualifiedName::new(None, "pi"),
                data: Some("data".to_string()),
            }),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_037() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("037", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
                }
            ),
            Document::Comment(" comment ".to_string()),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_038() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("038", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: Some(vec![Misc {
                    content: Box::new(Document::Nested(vec![Document::Comment(
                        " comment ".to_string()
                    )])),
                    state: MiscState::BeforeDoctype,
                }]),
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
                }
            ),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_039() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("039", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: Some(vec![Misc {
                    content: Box::new(Document::Nested(vec![Document::ProcessingInstruction(
                        ProcessingInstruction {
                            target: QualifiedName::new(None, "pi"),
                            data: Some("data".to_string()),
                        },
                    )])),
                    state: MiscState::BeforeDoctype,
                }]),
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
                }
            ),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_040() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("040", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: AttributeValue::Value("\"<&>'".to_string()), // using .into() instead of Cow::Borrowed
                    }]),
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
fn test_valid_sa_041() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("041", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: AttributeValue::Value("A".to_string()), // '&#65;' decodes to 'A'
                    }]),
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
fn test_valid_sa_042() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("042", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("A".to_string()))), // '&#00000000000000000000000000000000065;' decodes to 'A'
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
fn test_valid_sa_043() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("043", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: AttributeValue::Value("foo\nbar".to_string()), // attribute value spans multiple lines
                    }]),
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
fn test_valid_sa_044() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("044", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        QualifiedName::new(None, "e"),
                                        ConditionalState::ZeroOrMore,
                                    )],
                                    ConditionalState::None,
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "e"),
                            content_spec: Some(DeclarationContent::Empty),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "e"),
                            att_defs: Some(vec![
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a1"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("v1".to_string()),
                                },
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a2"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("v2".to_string()),
                                },
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a3"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Implied,
                                },
                            ]),
                        }),
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
                        attributes: Some(vec![Attribute::Instance {
                            name: QualifiedName::new(None, "a3"),
                            value: AttributeValue::Value("v3".to_string()),
                        }]),
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "e"),
                        attributes: Some(vec![Attribute::Instance {
                            name: QualifiedName::new(None, "a1"),
                            value: AttributeValue::Value("w1".to_string()),
                        }]),
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "e"),
                        attributes: Some(vec![
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a2"),
                                value: AttributeValue::Value("w2".to_string()),
                            },
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a3"),
                                value: AttributeValue::Value("v3".to_string()),
                            },
                        ]),
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
fn test_valid_sa_045() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("045", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a1"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("v1".to_string()),
                                },
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a1"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("z1".to_string()),
                                },
                            ]),
                        }),
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
fn test_valid_sa_046() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("046", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a1"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("v1".to_string()),
                                },
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a2"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("v2".to_string()),
                                },
                            ]),
                        }),
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
fn test_valid_sa_047() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("047", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("X\nY".to_string()))),
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
fn test_valid_sa_048() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("048", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("]".to_string()))),
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
fn test_valid_sa_049() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("049", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("£".to_string()))),
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
fn test_valid_sa_050() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("050", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("เจมส์".to_string()))),
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
fn test_valid_sa_051() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("051", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "เจมส์"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "เจมส์"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "เจมส์"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: QualifiedName::new(None, "เจมส์"),
                    attributes: None,
                    state: TagState::End,
                },
            ),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_052() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("052", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("𐀀􏿽".to_string()))),
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
fn test_valid_sa_053() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("053", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "<e/>".to_string()
                                ))
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        QualifiedName::new(None, "e"),
                                        ConditionalState::None,
                                    ),],
                                    ConditionalState::None
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "e"),
                            content_spec: Some(DeclarationContent::Empty)
                        })
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::EmptyTag(Tag {
                    // TODO: consider that this is a variant of a tag; therefore, should it be nested?
                    name: QualifiedName::new(None, "e"),
                    attributes: None,
                    state: TagState::Empty,
                })),
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
fn test_valid_sa_054() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("054", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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

// TODO: analyze the misc to see if Some(Vec<Misc.content(Box<Document::Nested>)>) is correct
#[test]
fn test_valid_sa_055() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("055", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: Some(vec![Misc {
                    content: Box::new(Document::Nested(vec![Document::ProcessingInstruction(
                        ProcessingInstruction {
                            target: QualifiedName::new(None, "pi"),
                            data: Some("data".to_string()),
                        }
                    )])),
                    state: MiscState::AfterDoctype,
                }]),
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
fn test_valid_sa_056() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("056", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("A".to_string()))), // '&#x0000000000000000000000000000000000000041;' decodes to 'A'
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
fn test_valid_sa_057() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("057", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        QualifiedName::new(None, "a"),
                                        ConditionalState::ZeroOrMore,
                                    ),],
                                    ConditionalState::None
                                )
                            )),
                        }
                    )]),
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
fn test_valid_sa_058() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("058", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::Tokenized(TokenizedType::NMTOKENS),
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: AttributeValue::Value(" 1  	2 	".to_string()), // The attribute value from the input
                    }]),
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
fn test_valid_sa_059() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("059", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        QualifiedName::new(None, "e"),
                                        ConditionalState::ZeroOrMore,
                                    ),],
                                    ConditionalState::None
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "e"),
                            content_spec: Some(DeclarationContent::Empty),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "e"),
                            att_defs: Some(vec![
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a1"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Implied,
                                },
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a2"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Implied,
                                },
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a3"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Implied,
                                },
                            ]),
                        }),
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
                        attributes: Some(vec![
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a1"),
                                value: AttributeValue::Value("v1".to_string()),
                            },
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a2"),
                                value: AttributeValue::Value("v2".to_string()),
                            },
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a3"),
                                value: AttributeValue::Value("v3".to_string()),
                            },
                        ]),
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "e"),
                        attributes: Some(vec![
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a1"),
                                value: AttributeValue::Value("w1".to_string()),
                            },
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a2"),
                                value: AttributeValue::Value("v2".to_string()),
                            },
                        ]),
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "e"),
                        attributes: Some(vec![
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a1"),
                                value: AttributeValue::Value("v1".to_string()),
                            },
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a2"),
                                value: AttributeValue::Value("w2".to_string()),
                            },
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a3"),
                                value: AttributeValue::Value("v3".to_string()),
                            },
                        ]),
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
fn test_valid_sa_060() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("060", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                //TODO: consider if this should be merged into Document::Content(Some("X\nY".to_string())),. Significant reworking needed to do this.
                Box::new(Document::Nested(vec![
                    Document::Content(Some("X".to_string())),
                    Document::Content(Some("\n".to_string())),
                    Document::Content(Some("Y".to_string())),
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
fn test_valid_sa_061() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("061", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("£".to_string()))),
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
fn test_valid_sa_062() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("062", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Nested(vec![
                    //TODO: Same as test 60, should this all be combined?
                    Document::Content(Some("เจม".to_string())),
                    Document::Content(Some("ส์".to_string())),
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
fn test_valid_sa_063() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("063", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "เจมส์"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "เจมส์"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "เจมส์"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: QualifiedName::new(None, "เจมส์"),
                    attributes: None,
                    state: TagState::End,
                },
            ),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_064() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("064", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("\u{10000}\u{10FFFD}".to_string()))),
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
fn test_valid_sa_065() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("065", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Reference(
                                    Reference::CharRef("<".to_string())
                                )),
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
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
fn test_valid_sa_066() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("066", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Comment(Document::Comment(
                            " 34 is double quote ".to_string()
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e1"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Reference(
                                    Reference::CharRef("\"".to_string())
                                )),
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: AttributeValue::Value("\"".to_string()),
                    }]),
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
fn test_valid_sa_067() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("067", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("\r".to_string()))),
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
fn test_valid_sa_068() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("068", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Reference(
                                    Reference::CharRef("\r".to_string())
                                )),
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("\r".to_string()))), // "&#13;" is a carriage return
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
fn test_valid_sa_069() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("069", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Notation {
                            name: QualifiedName::new(None, "n"),
                            id: ID::PublicID("whatever".to_string()),
                        }),
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
fn test_valid_sa_070() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("070", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName {
                        prefix: None,
                        local_part: "doc".to_string(),
                    },
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(
                            EntityDecl::Parameter(ParameterEntityDeclaration {
                                name: QualifiedName {
                                    prefix: None,
                                    local_part: "e".to_string(),
                                },
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "<!ELEMENT doc (#PCDATA)>".to_string()
                                )),
                            })
                        )),
                        InternalSubset::DeclSep {
                            reference: Reference::EntityRef(QualifiedName {
                                prefix: None,
                                local_part: "e".to_string(),
                            }),
                            expansion: Some(Box::new(InternalSubset::MarkupDecl(
                                MarkupDeclaration::Element {
                                    name: QualifiedName {
                                        prefix: None,
                                        local_part: "doc".to_string(),
                                    },
                                    content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                                }
                            ))),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName {
                        prefix: None,
                        local_part: "doc".to_string(),
                    },
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: QualifiedName {
                        prefix: None,
                        local_part: "doc".to_string(),
                    },
                    attributes: None,
                    state: TagState::End,
                },
            ),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_071() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("071", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Tokenized(TokenizedType::ID),
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
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
fn test_valid_sa_072() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("072", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Tokenized(TokenizedType::IDREF),
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
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
fn test_valid_sa_073() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("073", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Tokenized(TokenizedType::IDREFS),
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
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
fn test_valid_sa_074() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("074", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Tokenized(TokenizedType::ENTITY),
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
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
fn test_valid_sa_075() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("075", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Tokenized(TokenizedType::ENTITIES),
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
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
fn test_valid_sa_076() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("076", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Enumerated {
                                    notation: Some(vec![
                                        QualifiedName::new(None, "n1"),
                                        QualifiedName::new(None, "n2"),
                                    ]),
                                    enumeration: None,
                                },
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Notation {
                            name: QualifiedName::new(None, "n1"),
                            id: ID::ExternalID(ExternalID::System(
                                "http://www.w3.org/".to_string()
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Notation {
                            name: QualifiedName::new(None, "n2"),
                            id: ID::ExternalID(ExternalID::System(
                                "http://www.w3.org/".to_string()
                            )),
                        }),
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
fn test_valid_sa_077() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("077", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Enumerated {
                                    notation: None,
                                    enumeration: Some(vec!["1".to_string(), "2".to_string()]),
                                },
                                default_decl: DefaultDecl::Implied,
                            }]),
                        })
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
fn test_valid_sa_078() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("078", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Required,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: AttributeValue::Value("v".to_string()),
                    }]),
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
fn test_valid_sa_079() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("079", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Fixed("v".to_string()),
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: AttributeValue::Value("v".to_string()),
                    }]),
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
fn test_valid_sa_080() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("080", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Fixed("v".to_string()),
                            }]),
                        }),
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

// TODO: test 081
#[test]
fn test_valid_sa_081() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("081", Config::default())?;

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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![
                                        ContentParticle::Name(
                                            QualifiedName::new(None, "a"),
                                            ConditionalState::None
                                        ),
                                        ContentParticle::Name(
                                            QualifiedName::new(None, "b"),
                                            ConditionalState::None
                                        ),
                                        ContentParticle::Name(
                                            QualifiedName::new(None, "c"),
                                            ConditionalState::None
                                        ),
                                    ],
                                    ConditionalState::None
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "a"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        QualifiedName::new(None, "a"),
                                        ConditionalState::Optional
                                    ),],
                                    ConditionalState::None
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "b"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        QualifiedName::new(None, "b"),
                                        ConditionalState::ZeroOrMore
                                    ),],
                                    ConditionalState::None
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "c"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Choice(
                                    vec![
                                        ContentParticle::Name(
                                            QualifiedName::new(None, "a"),
                                            ConditionalState::None
                                        ),
                                        ContentParticle::Name(
                                            QualifiedName::new(None, "b"),
                                            ConditionalState::None
                                        ),
                                    ],
                                    ConditionalState::OneOrMore
                                )
                            )),
                        }),
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
                        name: QualifiedName::new(None, "a"),
                        attributes: None,
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "b"),
                        attributes: None,
                        state: TagState::Empty,
                    }),
                    Document::Element(
                        Tag {
                            name: QualifiedName::new(None, "c"),
                            attributes: None,
                            state: TagState::Start,
                        },
                        Box::new(Document::EmptyTag(Tag {
                            name: QualifiedName::new(None, "a"),
                            attributes: None,
                            state: TagState::Empty,
                        })),
                        Tag {
                            name: QualifiedName::new(None, "c"),
                            attributes: None,
                            state: TagState::End,
                        },
                    ),
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
fn test_valid_sa_082() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("082", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(
                            EntityDecl::Parameter(ParameterEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("e.dtd".to_string()),
                                    n_data: None,
                                    text_decl: None,
                                },
                            })
                        )),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
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

//TODO: test 083
#[test]
fn test_valid_sa_083() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("083", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(
                            EntityDecl::Parameter(ParameterEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::Public {
                                        pubid: "whatever".to_string(),
                                        system_identifier: Box::new(ExternalID::System(
                                            "e.dtd".to_string()
                                        )),
                                    },
                                    n_data: None,
                                    text_decl: None,
                                },
                            })
                        )),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
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
fn test_valid_sa_084() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("084", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
fn test_valid_sa_085() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("085", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(
                            EntityDecl::Parameter(ParameterEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "<foo>".to_string()
                                )),
                            })
                        )),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            EntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "".to_string()
                                )),
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("<foo>".to_string()))),
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
fn test_valid_sa_086() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("086", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "".to_string()
                                )),
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "<foo>".to_string()
                                )),
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("".to_string()))),
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
fn test_valid_sa_087() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("087", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "<foo/&#62;".to_string()
                                )),
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        QualifiedName::new(None, "foo"),
                                        ConditionalState::None,
                                    ),],
                                    ConditionalState::None
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "foo"),
                            content_spec: Some(DeclarationContent::Empty),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::EmptyTag(Tag {
                    name: QualifiedName::new(None, "foo"),
                    attributes: None,
                    state: TagState::Empty,
                })),
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
fn test_valid_sa_088() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("088", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "<foo>".to_string()
                                )),
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("<foo>".to_string()))), // Assumed to be a string because it's only an open tag
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
fn test_valid_sa_089() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("089", Config::default())?;

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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "\u{10000}\u{10FFFD}\u{10FFFF}".to_string()
                                )),
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        })
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some(
                    "\u{10000}\u{10FFFD}\u{10FFFF}".to_string()
                ))),
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
fn test_valid_sa_090() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("090", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "e"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Enumerated {
                                    notation: Some(vec![QualifiedName::new(None, "n")]),
                                    enumeration: None,
                                },
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        QualifiedName::new(None, "e"),
                                        ConditionalState::None
                                    ),],
                                    ConditionalState::ZeroOrMore,
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "e"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Notation {
                            name: QualifiedName::new(None, "n"),
                            id: ID::PublicID("whatever".to_string()),
                        })
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
fn test_valid_sa_091() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("091", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Notation {
                            name: QualifiedName::new(None, "n"),
                            id: ID::ExternalID(ExternalID::System(
                                "http://www.w3.org/".to_string()
                            ))
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("http://www.w3.org/".to_string()),
                                    n_data: Some(QualifiedName::new(None, "n")),
                                    text_decl: None,
                                }
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Tokenized(TokenizedType::ENTITY),
                                default_decl: DefaultDecl::Value("e".to_string()),
                            }]),
                        })
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
fn test_valid_sa_092() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("092", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Sequence(
                                    vec![ContentParticle::Name(
                                        QualifiedName::new(None, "a"),
                                        ConditionalState::None
                                    ),],
                                    ConditionalState::ZeroOrMore
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "a"),
                            content_spec: Some(DeclarationContent::Empty),
                        }),
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
                        name: QualifiedName::new(None, "a"),
                        attributes: None,
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "a"),
                        attributes: None,
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "a"),
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
fn test_valid_sa_093() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("093", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
fn test_valid_sa_094() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("094", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(
                            EntityDecl::Parameter(ParameterEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "foo".to_string()
                                )),
                            })
                        )),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Value("%e;".to_string()),
                            }]),
                        }),
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
fn test_valid_sa_095() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("095", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a1"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Implied,
                                },
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a1"),
                                    att_type: AttType::Tokenized(TokenizedType::NMTOKENS),
                                    default_decl: DefaultDecl::Implied,
                                }
                            ]),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: AttributeValue::Value("1  2".to_string()),
                    }]),
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
fn test_valid_sa_096() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("096", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::Tokenized(TokenizedType::NMTOKENS),
                                default_decl: DefaultDecl::Value(" 1  \t2 \t".to_string()),
                            }]),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
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

//TODO: Test 097 with lookup in test 097.ent
#[test]
fn test_valid_sa_097() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file(
        "097",
        Config {
            external_parse_config: ExternalEntityParseConfig {
                allow_ext_parse: true,
                ignore_ext_parse_warning: true,
                base_directory: Some("tests/xmltest/valid/sa".into()),
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(
                            EntityDecl::Parameter(ParameterEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::System("097.ent".to_string()),
                                    n_data: None,
                                    text_decl: None,
                                },
                            })
                        )),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a1"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("v1".to_string()),
                                },
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a2"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("v2".to_string()),
                                },
                            ]),
                        }),
                        InternalSubset::DeclSep {
                            reference: Reference::EntityRef(QualifiedName::new(None, "e")),
                            expansion: Some(Box::new(InternalSubset::MarkupDecl(
                                MarkupDeclaration::AttList {
                                    name: QualifiedName::new(None, "doc"),
                                    att_defs: Some(vec![Attribute::Definition {
                                        name: QualifiedName::new(None, "a2"),
                                        att_type: AttType::CDATA,
                                        default_decl: DefaultDecl::Implied,
                                    }]),
                                }
                            ))),
                        }
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
fn test_valid_sa_098() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("098", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::ProcessingInstruction(ProcessingInstruction {
                    target: QualifiedName::new(None, "pi"),
                    data: Some("x\ny".to_string()),
                })),
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
fn test_valid_sa_099() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("099", Config::default())?;

    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: Some(XmlDecl {
                    version: "1.0".to_string(),
                    encoding: Some("utf-8".to_string()),
                    standalone: None,
                }),
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
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
fn test_valid_sa_100() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("100", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::External {
                                    id: ExternalID::Public {
                                        pubid: ";!*#@$_%".to_string(),
                                        system_identifier: Box::new(ExternalID::System(
                                            "100.xml".to_string()
                                        ))
                                    },
                                    n_data: None,
                                    text_decl: None,
                                },
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
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
fn test_valid_sa_101() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("101", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Reference(
                                    Reference::CharRef("\"".to_string())
                                )),
                            }
                        ))),
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
fn test_valid_sa_102() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("102", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: AttributeValue::Value("\"".to_string()),
                    }]),
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
fn test_valid_sa_103() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("103", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Nested(vec![
                    Document::Content(Some("<".to_string())), //TODO: look at post-processing step to merge these into <doc> then parse it as an element
                    Document::Content(Some("doc>".to_string())),
                ]),),
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
fn test_valid_sa_104() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("104", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: AttributeValue::Value("x\ty".to_string()), // decoded tab character
                    }]),
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
fn test_valid_sa_105() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("105", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: AttributeValue::Value("x\ty".to_string()),
                    }]),
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
fn test_valid_sa_106() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("106", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: AttributeValue::Value("x\ny".to_string()),
                    }]),
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
fn test_valid_sa_107() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("107", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: AttributeValue::Value("x\ny".to_string()),
                    }]),
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
fn test_valid_sa_108() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("108", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "\n".to_string()
                                )),
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: AttributeValue::Value("x\ny".to_string()),
                    }]),
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
fn test_valid_sa_109() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("109", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: AttributeValue::Value("".to_string()),
                    }]),
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

//TODO: Test 110. Need to verify normalization behavior for `\r\n` current parsing replaces all instances of that with `\n`
#[test]
fn test_valid_sa_110() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("110", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "\r\n".to_string()
                                )),
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: AttributeValue::Value("x\ny".to_string()),
                    }]),
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
fn test_valid_sa_111() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("111", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Tokenized(TokenizedType::NMTOKENS),
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: AttributeValue::Value(" x  y ".to_string()), // &#32; decodes to space
                    }]),
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

//TODO: test 112
#[test]
fn test_valid_sa_112() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("112", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Children(
                                ContentParticle::Choice(
                                    vec![
                                        ContentParticle::Name(
                                            QualifiedName::new(None, "a"),
                                            ConditionalState::None
                                        ),
                                        ContentParticle::Name(
                                            QualifiedName::new(None, "b"),
                                            ConditionalState::None
                                        ),
                                    ],
                                    ConditionalState::None,
                                )
                            )),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "a"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Nested(vec![Document::Element(
                    Tag {
                        name: QualifiedName::new(None, "a"),
                        attributes: None,
                        state: TagState::Start,
                    },
                    Box::new(Document::Empty),
                    Tag {
                        name: QualifiedName::new(None, "a"),
                        attributes: None,
                        state: TagState::End,
                    },
                ),])),
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
fn test_valid_sa_113() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("113", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::AttList {
                            name: QualifiedName::new(None, "e"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        }),
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

//TODO: Test 114. may need to decode within the Document::Element section instead of directly in the attribute because CDATA is not supposed to decode and if it's in the content, then it should avoid decoding
#[test]
fn test_valid_sa_114() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("114", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "<![CDATA[&foo;]]>".to_string()
                                )),
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::CDATA("&foo;".to_string())),
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

//TODO: Test 115
#[test]
fn test_valid_sa_115() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("115", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e1"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Reference(
                                    Reference::EntityRef(QualifiedName::new(None, "e2"))
                                )),
                            }
                        ))),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e2"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "v".to_string()
                                )),
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("v".to_string()))),
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
fn test_valid_sa_116() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("116", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::CDATA("\n".to_string())),
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
fn test_valid_sa_117() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("117", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "rsqb"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "]".to_string()
                                )),
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("]".to_string()))),
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
fn test_valid_sa_118() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("118", Config::default())?;
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
                        InternalSubset::MarkupDecl(MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
                        }),
                        InternalSubset::MarkupDecl(MarkupDeclaration::Entity(EntityDecl::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "rsqb"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "]]".to_string()
                                )),
                            }
                        ))),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("]]".to_string()))),
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
fn test_valid_sa_119() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("119", Config::default())?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::MarkupDecl(
                        MarkupDeclaration::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Any),
                        }
                    )]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Comment(" -á ".to_string())),
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
