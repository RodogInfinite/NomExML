use std::{error::Error, fs::File};

use nomexml::{
    declaration::{Declaration, DeclarationContent, Mixed},
    parse_file, ConditionalState, Document, Tag, TagState,
};

fn test_xml_file<'a>(file_number: &str) -> Result<Document<'a>, Box<dyn Error>> {
    let mut file = File::open(format!("tests/xmltest/valid/sa/{file_number}.xml"))?;
    let document = parse_file(&mut file)?;
    Ok(document)
}

#[test]
fn test_valid_sa_001() -> Result<(), Box<dyn Error>> {
    let document = test_xml_file("001")?;
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
                Tag::Tag {
                    name: "doc".into(),
                    namespace: None,
                    state: TagState::Start,
                },
                Box::new(Document::Empty),
                Tag::Tag {
                    name: "doc".into(),
                    namespace: None,
                    state: TagState::End,
                },
            ),
        ]),
    );
    Ok(())
}

#[test]
fn test_valid_sa_002() {
    let document = test_xml_file("002");
}
