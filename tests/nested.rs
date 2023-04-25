use nomexml::{Element, Namespace};

#[test]
fn test_nested_recursion() {
    let input = "<root><inner_tag1>inner_tag1 content</inner_tag1><inner_tag2>2</inner_tag2><tst:inner_tag3>3</tst:inner_tag3><tst:inner_tag4><inner_inner_tag1>inner_inner_tag1 content</inner_inner_tag1><header>header contents</header><inner_inner_tag1>inner_inner_tag1 content2</inner_inner_tag1><inner_inner_tag2><inner_inner_inner_tag1>inner_inner_inner_tag1 content</inner_inner_inner_tag1></inner_inner_tag2></tst:inner_tag4></root>";

    let (_, result) = Element::parse_xml_str(input).unwrap();

    assert_eq!(
        result,
        Element::Node(
            Box::new(Element::Tag {
                open: true,
                close: true,
                name: "root",
                namespace: None,
            }),
            Box::new(Element::Nested(vec![
                Element::Node(
                    Box::new(Element::Tag {
                        open: true,
                        close: true,
                        name: "inner_tag1",
                        namespace: None,
                    }),
                    Box::new(Element::Content(Some("inner_tag1 content"))),
                    Box::new(Element::Tag {
                        open: true,
                        close: true,
                        name: "inner_tag1",
                        namespace: None,
                    })
                ),
                Element::Node(
                    Box::new(Element::Tag {
                        open: true,
                        close: true,
                        name: "inner_tag2",
                        namespace: None,
                    }),
                    Box::new(Element::Content(Some("2"))),
                    Box::new(Element::Tag {
                        open: true,
                        close: true,
                        name: "inner_tag2",
                        namespace: None,
                    })
                ),
                Element::Node(
                    Box::new(Element::Tag {
                        open: true,
                        close: true,
                        name: "inner_tag3",
                        namespace: Some(Namespace {
                            prefix: "tst",
                            uri: None
                        }),
                    }),
                    Box::new(Element::Content(Some("3"))),
                    Box::new(Element::Tag {
                        open: true,
                        close: true,
                        name: "inner_tag3",
                        namespace: Some(Namespace {
                            prefix: "tst",
                            uri: None
                        }),
                    })
                ),
                Element::Node(
                    Box::new(Element::Tag {
                        open: true,
                        close: true,
                        name: "inner_tag4",
                        namespace: Some(Namespace {
                            prefix: "tst",
                            uri: None
                        }),
                    }),
                    Box::new(Element::Nested(vec![
                        Element::Node(
                            Box::new(Element::Tag {
                                open: true,
                                close: true,
                                name: "inner_inner_tag1",
                                namespace: None,
                            }),
                            Box::new(Element::Content(Some("inner_inner_tag1 content"))),
                            Box::new(Element::Tag {
                                open: true,
                                close: true,
                                name: "inner_inner_tag1",
                                namespace: None,
                            })
                        ),
                        Element::Node(
                            Box::new(Element::Tag {
                                open: true,
                                close: true,
                                name: "header",
                                namespace: None,
                            }),
                            Box::new(Element::Content(Some("header contents"))),
                            Box::new(Element::Tag {
                                open: true,
                                close: true,
                                name: "header",
                                namespace: None,
                            })
                        ),
                        Element::Node(
                            Box::new(Element::Tag {
                                open: true,
                                close: true,
                                name: "inner_inner_tag1",
                                namespace: None,
                            }),
                            Box::new(Element::Content(Some("inner_inner_tag1 content2"))),
                            Box::new(Element::Tag {
                                open: true,
                                close: true,
                                name: "inner_inner_tag1",
                                namespace: None,
                            })
                        ),
                        Element::Node(
                            Box::new(Element::Tag {
                                open: true,
                                close: true,
                                name: "inner_inner_tag2",
                                namespace: None,
                            }),
                            Box::new(Element::Node(
                                Box::new(Element::Tag {
                                    open: true,
                                    close: true,
                                    name: "inner_inner_inner_tag1",
                                    namespace: None,
                                }),
                                Box::new(Element::Content(Some("inner_inner_inner_tag1 content"))),
                                Box::new(Element::Tag {
                                    open: true,
                                    close: true,
                                    name: "inner_inner_inner_tag1",
                                    namespace: None,
                                })
                            )),
                            Box::new(Element::Tag {
                                open: true,
                                close: true,
                                name: "inner_inner_tag2",
                                namespace: None,
                            })
                        ),
                    ])),
                    Box::new(Element::Tag {
                        open: true,
                        close: true,
                        name: "inner_tag4",
                        namespace: Some(Namespace {
                            prefix: "tst",
                            uri: None
                        }),
                    })
                ),
            ])),
            Box::new(Element::Tag {
                open: true,
                close: true,
                name: "root",
                namespace: None,
            }),
        )
    );
}
