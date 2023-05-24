use std::{borrow::Cow, error::Error, fs::File};

use nomexml::{
    attribute::{AttType, Attribute, DefaultDecl},
    declaration::{Declaration, DeclarationContent, Mixed},
    parse_file, ConditionalState, Document, Tag, TagState,
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
            Document::Declaration(Some(Declaration::DocType {
                name: Some("doc".into()),
                external_id: None,
                int_subset: Some(vec![Declaration::Element {
                    name: Some("doc".into()),
                    content_spec: Some(DeclarationContent::Spec {
                        mixed: Mixed::PCDATA {
                            names: None,
                            parsed: true,
                            conditional_state: ConditionalState::None,
                        },
                        children: None,
                    }),
                },]),
            })),
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
            Document::Declaration(Some(Declaration::DocType {
                name: Some("doc".into()),
                external_id: None,
                int_subset: Some(vec![Declaration::Element {
                    name: Some("doc".into()),
                    content_spec: Some(DeclarationContent::Spec {
                        mixed: Mixed::PCDATA {
                            names: None,
                            parsed: true,
                            conditional_state: ConditionalState::None,
                        },
                        children: None,
                    }),
                },]),
            })),
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
            Document::Declaration(Some(Declaration::DocType {
                name: Some("doc".into()),
                external_id: None,
                int_subset: Some(vec![Declaration::Element {
                    name: Some("doc".into()),
                    content_spec: Some(DeclarationContent::Spec {
                        mixed: Mixed::PCDATA {
                            names: None,
                            parsed: true,
                            conditional_state: ConditionalState::None,
                        },
                        children: None,
                    }),
                },]),
            })),
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
            Document::Declaration(Some(Declaration::DocType {
                name: Some("doc".into()),
                external_id: None,
                int_subset: Some(vec![
                    Declaration::Element {
                        name: Some("doc".into()),
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                conditional_state: ConditionalState::None,
                            },
                            children: None,
                        }),
                    },
                    Declaration::AttList {
                        name: Some("doc".into()),
                        att_defs: Some(vec![Attribute::Definition {
                            name: "a1".into(),
                            att_type: AttType::CDATA,
                            default_decl: DefaultDecl::Implied,
                        },]),
                    }
                ])
            })),
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
            Document::Declaration(Some(Declaration::DocType {
                name: Some("doc".into()),
                external_id: None,
                int_subset: Some(vec![
                    Declaration::Element {
                        name: Some("doc".into()),
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                conditional_state: ConditionalState::None,
                            },
                            children: None,
                        }),
                    },
                    Declaration::AttList {
                        name: Some("doc".into()),
                        att_defs: Some(vec![Attribute::Definition {
                            name: "a1".into(),
                            att_type: AttType::CDATA,
                            default_decl: DefaultDecl::Implied,
                        },]),
                    }
                ])
            })),
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
            Document::Declaration(Some(Declaration::DocType {
                name: Some("doc".into()),
                external_id: None,
                int_subset: Some(vec![
                    Declaration::Element {
                        name: Some("doc".into()),
                        content_spec: Some(DeclarationContent::Spec {
                            mixed: Mixed::PCDATA {
                                names: None,
                                parsed: true,
                                conditional_state: ConditionalState::None,
                            },
                            children: None,
                        }),
                    },
                    Declaration::AttList {
                        name: Some("doc".into()),
                        att_defs: Some(vec![Attribute::Definition {
                            name: "a1".into(),
                            att_type: AttType::CDATA,
                            default_decl: DefaultDecl::Implied,
                        },]),
                    }
                ])
            })),
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
            Document::Declaration(Some(Declaration::DocType {
                name: Some("doc".into()),
                external_id: None,
                int_subset: Some(vec![Declaration::Element {
                    name: Some("doc".into()),
                    content_spec: Some(DeclarationContent::Spec {
                        mixed: Mixed::PCDATA {
                            names: None,
                            parsed: true,
                            conditional_state: ConditionalState::None,
                        },
                        children: None,
                    }),
                },]),
            })),
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
            Document::Declaration(Some(Declaration::DocType {
                name: Some("doc".into()),
                external_id: None,
                int_subset: Some(vec![Declaration::Element {
                    name: Some("doc".into()),
                    content_spec: Some(DeclarationContent::Spec {
                        mixed: Mixed::PCDATA {
                            names: None,
                            parsed: true,
                            conditional_state: ConditionalState::None,
                        },
                        children: None,
                    }),
                },]),
            })),
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
