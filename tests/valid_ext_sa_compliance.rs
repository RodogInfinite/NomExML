// use nom_xml::{
//     io::parse_file,
//     prolog::{
//         content_particle::ContentParticle,
//         declaration_content::{DeclarationContent, Mixed},
//         doctype::DocType,
//         external_id::ExternalID,
//         subset::{
//             entity_declaration::{EntityDecl, GeneralEntityDeclaration},
//             entity_definition::EntityDefinition,
//             internal::InternalSubset,
//             markup_declaration::MarkupDeclaration,
//         },
//         textdecl::TextDecl,
//     },
//     tag::{Tag, TagState},
//     ConditionalState, Config, Document, ExternalEntityParseConfig, QualifiedName,
// };
// use std::{error::Error, fs::File};
// fn test_valid_sa_file(file_number: &str, config: Config) -> Result<Document, Box<dyn Error>> {
//     let mut file = File::open(format!("tests/xmltest/valid/ext-sa/{file_number}.xml"))?;

//     let document = parse_file(&mut file, config)?;
//     Ok(document)
// }

// #[test]
// fn test_valid_sa_001() -> Result<(), Box<dyn Error>> {
//     let document = test_valid_sa_file(
//         "001",
//         Config {
//             external_parse_config: ExternalEntityParseConfig {
//                 allow_ext_parse: true,
//                 ignore_ext_parse_warning: true,
//                 base_directory: Some("tests/xmltest/valid/ext-sa".into()),
//             },
//         },
//     )?;
//     assert_eq!(
//         document,
//         Document::Nested(vec![
//             Document::Prolog {
//                 xml_decl: None,

//                 misc: None,
//                 doc_type: Some(DocType {
//                     name: QualifiedName::new(None, "doc"),
//                     external_id: None,
//                     int_subset: Some(vec![
//                         InternalSubset::MarkupDecl(Some(MarkupDeclaration::Element {
//                             name: QualifiedName::new(None, "doc"),
//                             content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
//                         })),
//                         InternalSubset::MarkupDecl(Some(MarkupDeclaration::Entity(
//                             EntityDecl::General(GeneralEntityDeclaration {
//                                 name: QualifiedName::new(None, "e"),
//                                 entity_def: EntityDefinition::External {
//                                     id: ExternalID::System("001.ent".to_string()),
//                                     n_data: None,
//                                     text_decl: None
//                                 },
//                             })
//                         ))),
//                     ]),
//                 }),
//             },
//             Document::Element(
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::Start,
//                 },
//                 Box::new(Document::Content(Some("Data\n".to_string()))),
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::End,
//                 },
//             ),
//         ]),
//     );
//     Ok(())
// }

// #[test]
// fn test_valid_sa_002() -> Result<(), Box<dyn Error>> {
//     let document = test_valid_sa_file(
//         "002",
//         Config {
//             external_parse_config: ExternalEntityParseConfig {
//                 allow_ext_parse: true,
//                 ignore_ext_parse_warning: true,
//                 base_directory: Some("tests/xmltest/valid/ext-sa".into()),
//             },
//         },
//     )?;
//     assert_eq!(
//         document,
//         Document::Nested(vec![
//             Document::Prolog {
//                 xml_decl: None,

//                 misc: None,
//                 doc_type: Some(DocType {
//                     name: QualifiedName::new(None, "doc"),
//                     external_id: None,
//                     int_subset: Some(vec![
//                         InternalSubset::Element {
//                             name: QualifiedName::new(None, "doc"),
//                             content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
//                         },
//                         InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
//                             name: QualifiedName::new(None, "e"),
//                             entity_def: EntityDefinition::External {
//                                 id: ExternalID::System("002.ent".to_string()),
//                                 n_data: None,
//                             },
//                         })),
//                     ]),
//                 }),
//             },
//             Document::Element(
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::Start,
//                 },
//                 Box::new(Document::Content(Some("Data".to_string()))),
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::End,
//                 },
//             ),
//         ]),
//     );
//     Ok(())
// }

// #[test]
// fn test_valid_sa_003() -> Result<(), Box<dyn Error>> {
//     let document = test_valid_sa_file(
//         "003", // adjusted for the new test
//         Config {
//             external_parse_config: ExternalEntityParseConfig {
//                 allow_ext_parse: true,
//                 ignore_ext_parse_warning: true,
//                 base_directory: Some("tests/xmltest/valid/ext-sa".into()),
//             },
//         },
//     )?;
//     assert_eq!(
//         document,
//         Document::Nested(vec![
//             Document::Prolog {
//                 xml_decl: None,

