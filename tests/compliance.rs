use std::{borrow::Cow, error::Error, fs::File};

use nom_xml::{
    attribute::{AttType, Attribute, DefaultDecl},
    io::parse_file,
    misc::{Misc, MiscState},
    processing_instruction::ProcessingInstruction,
    prolog::{
        content_particle::ContentParticle,
        declaration_content::{DeclarationContent, Mixed},
        doctype::DocType,
        internal_subset::{
            EntityDeclaration, EntityDefinition, EntityValue, GeneralEntityDeclaration,
            InternalSubset,
        },
        xmldecl::{Standalone, XmlDecl},
    },
    tag::{Tag, TagState},
    ConditionalState, Document, QualifiedName,
};

fn test_valid_sa_file<'a>(file_number: &str) -> Result<Document<'a>, Box<dyn Error>> {
    let mut file = File::open(format!("tests/xmltest/valid/sa/{file_number}.xml"))?;
    let document = parse_file(&mut file)?;
    Ok(document)
}

#[test]
fn test_valid_sa_001() -> Result<(), Box<dyn Error>> {
    let document = test_valid_sa_file("001")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("002")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("003")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("004")?;
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
                                zero_or_more: false,
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
    let document = test_valid_sa_file("005")?;
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
                                zero_or_more: false,
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
    let document = test_valid_sa_file("006")?;
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
                                zero_or_more: false,
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
    let document = test_valid_sa_file("007")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("008")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("009")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("010")?;
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
                                zero_or_more: false,
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
    let document = test_valid_sa_file("011")?;
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
                                zero_or_more: false,
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
    let document = test_valid_sa_file("012")?;
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
                                zero_or_more: false,
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
    let document = test_valid_sa_file("013")?;
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
                                zero_or_more: false,
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
    let document = test_valid_sa_file("014")?;
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
                                zero_or_more: false,
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
    let document = test_valid_sa_file("015")?;
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
                                zero_or_more: false,
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
    let document = test_valid_sa_file("016")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("017")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("017a")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("018")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("019")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("020")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("021")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("022")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("023")?;
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
                                zero_or_more: false,
                            })),
                        },
                        InternalSubset::Entity(EntityDeclaration::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "".into()
                                )),
                            }
                        )),
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
    let document = test_valid_sa_file("024")?;
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
                                ContentParticle::Name(
                                    QualifiedName::new(None, "foo"),
                                    ConditionalState::None
                                )
                            )),
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "foo"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            })),
                        },
                        InternalSubset::Entity(EntityDeclaration::General(
                            GeneralEntityDeclaration {
                                name: QualifiedName::new(None, "e"),
                                entity_def: EntityDefinition::EntityValue(EntityValue::Value(
                                    "<foo></foo>".into()
                                )),
                            }
                        )),
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: QualifiedName::new(None, "doc"),
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some("<foo></foo>".into()))),
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
    let document = test_valid_sa_file("025")?;
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
                                ContentParticle::Name(
                                    QualifiedName::new(None, "foo"),
                                    ConditionalState::ZeroOrMore
                                )
                            )),
                        },
                        InternalSubset::Element {
                            name: QualifiedName::new(None, "foo"),
                            content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
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
    let document = test_valid_sa_file("026")?;
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
                                ContentParticle::Name(
                                    QualifiedName::new(None, "foo"),
                                    ConditionalState::ZeroOrMore
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
    let document = test_valid_sa_file("027")?;
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
                                ContentParticle::Name(
                                    QualifiedName::new(None, "foo"),
                                    ConditionalState::ZeroOrMore
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
    let document = test_valid_sa_file("028")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("029")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("030")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("031")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("032")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("033")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("034")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("035")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("036")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("037")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("038")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("039")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("040")?;
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
                                zero_or_more: false,
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
    let document = test_valid_sa_file("041")?;
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
                                zero_or_more: false,
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
    let document = test_valid_sa_file("042")?;
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
                            zero_or_more: false,
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
    let document = test_valid_sa_file("043")?;
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
                                zero_or_more: false,
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
    let document = test_valid_sa_file("044")?;
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
                                ContentParticle::Name(
                                    QualifiedName::new(None, "e"),
                                    ConditionalState::ZeroOrMore,
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
    let document = test_valid_sa_file("045")?;
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
                                zero_or_more: false
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
    let document = test_valid_sa_file("046")?;
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
                                zero_or_more: false
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
    let document = test_valid_sa_file("047")?;
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
                            zero_or_more: false
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
    let document = test_valid_sa_file("048")?;
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
                            zero_or_more: false
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
    let document = test_valid_sa_file("049")?;
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
                            zero_or_more: false
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
