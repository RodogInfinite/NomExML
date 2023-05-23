use nomexml::{Document, Namespace, Tag, TagState};

#[test]
fn test_nested_recursion() {
    let input = "<root><inner_tag1>inner_tag1 content</inner_tag1><inner_tag2>2</inner_tag2><tst:inner_tag3>3</tst:inner_tag3><tst:inner_tag4><inner_inner_tag1>inner_inner_tag1 content</inner_inner_tag1><header>header contents</header><inner_inner_tag1>inner_inner_tag1 content2</inner_inner_tag1><inner_inner_tag2><inner_inner_inner_tag1>inner_inner_inner_tag1 content</inner_inner_inner_tag1></inner_inner_tag2></tst:inner_tag4></root>";

    let (_, result) = Document::parse_xml_str(input).unwrap();

    assert_eq!(
        result,
        Document::Element(
            Tag {
                name: "root".into(),
                namespace: None,
                attributes: None,
                state: TagState::Start,
            },
            Box::new(Document::Nested(vec![
                Document::Element(
                    Tag {
                        name: "inner_tag1".into(),
                        namespace: None,
                        attributes: None,
                        state: TagState::Start,
                    },
                    Box::new(Document::Content(Some("inner_tag1 content".into()))),
                    Tag {
                        name: "inner_tag1".into(),
                        namespace: None,
                        attributes: None,
                        state: TagState::End,
                    },
                ),
                Document::Element(
                    Tag {
                        name: "inner_tag2".into(),
                        namespace: None,
                        attributes: None,
                        state: TagState::Start,
                    },
                    Box::new(Document::Content(Some("2".into()))),
                    Tag {
                        name: "inner_tag2".into(),
                        namespace: None,
                        attributes: None,
                        state: TagState::End,
                    },
                ),
                Document::Element(
                    Tag {
                        name: "inner_tag3".into(),
                        namespace: Some(Namespace {
                            declaration: None,
                            prefix: "tst".into(),
                            uri: None,
                        }),
                        attributes: None,
                        state: TagState::Start,
                    },
                    Box::new(Document::Content(Some("3".into()))),
                    Tag {
                        name: "inner_tag3".into(),
                        namespace: Some(Namespace {
                            declaration: None,
                            prefix: "tst".into(),
                            uri: None,
                        }),
                        attributes: None,
                        state: TagState::End,
                    },
                ),
                Document::Element(
                    Tag {
                        name: "inner_tag4".into(),
                        namespace: Some(Namespace {
                            declaration: None,
                            prefix: "tst".into(),
                            uri: None,
                        }),
                        attributes: None,
                        state: TagState::Start,
                    },
                    Box::new(Document::Nested(vec![
                        Document::Element(
                            Tag {
                                name: "inner_inner_tag1".into(),
                                namespace: None,
                                attributes: None,
                                state: TagState::Start,
                            },
                            Box::new(Document::Content(Some("inner_inner_tag1 content".into()))),
                            Tag {
                                name: "inner_inner_tag1".into(),
                                namespace: None,
                                attributes: None,
                                state: TagState::End,
                            },
                        ),
                        Document::Element(
                            Tag {
                                name: "header".into(),
                                namespace: None,
                                attributes: None,
                                state: TagState::Start,
                            },
                            Box::new(Document::Content(Some("header contents".into()))),
                            Tag {
                                name: "header".into(),
                                namespace: None,
                                attributes: None,
                                state: TagState::End,
                            },
                        ),
                        Document::Element(
                            Tag {
                                name: "inner_inner_tag1".into(),
                                namespace: None,
                                attributes: None,
                                state: TagState::Start,
                            },
                            Box::new(Document::Content(Some("inner_inner_tag1 content2".into()))),
                            Tag {
                                name: "inner_inner_tag1".into(),
                                namespace: None,
                                attributes: None,
                                state: TagState::End,
                            },
                        ),
                        Document::Element(
                            Tag {
                                name: "inner_inner_tag2".into(),
                                namespace: None,
                                attributes: None,
                                state: TagState::Start,
                            },
                            Box::new(Document::Element(
                                Tag {
                                    name: "inner_inner_inner_tag1".into(),
                                    namespace: None,
                                    attributes: None,
                                    state: TagState::Start,
                                },
                                Box::new(Document::Content(Some(
                                    "inner_inner_inner_tag1 content".into()
                                ))),
                                Tag {
                                    name: "inner_inner_inner_tag1".into(),
                                    namespace: None,
                                    attributes: None,
                                    state: TagState::End,
                                },
                            ),),
                            Tag {
                                name: "inner_inner_tag2".into(),
                                namespace: None,
                                attributes: None,
                                state: TagState::End,
                            },
                        ),
                    ])),
                    Tag {
                        name: "inner_tag4".into(),
                        namespace: Some(Namespace {
                            declaration: None,
                            prefix: "tst".into(),
                            uri: None,
                        }),
                        attributes: None,
                        state: TagState::End,
                    },
                ),
            ])),
            Tag {
                name: "root".into(),
                namespace: None,
                attributes: None,
                state: TagState::End,
            },
        )
    );
}