//                 misc: None,
//                 doc_type: Some(DocType {
//                     name: QualifiedName::new(None, "doc"),
//                     external_id: None,
//                     int_subset: Some(vec![
//                         InternalSubset::Element {
//                             name: QualifiedName::new(None, "doc"),
//                             content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
//                         },
//                         InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
//                             name: QualifiedName::new(None, "e"),
//                             entity_def: EntityDefinition::External {
//                                 id: ExternalID::System("003.ent".to_string()),
//                                 n_data: None,
//                             },
//                         })),
//                     ]),
//                 }),
//             },
//             Document::Element(
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::Start,
//                 },
//                 Box::new(Document::Empty),
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::End,
//                 },
//             ),
//         ]),
//     );
//     Ok(())
// }

// #[test]
// fn test_valid_sa_004() -> Result<(), Box<dyn Error>> {
//     let document = test_valid_sa_file(
//         "004", // adjusted for the new test
//         Config {
//             external_parse_config: ExternalEntityParseConfig {
//                 allow_ext_parse: true,
//                 ignore_ext_parse_warning: true,
//                 base_directory: Some("tests/xmltest/valid/ext-sa".into()),
//             },
//         },
//     )?;

//     assert_eq!(
//         document,
//         Document::Nested(vec![
//             Document::Prolog {
//                 xml_decl: None,

//                 misc: None,
//                 doc_type: Some(DocType {
//                     name: QualifiedName::new(None, "doc"),
//                     external_id: None,
//                     int_subset: Some(vec![
//                         InternalSubset::Element {
//                             name: QualifiedName::new(None, "doc"),
//                             content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
//                         },
//                         InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
//                             name: QualifiedName::new(None, "e"),
//                             entity_def: EntityDefinition::External {
//                                 id: ExternalID::System("004.ent".to_string()),
//                                 n_data: None,
//                             },
//                         })),
//                     ]),
//                 }),
//             },
//             Document::Element(
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::Start,
//                 },
//                 Box::new(Document::Content(Some("Data\n".to_string()))),
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::End,
//                 },
//             ),
//         ]),
//     );

//     Ok(())
// }

// #[test]
// fn test_valid_sa_005() -> Result<(), Box<dyn Error>> {
//     let document = test_valid_sa_file(
//         "005", // adjusted for the new test
//         Config {
//             external_parse_config: ExternalEntityParseConfig {
//                 allow_ext_parse: true,
//                 ignore_ext_parse_warning: true,
//                 base_directory: Some("tests/xmltest/valid/ext-sa".into()),
//             },
//         },
//     )?;
//     assert_eq!(
//         document,
//         Document::Nested(vec![
//             Document::Prolog {
//                 xml_decl: None,

//                 misc: None,
//                 doc_type: Some(DocType {
//                     name: QualifiedName::new(None, "doc"),
//                     external_id: None,
//                     int_subset: Some(vec![
//                         InternalSubset::Element {
//                             name: QualifiedName::new(None, "doc"),
//                             content_spec: Some(DeclarationContent::Children(
//                                 ContentParticle::Sequence(
//                                     vec![ContentParticle::Name(
//                                         QualifiedName::new(None, "e"),
//                                         ConditionalState::ZeroOrMore
//                                     )],
//                                     ConditionalState::None
//                                 )
//                             )),
//                         },
//                         InternalSubset::Element {
//                             name: QualifiedName::new(None, "e"),
//                             content_spec: Some(DeclarationContent::Empty),
//                         },
//                         InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
//                             name: QualifiedName::new(None, "e"),
//                             entity_def: EntityDefinition::External {
//                                 id: ExternalID::System("005.ent".to_string()),
//                                 n_data: None,
//                             },
//                         })),
//                     ]),
//                 }),
//             },
//             Document::Element(
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::Start,
//                 },
//                 Box::new(Document::Nested(vec![
//                     Document::EmptyTag(Tag {
//                         name: QualifiedName::new(None, "e"),
//                         attributes: None,
//                         state: TagState::Empty,
//                     }),
//                     Document::EmptyTag(Tag {
//                         name: QualifiedName::new(None, "e"),
//                         attributes: None,
//                         state: TagState::Empty,
//                     }),
//                     Document::EmptyTag(Tag {
//                         name: QualifiedName::new(None, "e"),
//                         attributes: None,
//                         state: TagState::Empty,
//                     }),
//                 ])),
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::End,
//                 },
//             ),
//         ]),
//     );

//     Ok(())
// }

// #[test]
// fn test_valid_sa_006() -> Result<(), Box<dyn Error>> {
//     let document = test_valid_sa_file(
//         "006",
//         Config {
//             external_parse_config: ExternalEntityParseConfig {
//                 allow_ext_parse: true,
//                 ignore_ext_parse_warning: true,
//                 base_directory: Some("tests/xmltest/valid/ext-sa".into()),
//             },
//         },
//     )?;
//     assert_eq!(
//         document,
//         Document::Nested(vec![
//             Document::Prolog {
//                 xml_decl: None,

