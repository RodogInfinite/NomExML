use std::{borrow::Cow, error::Error, fs::File};

use nomxml::{
    attribute::{AttType, Attribute, DefaultDecl},
    document::{Document, ProcessingInstruction},
    parse_file,
    prolog::{DeclarationContent, DocType, InternalSubset, Mixed},
    tag::{Tag, TagState},
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: "doc".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: "doc".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: "doc".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: "doc".into(),
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
                            name: "doc".into(),
                            att_defs: Some(vec![Attribute::Definition {
                                name: "a1".into(),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: "doc".into(),
                    namespace: None,
                    attributes: Some(vec![Attribute::Instance {
                        name: "a1".into(),
                        value: "v1".into(),
                    }]),
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: "doc".into(),
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
                            name: "doc".into(),
                            att_defs: Some(vec![Attribute::Definition {
                                name: "a1".into(),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: "doc".into(),
                    namespace: None,
                    attributes: Some(vec![Attribute::Instance {
                        name: "a1".into(),
                        value: "v1".into(),
                    }]),
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: "doc".into(),
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
                            name: "doc".into(),
                            att_defs: Some(vec![Attribute::Definition {
                                name: "a1".into(),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: "doc".into(),
                    namespace: None,
                    attributes: Some(vec![Attribute::Instance {
                        name: "a1".into(),
                        value: "v1".into(),
                    }]),
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: "doc".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some(Cow::Owned(" ".into())))),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: "doc".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some(Cow::Owned("&<>\"'".into())))),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: "doc".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Content(Some(Cow::Owned(" ".into())))),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: "doc".into(),
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
                            name: "doc".into(),
                            att_defs: Some(vec![Attribute::Definition {
                                name: "a1".into(),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: "doc".into(),
                    namespace: None,
                    attributes: Some(vec![Attribute::Instance {
                        name: "a1".into(),
                        value: "v1".into(),
                    }]),
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: "doc".into(),
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
                            name: "doc".into(),
                            att_defs: Some(vec![
                                Attribute::Definition {
                                    name: "a1".into(),
                                    att_type: AttType::CDATA,
                                    default_decl: DefaultDecl::Implied,
                                },
                                Attribute::Definition {
                                    name: "a2".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: Some(vec![
                        Attribute::Instance {
                            name: "a1".into(),
                            value: "v1".into(),
                        },
                        Attribute::Instance {
                            name: "a2".into(),
                            value: "v2".into(),
                        },
                    ]),
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: "doc".into(),
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
                            name: "doc".into(),
                            att_defs: Some(vec![Attribute::Definition {
                                name: ":".into(),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: "doc".into(),
                    namespace: None,
                    attributes: Some(vec![Attribute::Instance {
                        name: ":".into(),
                        value: "v1".into(),
                    }]),
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: "doc".into(),
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
                            name: "doc".into(),
                            att_defs: Some(vec![Attribute::Definition {
                                name: "_.-0123456789".into(),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: "doc".into(),
                    namespace: None,
                    attributes: Some(vec![Attribute::Instance {
                        name: "_.-0123456789".into(),
                        value: "v1".into(),
                    }]),
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: "doc".into(),
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
                            name: "doc".into(),
                            att_defs: Some(vec![Attribute::Definition {
                                name: "abcdefghijklmnopqrstuvwxyz".into(),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: "doc".into(),
                    namespace: None,
                    attributes: Some(vec![Attribute::Instance {
                        name: "abcdefghijklmnopqrstuvwxyz".into(),
                        value: "v1".into(),
                    }]),
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![
                        InternalSubset::Element {
                            name: "doc".into(),
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
                            name: "doc".into(),
                            att_defs: Some(vec![Attribute::Definition {
                                name: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".into(),
                                att_type: AttType::CDATA,
                                default_decl: DefaultDecl::Implied,
                            },]),
                        },
                    ]),
                }),
            },
            Document::Element(
                Tag {
                    name: "doc".into(),
                    namespace: None,
                    attributes: Some(vec![Attribute::Instance {
                        name: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".into(),
                        value: "v1".into(),
                    }]),
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: "doc".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::ProcessingInstruction(ProcessingInstruction {
                    target: "pi".into(),
                    data: None,
                })),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: "doc".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Nested(vec![
                    Document::ProcessingInstruction(ProcessingInstruction {
                        target: "pi".into(),
                        data: Some("some data".into()),
                    }),
                    Document::ProcessingInstruction(ProcessingInstruction {
                        target: "x".into(),
                        data: None,
                    }),
                ])),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: "doc".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::ProcessingInstruction(ProcessingInstruction {
                    target: "pi".into(),
                    data: Some("some data ? > <?".into()),
                })),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: "doc".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::CDATA("<foo>".into())),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: "doc".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::CDATA("<&".into())),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: "doc".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::CDATA("<&]>]".into())),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: "doc".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Comment(" a comment ".into())),
                Tag {
                    name: "doc".into(),
                    namespace: None,
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
                doc_type: Some(DocType {
                    name: "doc".into(),
                    external_id: None,
                    int_subset: Some(vec![InternalSubset::Element {
                        name: "doc".into(),
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
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::Start,
                },
                Box::new(Document::Comment(" a comment ->".into())),
                Tag {
                    name: "doc".into(),
                    namespace: None,
                    attributes: None,
                    state: TagState::End,
                },
            ),
        ]),
    );
    Ok(())
}
