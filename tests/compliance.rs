use std::{borrow::Cow, error::Error, fs::File};

use nom_xml::{
    attribute::{AttType, Attribute, DefaultDecl},
    io::parse_file,
    processing_instruction::ProcessingInstruction,
    prolog::{
        declaration_content::{DeclarationContent, Mixed},
        doctype::DocType,
        internal_subset::InternalSubset,
    },
    tag::{Tag, TagState},
    Document, QualifiedName,
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
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            },
                            children: None,
                        }),
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
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            },
                            children: None,
                        }),
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
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            },
                            children: None,
                        }),
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
                            content_spec: Some(DeclarationContent::Spec {
                                mixed: Mixed::PCDATA {
                                    names: None,
                                    parsed: true,
                                    zero_or_more: false,
                                },
                                children: None,
                            }),
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
                            content_spec: Some(DeclarationContent::Spec {
                                mixed: Mixed::PCDATA {
                                    names: None,
                                    parsed: true,
                                    zero_or_more: false,
                                },
                                children: None,
                            }),
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
                            content_spec: Some(DeclarationContent::Spec {
                                mixed: Mixed::PCDATA {
                                    names: None,
                                    parsed: true,
                                    zero_or_more: false,
                                },
                                children: None,
                            }),
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
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            },
                            children: None,
                        }),
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
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            },
                            children: None,
                        }),
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
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            },
                            children: None,
                        }),
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
                            content_spec: Some(DeclarationContent::Spec {
                                mixed: Mixed::PCDATA {
                                    names: None,
                                    parsed: true,
                                    zero_or_more: false,
                                },
                                children: None,
                            }),
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
                            content_spec: Some(DeclarationContent::Spec {
                                mixed: Mixed::PCDATA {
                                    names: None,
                                    parsed: true,
                                    zero_or_more: false,
                                },
                                children: None,
                            }),
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
                            content_spec: Some(DeclarationContent::Spec {
                                mixed: Mixed::PCDATA {
                                    names: None,
                                    parsed: true,
                                    zero_or_more: false,
                                },
                                children: None,
                            }),
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
                            content_spec: Some(DeclarationContent::Spec {
                                mixed: Mixed::PCDATA {
                                    names: None,
                                    parsed: true,
                                    zero_or_more: false,
                                },
                                children: None,
                            }),
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
                            content_spec: Some(DeclarationContent::Spec {
                                mixed: Mixed::PCDATA {
                                    names: None,
                                    parsed: true,
                                    zero_or_more: false,
                                },
                                children: None,
                            }),
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
                            content_spec: Some(DeclarationContent::Spec {
                                mixed: Mixed::PCDATA {
                                    names: None,
                                    parsed: true,
                                    zero_or_more: false,
                                },
                                children: None,
                            }),
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
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            },
                            children: None,
                        }),
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
                    target: QualifiedName {
                        prefix: None,
                        local_part: "pi".into()
                    },
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
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            },
                            children: None,
                        }),
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
                        target: QualifiedName {
                            prefix: None,
                            local_part: "pi".into()
                        },
                        data: Some("some data ".into()),
                    }),
                    Document::ProcessingInstruction(ProcessingInstruction {
                        target: QualifiedName {
                            prefix: None,
                            local_part: "x".into()
                        },
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
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            },
                            children: None,
                        }),
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
                    target: QualifiedName {
                        prefix: None,
                        local_part: "pi".into()
                    },
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
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            },
                            children: None,
                        }),
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
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            },
                            children: None,
                        }),
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
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            },
                            children: None,
                        }),
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
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            },
                            children: None,
                        }),
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
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                zero_or_more: false,
                            },
                            children: None,
                        }),
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