//                 misc: None,
//                 doc_type: Some(DocType {
//                     name: QualifiedName::new(None, "doc"),
//                     external_id: None,
//                     int_subset: Some(vec![
//                         InternalSubset::Element {
//                             name: QualifiedName::new(None, "doc"),
//                             content_spec: Some(DeclarationContent::Mixed(Mixed::Names(vec![
//                                 QualifiedName::new(None, "e")
//                             ]))),
//                         },
//                         InternalSubset::Element {
//                             name: QualifiedName::new(None, "e"),
//                             content_spec: Some(DeclarationContent::Empty),
//                         },
//                         InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
//                             name: QualifiedName::new(None, "e"),
//                             entity_def: EntityDefinition::External {
//                                 id: ExternalID::System("006.ent".to_string()),
//                                 n_data: None,
//                             },
//                         })),
//                     ]),
//                 }),
//             },
//             Document::Element(
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::Start,
//                 },
//                 Box::new(Document::Nested(vec![
//                     Document::Content(Some("Data\n".to_string())),
//                     Document::EmptyTag(Tag {
//                         name: QualifiedName::new(None, "e"),
//                         attributes: None,
//                         state: TagState::Empty,
//                     }),
//                     Document::Content(Some("More data\n".to_string())),
//                     Document::EmptyTag(Tag {
//                         name: QualifiedName::new(None, "e"),
//                         attributes: None,
//                         state: TagState::Empty,
//                     }),
//                 ])),
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::End,
//                 },
//             ),
//         ]),
//     );

//     Ok(())
// }

// #[test]
// fn test_valid_sa_007() -> Result<(), Box<dyn Error>> {
//     let document = test_valid_sa_file(
//         "007",
//         Config {
//             external_parse_config: ExternalEntityParseConfig {
//                 allow_ext_parse: true,
//                 ignore_ext_parse_warning: true,
//                 base_directory: Some("tests/xmltest/valid/ext-sa".into()),
//             },
//         },
//     )?;

//     assert_eq!(
//         document,
//         Document::Nested(vec![
//             Document::Prolog {
//                 xml_decl: None,

//                 misc: None,
//                 doc_type: Some(DocType {
//                     name: QualifiedName::new(None, "doc"),
//                     external_id: None,
//                     int_subset: Some(vec![
//                         InternalSubset::Element {
//                             name: QualifiedName::new(None, "doc"),
//                             content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
//                         },
//                         InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
//                             name: QualifiedName::new(None, "e"),
//                             entity_def: EntityDefinition::External {
//                                 id: ExternalID::System("007.ent".to_string()),
//                                 n_data: None,
//                             },
//                         })),
//                     ]),
//                 }),
//             },
//             Document::Element(
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::Start,
//                 },
//                 Box::new(Document::Nested(vec![
//                     Document::Content(Some("X".to_string())),
//                     Document::Content(Some("Y".to_string())), // Here's the data from the new .ent file
//                     Document::Content(Some("Z".to_string())),
//                 ])),
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::End,
//                 },
//             ),
//         ]),
//     );

//     Ok(())
// }

// #[test]
// fn test_valid_sa_008() -> Result<(), Box<dyn Error>> {
//     let document = test_valid_sa_file(
//         "008",
//         Config {
//             external_parse_config: ExternalEntityParseConfig {
//                 allow_ext_parse: true,
//                 ignore_ext_parse_warning: true,
//                 base_directory: Some("tests/xmltest/valid/ext-sa".into()),
//             },
//         },
//     )?;

//     assert_eq!(
//         document,
//         Document::Nested(vec![
//             Document::Prolog {
//                 xml_decl: None,

//                 misc: None,
//                 doc_type: Some(DocType {
//                     name: QualifiedName::new(None, "doc"),
//                     external_id: None,
//                     int_subset: Some(vec![
//                         InternalSubset::Element {
//                             name: QualifiedName::new(None, "doc"),
//                             content_spec: Some(DeclarationContent::Mixed(Mixed::PCDATA)),
//                         },
//                         InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
//                             name: QualifiedName::new(None, "e"),
//                             entity_def: EntityDefinition::External {
//                                 id: ExternalID::System("008.ent".to_string()), // Update to 008.ent
//                                 n_data: None,
//                             },
//                         })),
//                     ]),
//                 }),
//             },
//             Document::Element(
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::Start,
//                 },
//                 Box::new(Document::Nested(vec![
//                     Document::Content(Some("X".to_string())),
//                     Document::Content(Some("Y".to_string())), // The data from the 008.ent file
//                     Document::Content(Some("Z".to_string())),
//                 ])),
//                 Tag {
//                     name: QualifiedName::new(None, "doc"),
//                     attributes: None,
//                     state: TagState::End,
//                 },
//             ),
//         ]),
//     );

//     Ok(())
// }

// // where should this go?
// // TextDecl {
// // version: None,
// // encoding: "UTF-16".to_string(),
// // },
