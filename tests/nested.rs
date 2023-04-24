use NomExML::{Element, Tag, Namespace};


#[test]
fn test_nested_recursion() {
    let input = "<root><inner_tag1>inner_tag1 content</inner_tag1><inner_tag2>2</inner_tag2><tst:inner_tag3>3</tst:inner_tag3><tst:inner_tag4><inner_inner_tag1>inner_inner_tag1 content</inner_inner_tag1><header>header contents></header><inner_inner_tag1>inner_inner_tag1 content2</inner_inner_tag1><inner_inner_tag2><inner_inner_inner_tag1>inner_inner_inner_tag1 content</inner_inner_inner_tag1></inner_inner_tag2></tst:inner_tag4></root>";
    let (tail, result) = Element::parse_xml_str(input).unwrap();
    println!("result:\n{result:?}");
    println!("tail: {tail:?}");

    assert_eq!(
        result,
        Element::Node(
            Tag::Open("root"),
            Box::new(Element::Nested(vec![
                Element::Node(
                    Tag::Open("inner_tag1"),
                    Box::new(Element::Content("inner_tag1 content")),
                    Tag::Close("inner_tag1")
                ),
                Element::Node(
                    Tag::Open("inner_tag2"),
                    Box::new(Element::Content("2")),
                    Tag::Close("inner_tag2")
                ),
                Element::Node(
                    Tag::NS(Namespace::Prefix("tst"), Box::new(Tag::Open("inner_tag3"))),
                    Box::new(Element::Content("3")),
                    Tag::NS(Namespace::Prefix("tst"), Box::new(Tag::Close("inner_tag3")))
                ),
                Element::Node(
                    Tag::NS(Namespace::Prefix("tst"), Box::new(Tag::Open("inner_tag4"))),
                    Box::new(Element::Nested(vec![
                        Element::Node(
                            Tag::Open("inner_inner_tag1"),
                            Box::new(Element::Content("inner_inner_tag1 content")),
                            Tag::Close("inner_inner_tag1")
                        ),
                        Element::Node(
                            Tag::Open("header"),
                            Box::new(Element::Content("header contents>")),
                            Tag::Close("header")
                        ),
                        Element::Node(
                            Tag::Open("inner_inner_tag1"),
                            Box::new(Element::Content("inner_inner_tag1 content2")),
                            Tag::Close("inner_inner_tag1")
                        ),
                        Element::Node(
                            Tag::Open("inner_inner_tag2"),
                            Box::new(Element::Node(
                                Tag::Open("inner_inner_inner_tag1"),
                                Box::new(Element::Content("inner_inner_inner_tag1 content")),
                                Tag::Close("inner_inner_inner_tag1")
                            )),
                            Tag::Close("inner_inner_tag2")
                        ),
                        
                    ])),
                    Tag::NS(Namespace::Prefix("tst"), Box::new(Tag::Close("inner_tag4")))
                ),
            ])),
            Tag::Close("root")
        )
    );
}