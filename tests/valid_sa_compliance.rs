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
    ConditionalState, Document, QualifiedName,
};
use std::{borrow::Cow, error::Error, fs::File};

fn test_valid_sa_file<'a>(
    file_number: &str,
    buffer: &'a mut String,
) -> Result<Document<'a>, Box<dyn Error>> {
    let mut file = File::open(format!("tests/xmltest/valid/sa/{file_number}.xml"))?;

    let document = parse_file(&mut file, buffer)?;
    Ok(document)
}

#[test]
fn test_valid_sa_001() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let document = test_valid_sa_file("001", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    }]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("002", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("003", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("004", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: "v1".into(),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("005", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: "v1".into(),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("006", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: "v1".into(),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("007", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some(Cow::Owned(" ".into())))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("008", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some(Cow::Owned("&<>\"'".into())))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("009", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some(Cow::Owned(" ".into())))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("010", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: "v1".into(),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("011", &mut buffer)?;
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
                        InternalSubset::AttList {
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
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![
                        Attribute::Instance {
                            name: QualifiedName::new(None, "a1"),
                            value: "v1".into(),
                        },
                        Attribute::Instance {
                            name: QualifiedName::new(None, "a2"),
                            value: "v2".into(),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("012", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, ":"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, ":"),
                        value: "v1".into(),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("013", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "_.-0123456789"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "_.-0123456789"),
                        value: "v1".into(),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("014", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "abcdefghijklmnopqrstuvwxyz"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "abcdefghijklmnopqrstuvwxyz"),
                        value: "v1".into(),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("015", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
                        value: "v1".into(),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("016", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("017", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
                        data: Some("some data ".into()),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("017a", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
                    data: Some("some data ? > <?".into()),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("018", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::CDATA("<foo>".into())),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("019", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::CDATA("<&".into())),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("020", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::CDATA("<&]>]".into())),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("021", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Comment(" a comment ".into())),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("022", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Comment(" a comment ->".into())),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("023", &mut buffer)?;
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
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "".into()
                            )),
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
                Box::new(Document::Content(Some("".into()))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("024", &mut buffer)?;
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
                                        QualifiedName::new(None, "foo"),
                                        ConditionalState::None
                                    )],
                                    ConditionalState::None
                                )
                            )),
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "foo"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::EntityValue(EntityValue::Document(
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
                                )
                            )),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("025", &mut buffer)?;
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
                                        QualifiedName::new(None, "foo"),
                                        ConditionalState::ZeroOrMore
                                    ),],
                                    ConditionalState::None
                                )
                            )),
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "foo"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("026", &mut buffer)?;
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
                                        QualifiedName::new(None, "foo"),
                                        ConditionalState::ZeroOrMore
                                    )],
                                    ConditionalState::None
                                )
                            )),
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "foo"),
                            content_spec: Some(DeclarationContent::Empty),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("027", &mut buffer)?;
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
                                        QualifiedName::new(None, "foo"),
                                        ConditionalState::ZeroOrMore
                                    )],
                                    ConditionalState::None
                                )
                            )),
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "foo"),
                            content_spec: Some(DeclarationContent::Any),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("028", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: Some(XmlDecl {
                    version: Cow::Borrowed("1.0"),
                    encoding: None,
                    standalone: None,
                }),
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("029", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: Some(XmlDecl {
                    version: Cow::Borrowed("1.0"),
                    encoding: None,
                    standalone: None,
                }),
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("030", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: Some(XmlDecl {
                    version: Cow::Borrowed("1.0"),
                    encoding: None,
                    standalone: None,
                }),
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("031", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: Some(XmlDecl {
                    version: Cow::Borrowed("1.0"),
                    encoding: Some(Cow::Borrowed("UTF-8")),
                    standalone: None,
                }),
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("032", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: Some(XmlDecl {
                    version: Cow::Borrowed("1.0"),
                    encoding: None,
                    standalone: Some(Standalone::Yes),
                }),
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("033", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: Some(XmlDecl {
                    version: Cow::Borrowed("1.0"),
                    encoding: Some(Cow::Borrowed("UTF-8")),
                    standalone: Some(Standalone::Yes),
                }),
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("034", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("035", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("036", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
                data: Some("data".into()),
            }),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_037() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let document = test_valid_sa_file("037", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
            Document::Comment(" comment ".into()),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_038() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let document = test_valid_sa_file("038", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: Some(vec![Misc {
                    content: Box::new(Document::Nested(vec![Document::Comment(
                        " comment ".into()
                    )])),
                    state: MiscState::BeforeDoctype,
                },]),
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    }]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("039", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: Some(vec![Misc {
                    content: Box::new(Document::Nested(vec![Document::ProcessingInstruction(
                        ProcessingInstruction {
                            target: QualifiedName::new(None, "pi"),
                            data: Some("data".into()),
                        },
                    )])),
                    state: MiscState::BeforeDoctype,
                },]),
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    }]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("040", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: Cow::Borrowed("\"<&>'"), // decoding is applied to the attribute values
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("041", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: Cow::Borrowed("A"), // '&#65;' decodes to 'A'
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("042", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some(Cow::Borrowed("A")))), // '&#00000000000000000000000000000000065;' decodes to 'A'
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("043", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: Cow::Borrowed("foo\nbar"), // attribute value spans multiple lines
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("044", &mut buffer)?;
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
                                        ConditionalState::ZeroOrMore,
                                    )],
                                    ConditionalState::None,
                                )
                            )),
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "e"),
                            content_spec: Some(DeclarationContent::Empty),
                        },
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "e"),
                            att_defs: Some(vec![
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a1"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("v1".into()),
                                },
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a2"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("v2".into()),
                                },
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a3"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Implied,
                                },
                            ]),
                        },
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
                            value: Cow::Borrowed("v3"),
                        }]),
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "e"),
                        attributes: Some(vec![Attribute::Instance {
                            name: QualifiedName::new(None, "a1"),
                            value: Cow::Borrowed("w1"),
                        }]),
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "e"),
                        attributes: Some(vec![
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a2"),
                                value: Cow::Borrowed("w2"),
                            },
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a3"),
                                value: Cow::Borrowed("v3"),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("045", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a1"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("v1".into()),
                                },
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a1"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("z1".into()),
                                },
                            ]),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("046", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a1"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("v1".into()),
                                },
                                Attribute::Definition {
                                    name: QualifiedName::new(None, "a2"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value("v2".into()),
                                },
                            ]),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("047", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("X\nY".into()))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("048", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("]".into()))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("049", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("£".into()))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("050", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("เจมส์".into()))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("051", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "เจมส์"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "เจมส์"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("052", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("𐀀􏿽".into()))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("053", &mut buffer)?;
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
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::EntityValue(EntityValue::Document(
                                Document::EmptyTag(Tag {
                                    name: QualifiedName::new(None, "e"),
                                    attributes: None,
                                    state: TagState::Empty,
                                })
                            ))
                        })),
                        InternalSubset::Element {
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
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "e"),
                            content_spec: Some(DeclarationContent::Empty)
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("054", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    }]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("055", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: Some(vec![Misc {
                    content: Box::new(Document::Nested(vec![Document::ProcessingInstruction(
                        ProcessingInstruction {
                            target: QualifiedName::new(None, "pi"),
                            data: Some("data".into()),
                        }
                    )])),
                    state: MiscState::AfterDoctype,
                }]),
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    }]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("056", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some(Cow::Borrowed("A")))), // '&#x0000000000000000000000000000000000000041;' decodes to 'A'
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("057", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
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
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("058", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::Tokenized(TokenizedType::NMTOKENS),
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: Cow::Borrowed(" 1  	2 	"),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("059", &mut buffer)?;
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
                                        ConditionalState::ZeroOrMore,
                                    ),],
                                    ConditionalState::None
                                )
                            )),
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "e"),
                            content_spec: Some(DeclarationContent::Empty),
                        },
                        InternalSubset::AttList {
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
                        },
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
                                value: Cow::Borrowed("v1"),
                            },
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a2"),
                                value: Cow::Borrowed("v2"),
                            },
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a3"),
                                value: Cow::Borrowed("v3"),
                            },
                        ]),
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "e"),
                        attributes: Some(vec![
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a1"),
                                value: Cow::Borrowed("w1"),
                            },
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a2"),
                                value: Cow::Borrowed("v2"),
                            },
                        ]),
                        state: TagState::Empty,
                    }),
                    Document::EmptyTag(Tag {
                        name: QualifiedName::new(None, "e"),
                        attributes: Some(vec![
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a1"),
                                value: Cow::Borrowed("v1"),
                            },
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a2"),
                                value: Cow::Borrowed("w2"),
                            },
                            Attribute::Instance {
                                name: QualifiedName::new(None, "a3"),
                                value: Cow::Borrowed("v3"),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("060", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                //TODO: consider if this should be merged into Document::Content(Some("X\nY".into())),. Significant reworking needed to do this.
                Box::new(Document::Nested(vec![
                    Document::Content(Some("X".into())),
                    Document::Content(Some("\n".into())),
                    Document::Content(Some("Y".into())),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("061", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some(Cow::Borrowed("£")))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("062", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
                    Document::Content(Some(Cow::Borrowed("เจม"))),
                    Document::Content(Some(Cow::Borrowed("ส์"))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("063", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "เจมส์"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "เจมส์"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("064", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some(Cow::Borrowed(
                    "\u{10000}\u{10FFFD}"
                )))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("065", &mut buffer)?;
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
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "<".into()
                            )),
                        })),
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("066", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
                        InternalSubset::Comment(Document::Comment(" 34 is double quote ".into())),
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e1"),
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "\"".into()
                            )),
                        })),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: Cow::Borrowed("\""),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("067", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    }]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some(Cow::Borrowed("\r")))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("068", &mut buffer)?;
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
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "\r".into()
                            )),
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
                Box::new(Document::Content(Some(Cow::Borrowed("\r")))), // "&#13;" is a carriage return
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("069", &mut buffer)?;
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
                        InternalSubset::Notation {
                            name: QualifiedName::new(None, "n"),
                            id: ID::PublicID("whatever".into()),
                        },
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
// #[test]
fn test_valid_sa_070() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let document = test_valid_sa_file("070", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName {
                        prefix: None,
                        local_part: "doc".into(),
                    },
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Entity(EntityDecl::Parameter(EntityDeclaration {
                            name: QualifiedName {
                                prefix: None,
                                local_part: "e".into(),
                            },
                            entity_def: EntityDefinition::EntityValue(EntityValue::InternalSubset(
                                Box::new(InternalSubset::Element {
                                    name: QualifiedName {
                                        prefix: None,
                                        local_part: "doc".into(),
                                    },
                                    content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                        names: None,
                                        parsed: true,
                                    })),
                                })
                            )),
                        })),
                        InternalSubset::DeclSep(Reference::EntityRef(QualifiedName {
                            prefix: None,
                            local_part: "e".into(),
                        })),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName {
                        prefix: None,
                        local_part: "doc".into(),
                    },
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: QualifiedName {
                        prefix: None,
                        local_part: "doc".into(),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("071", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Tokenized(TokenizedType::ID),
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("072", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Tokenized(TokenizedType::IDREF),
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("073", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Tokenized(TokenizedType::IDREFS),
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("074", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Tokenized(TokenizedType::ENTITY),
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("075", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Tokenized(TokenizedType::ENTITIES),
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("076", &mut buffer)?;
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
                        InternalSubset::AttList {
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
                        },
                        InternalSubset::Notation {
                            name: QualifiedName::new(None, "n1"),
                            id: ID::ExternalID(ExternalID::System(Cow::Borrowed(
                                "http://www.w3.org/"
                            ))),
                        },
                        InternalSubset::Notation {
                            name: QualifiedName::new(None, "n2"),
                            id: ID::ExternalID(ExternalID::System(Cow::Borrowed(
                                "http://www.w3.org/"
                            ))),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("077", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Enumerated {
                                    notation: None,
                                    enumeration: Some(vec![Cow::Borrowed("1"), Cow::Borrowed("2")]),
                                },
                                default_decl: DefaultDecl::Implied,
                            }]),
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
fn test_valid_sa_078() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let document = test_valid_sa_file("078", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Required,
                            }]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: Cow::Borrowed("v"),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("079", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Fixed(Cow::Borrowed("v")),
                            }]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: Cow::Borrowed("v"),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("080", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Fixed(Cow::Borrowed("v")),
                            }]),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("081", &mut buffer)?;

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
                        },
                        InternalSubset::Element {
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
                        },
                        InternalSubset::Element {
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
                        },
                        InternalSubset::Element {
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
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("082", &mut buffer)?;
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
                        InternalSubset::Entity(EntityDecl::Parameter(EntityDeclaration {
                            name: QualifiedName::new(None, "e".into()),
                            entity_def: EntityDefinition::External {
                                id: ExternalID::System(Cow::Borrowed("e.dtd")),
                                n_data: None,
                            },
                        })),
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("083", &mut buffer)?;
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
                        InternalSubset::Entity(EntityDecl::Parameter(EntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::External {
                                id: ExternalID::Public {
                                    pubid: Cow::Borrowed("whatever"),
                                    system_identifier: Box::new(ExternalID::System(Cow::Borrowed(
                                        "e.dtd"
                                    ))),
                                },
                                n_data: None,
                            },
                        })),
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("084", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("085", &mut buffer)?;
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
                        InternalSubset::Entity(EntityDecl::Parameter(EntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "<foo>".into()
                            )),
                        })),
                        InternalSubset::Entity(EntityDecl::General(EntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "".into()
                            )),
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
                Box::new(Document::Content(Some(Cow::Borrowed("<foo>")))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("086", &mut buffer)?;
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
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "".into()
                            )),
                        })),
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "<foo>".into()
                            )),
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
                Box::new(Document::Content(Some(Cow::Borrowed("")))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("087", &mut buffer)?;
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
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::EntityValue(EntityValue::Document(
                                Document::EmptyTag(Tag {
                                    name: QualifiedName::new(None, "foo"),
                                    attributes: None,
                                    state: TagState::Empty,
                                })
                            )),
                        })),
                        InternalSubset::Element {
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
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "foo"),
                            content_spec: Some(DeclarationContent::Empty),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("088", &mut buffer)?;
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
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "<foo>".into()
                            )),
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
                Box::new(Document::Content(Some(Cow::Borrowed("<foo>")))), // Assumed to be a string because it's only an open tag
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("089", &mut buffer)?;

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
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "\u{10000}\u{10FFFD}\u{10FFFF}".into()
                            )),
                        })),
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
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
                Box::new(Document::Content(Some(Cow::Borrowed(
                    "\u{10000}\u{10FFFD}\u{10FFFF}"
                )))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("090", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "e"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Enumerated {
                                    notation: Some(vec![QualifiedName::new(None, "n")]),
                                    enumeration: None,
                                },
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
                        InternalSubset::Element {
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
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "e"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
                        InternalSubset::Notation {
                            name: QualifiedName::new(None, "n"),
                            id: ID::PublicID("whatever".into()),
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
fn test_valid_sa_091() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let document = test_valid_sa_file("091", &mut buffer)?;
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
                        InternalSubset::Notation {
                            name: QualifiedName::new(None, "n"),
                            id: ID::ExternalID(ExternalID::System(Cow::Borrowed(
                                "http://www.w3.org/"
                            )))
                        },
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::External {
                                id: ExternalID::System(Cow::Borrowed("http://www.w3.org/")),
                                n_data: Some(QualifiedName::new(None, "n")),
                            }
                        })),
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Tokenized(TokenizedType::ENTITY),
                                default_decl: DefaultDecl::Value(Cow::Borrowed("e")),
                            }]),
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
fn test_valid_sa_092() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let document = test_valid_sa_file("092", &mut buffer)?;
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
                                        QualifiedName::new(None, "a"),
                                        ConditionalState::None
                                    ),],
                                    ConditionalState::ZeroOrMore
                                )
                            )),
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "a"),
                            content_spec: Some(DeclarationContent::Empty),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("093", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    }]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("094", &mut buffer)?;
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
                        InternalSubset::Entity(EntityDecl::Parameter(EntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "foo".into()
                            )),
                        })),
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Value("%e;".into()),
                            }]),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("095", &mut buffer)?;
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
                        InternalSubset::AttList {
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
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a1"),
                        value: Cow::Borrowed("1  2"),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("096", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::Tokenized(TokenizedType::NMTOKENS),
                                default_decl: DefaultDecl::Value(" 1  \t2 \t".into()),
                            }]),
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("097", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a1"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Value(Cow::Borrowed("v1")),
                            }]),
                        },
                        InternalSubset::Entity(EntityDecl::Parameter(ParameterEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::External {
                                id: ExternalID::System(Cow::Borrowed("097.ent")),
                                n_data: None,
                            },
                        })),
                        InternalSubset::Entities(vec![
                            Box::new(InternalSubset::AttList {
                                name: QualifiedName::new(None, "doc"),
                                att_defs: Some(vec![Attribute::Definition {
                                    name: QualifiedName::new(None, "a2"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Implied,
                                }]),
                            }),
                            Box::new(InternalSubset::AttList {
                                name: QualifiedName::new(None, "doc"),
                                att_defs: Some(vec![Attribute::Definition {
                                    name: QualifiedName::new(None, "a2"),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Value(Cow::Borrowed("v2")),
                                }]),
                            }),
                        ]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("098", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    }]),
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
                    data: Some(Cow::Borrowed("x\ny")),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("099", &mut buffer)?;

    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: Some(XmlDecl {
                    version: Cow::Borrowed("1.0"),
                    encoding: Some(Cow::Borrowed("utf-8")),
                    standalone: None,
                }),
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    }]),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("100", &mut buffer)?;
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
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e"),
                            entity_def: EntityDefinition::External {
                                id: ExternalID::Public {
                                    pubid: ";!*#@$_%".into(),
                                    system_identifier: Box::new(ExternalID::System(
                                        "100.xml".into()
                                    ))
                                },
                                n_data: None,
                            },
                        })),
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "doc"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("101", &mut buffer)?;
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
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "\"".into()
                            )),
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
fn test_valid_sa_102() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let document = test_valid_sa_file("102", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: Cow::Borrowed("\""),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("103", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    }]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Nested(vec![
                    Document::Content(Some("<".into())), //TODO: look at post-processing step to merge these into <doc> then parse it as an element
                    Document::Content(Some("doc>".into())),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("104", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: Cow::Borrowed("x\ty"), // decoded tab character
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("105", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: Cow::Borrowed("x\ty"),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("106", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: Cow::Borrowed("x\ny"),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("107", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: Cow::Borrowed("x\ny"),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("108", &mut buffer)?;
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
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "\n".into()
                            )),
                        })),
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: Cow::Borrowed("x\ny"),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("109", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: Cow::Borrowed(""),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("110", &mut buffer)?;
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
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "\r\n".into()
                            )),
                        })),
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: Cow::Borrowed("x\ny"),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("111", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "doc"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::Tokenized(TokenizedType::NMTOKENS),
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: Some(vec![Attribute::Instance {
                        name: QualifiedName::new(None, "a"),
                        value: Cow::Borrowed(" x  y "), // &#32; decodes to space
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("112", &mut buffer)?;
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
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "a"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                            })),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("113", &mut buffer)?;
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
                        InternalSubset::AttList {
                            name: QualifiedName::new(None, "e"),
                            att_defs: Some(vec![Attribute::Definition {
                                name: QualifiedName::new(None, "a"),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            }]),
                        },
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("114", &mut buffer)?;
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
                            entity_def: EntityDefinition::EntityValue(EntityValue::Document(
                                Document::CDATA(Cow::Borrowed("&foo;"))
                            )),
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
                Box::new(Document::CDATA(Cow::Borrowed("&foo;"))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("115", &mut buffer)?;
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
                            name: QualifiedName::new(None, "e1"),
                            entity_def: EntityDefinition::EntityValue(EntityValue::Reference(
                                Reference::EntityRef(QualifiedName::new(None, "e2"))
                            )),
                        })),
                        InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                            name: QualifiedName::new(None, "e2"),
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "v".into()
                            )),
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
                Box::new(Document::Content(Some(Cow::Borrowed("v")))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("116", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                            names: None,
                            parsed: true,
                        })),
                    },]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::CDATA("\n".into())),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("117", &mut buffer)?;
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
                            name: QualifiedName::new(None, "rsqb"),
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "]".into()
                            )),
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
                Box::new(Document::Content(Some(Cow::Borrowed("]")))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("118", &mut buffer)?;
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
                            name: QualifiedName::new(None, "rsqb"),
                            entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                "]]".into()
                            )),
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
                Box::new(Document::Content(Some(Cow::Borrowed("]]")))),
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
    let mut buffer = String::new();
    let document = test_valid_sa_file("119", &mut buffer)?;
    assert_eq!(
        document,
        Document::Nested(vec![
            Document::Prolog {
                xml_decl: None,
                misc: None,
                doc_type: Some(DocType {
                    name: QualifiedName::new(None, "doc"),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: QualifiedName::new(None, "doc"),
                        content_spec: Some(DeclarationContent::Any),
                    }]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Comment(Cow::Borrowed(" -á "))),
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
