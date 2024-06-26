use std::{cell::RefCell, collections::HashMap, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    combinator::{map, map_res, opt},
    multi::{fold_many1, many0, many1},
    sequence::tuple,
};

use crate::{
    attribute::Attribute,
    error,
    namespaces::ParseNamespace,
    parse::Parse,
    processing_instruction::ProcessingInstruction,
    prolog::{declaration_content::DeclarationContent, external_id::ExternalID, id::ID},
    reference::Reference,
    Document, IResult, Name,
};

use super::entity::{
    entity_declaration::{EntityDecl, GeneralEntityDeclaration, ParameterEntityDeclaration},
    entity_definition::EntityDefinition,
    entity_value::EntityValue,
    EntitySource,
};

#[derive(Clone, PartialEq, Eq)]
pub enum MarkupDeclaration {
    Element {
        name: Name,
        content_spec: Option<DeclarationContent>,
    },
    AttList {
        name: Name,
        att_defs: Option<Vec<Attribute>>,
    },
    Entity(EntityDecl),
    Notation {
        name: Name,
        id: ID,
    },
    ProcessingInstruction(ProcessingInstruction),
    Comment(Document),
}
impl<'a> ParseNamespace<'a> for MarkupDeclaration {}

impl<'a> Parse<'a> for MarkupDeclaration {
    type Args = (
        Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        EntitySource,
    );
    type Output = IResult<&'a str, Option<MarkupDeclaration>>;
    // [29] markupdecl ::= elementdecl | AttlistDecl | EntityDecl | NotationDecl | PI | Comment
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        let (entity_references, entity_source) = args;

        let (input, res) = opt(alt((
            Self::parse_element_declaration,
            |i| {
                Self::parse_attlist_declaration(i, entity_references.clone(), entity_source.clone())
            },
            |i| Self::parse_entity(i, entity_references.clone(), entity_source.clone()),
            Self::parse_notation,
            Self::parse_processing_instruction,
            Self::parse_comment,
        )))(input)?;
        Ok((input, res))
    }
}

impl MarkupDeclaration {
    // [45] elementdecl	::= '<!ELEMENT' S Name S contentspec S? '>'
    // Namespaces (Third Edition) [17] elementdecl	::= '<!ELEMENT' S QName S contentspec S? '>'
    fn parse_element_declaration(input: &str) -> IResult<&str, MarkupDeclaration> {
        let (
            input,
            (_element, _whitespace1, name, _whitespace2, content_spec, _whitespace, _close),
        ) = tuple((
            tag("<!ELEMENT"),
            Self::parse_multispace1,
            alt((Self::parse_name, Self::parse_qualified_name)),
            Self::parse_multispace1,
            |i| DeclarationContent::parse(i, ()),
            Self::parse_multispace0,
            tag(">"),
        ))(input)?;

        Ok((
            input,
            MarkupDeclaration::Element {
                name,
                content_spec: Some(content_spec),
            },
        ))
    }

    // [82] NotationDecl ::= '<!NOTATION' S Name S (ExternalID | PublicID) S? '>'	[VC: Unique Notation Name]
    fn parse_notation(input: &str) -> IResult<&str, MarkupDeclaration> {
        let (input, (_notation, _whitespace1, name, _whitespace2, id, _whitespace3, _close)) =
            tuple((
                tag("<!NOTATION"),
                Self::parse_multispace1,
                alt((Self::parse_name, Self::parse_qualified_name)),
                Self::parse_multispace1,
                |i| ID::parse(i, ()),
                Self::parse_multispace0,
                tag(">"),
            ))(input)?;

        Ok((input, MarkupDeclaration::Notation { name, id }))
    }

    fn parse_processing_instruction(input: &str) -> IResult<&str, MarkupDeclaration> {
        let (input, processing_instruction) = ProcessingInstruction::parse(input, ())?;
        Ok((
            input,
            MarkupDeclaration::ProcessingInstruction(processing_instruction),
        ))
    }
    // [52] AttlistDecl ::= '<!ATTLIST' S Name AttDef* S? '>'
    // Namespaces (Third Edition) [20] AttlistDecl ::= '<!ATTLIST' S QName AttDef* S? '>'
    pub fn parse_attlist_declaration(
        input: &str,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> IResult<&str, MarkupDeclaration> {
        let (input, (_start, _whitespace1, name, att_defs, _whitespace2, _close)) =
            tuple((
                tag("<!ATTLIST"),
                Self::parse_multispace1,
                alt((Self::parse_name, Self::parse_qualified_name)),
                many0(|i| {
                    Attribute::parse_definition(i, entity_references.clone(), entity_source.clone())
                }),
                Self::parse_multispace0,
                tag(">"),
            ))(input)?;
        Ok((
            input,
            MarkupDeclaration::AttList {
                name,
                att_defs: Some(att_defs),
            },
        ))
    }

    // [70] EntityDecl ::= GEDecl | PEDecl
    fn parse_entity(
        input: &str,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> IResult<&str, MarkupDeclaration> {
        alt((
            |i| {
                Self::parse_general_entity_declaration(
                    i,
                    entity_references.clone(),
                    entity_source.clone(),
                )
            },
            |i| {
                Self::parse_parameter_entity_declaration(
                    i,
                    entity_references.clone(),
                    entity_source.clone(),
                )
            },
        ))(input)
    }

    // [71] GEDecl ::= '<!ENTITY' S Name S EntityDef S? '>'
    fn parse_general_entity_declaration(
        input: &str,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> IResult<&str, MarkupDeclaration> {
        let (input, (_start, _whitespace1, name, _whitespace2)) = tuple((
            tag("<!ENTITY"),
            Self::parse_multispace1,
            Self::parse_name,
            Self::parse_multispace1,
        ))(input)?;

        let (input, (entity_def, _whitespace3, _close)) = tuple((
            |i| {
                Self::parse_entity_definition(
                    i,
                    name.clone(),
                    entity_references.clone(),
                    entity_source.clone(),
                )
            },
            Self::parse_multispace0,
            tag(">"),
        ))(input)?;
        Ok((
            input,
            MarkupDeclaration::Entity(EntityDecl::General(GeneralEntityDeclaration {
                name,
                entity_def,
            })),
        ))
    }

    // [72]    PEDecl ::=    '<!ENTITY' S '%' S Name S PEDef S? '>'
    fn parse_parameter_entity_declaration(
        input: &str,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> IResult<&str, MarkupDeclaration> {
        let (input, (_start, _whitespace1, _percent, _whitespace2, name, _whitespace3)) =
            tuple((
                tag("<!ENTITY"),
                Self::parse_multispace1,
                tag("%"),
                Self::parse_multispace1,
                Self::parse_name,
                Self::parse_multispace1,
            ))(input)?;

        let (input, (entity_def, _whitespace4, _close)) = tuple((
            |i| {
                Self::parse_parameter_definition(
                    i,
                    name.clone(),
                    entity_references.clone(),
                    entity_source.clone(),
                )
            },
            Self::parse_multispace0,
            tag(">"),
        ))(input)?;

        Ok((
            input,
            MarkupDeclaration::Entity(EntityDecl::Parameter(ParameterEntityDeclaration {
                name,
                entity_def,
            })),
        ))
    }

    // [74] PEDef ::= EntityValue | ExternalID
    fn parse_parameter_definition(
        input: &str,
        name: Name,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> IResult<&str, EntityDefinition> {
        alt((
            map(
                |i| {
                    Self::parse_entity_value(
                        i,
                        name.clone(),
                        entity_references.clone(),
                        entity_source.clone(),
                    )
                },
                EntityDefinition::EntityValue,
            ),
            map(
                |i| ExternalID::parse(i, ()),
                |id| EntityDefinition::External {
                    id,
                    n_data: None,
                    text_decl: None,
                },
            ),
        ))(input)
    }

    // [73] EntityDef ::= EntityValue | (ExternalID NDataDecl?)
    fn parse_entity_definition(
        input: &str,
        name: Name,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> IResult<&str, EntityDefinition> {
        alt((
            map(
                |i| {
                    Self::parse_entity_value(
                        i,
                        name.clone(),
                        entity_references.clone(),
                        entity_source.clone(),
                    )
                },
                EntityDefinition::EntityValue,
            ),
            map(
                tuple((
                    |i| ExternalID::parse(i, ()),
                    opt(Self::parse_ndata_declaration),
                )),
                |(id, n_data)| EntityDefinition::External {
                    id,
                    n_data,
                    text_decl: None,
                },
            ),
        ))(input)
    }

    // [76] NDataDecl ::= S 'NDATA' S Name
    fn parse_ndata_declaration(input: &str) -> IResult<&str, Name> {
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, _) = tag("NDATA")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_name(input)?;

        Ok((input, name))
    }
    // [9] EntityValue	::= '"' ([^%&"] | PEReference | Reference)* '"'|  "'" ([^%&'] | PEReference | Reference)* "'"
    pub fn parse_entity_value(
        input: &str,
        name: Name,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> IResult<&str, EntityValue> {
        //TODO: I hate this. Refactor is possible
        let cloned_references = entity_references.clone();
        let cloned_references2 = entity_references.clone();

        let cloned_entity_source = entity_source.clone();
        let cloned_entity_source2 = entity_source.clone();
        // TODO: removed alt((alt(()))) need to test if functionality is the same
        alt((
            map(
                tuple((
                    alt((char('\"'), char('\''))),
                    Self::capture_span(alt((
                        move |i| Document::parse_element(i, cloned_references.clone()),
                        Document::parse_cdata_section,
                    ))),
                    alt((char('\"'), char('\''))),
                )),
                |(_, (raw_entity_value, doc), _)| {
                    entity_references.borrow_mut().insert(
                        (name.clone(), EntitySource::Internal),
                        EntityValue::Document(doc),
                    );
                    EntityValue::Value(raw_entity_value.to_string())
                },
            ),
            map_res(
                tuple((
                    alt((char('\"'), char('\''))),
                    Self::capture_span(move |i| {
                        Self::parse(i, (cloned_references2.clone(), entity_source.clone()))
                    }),
                    alt((char('\"'), char('\''))),
                )),
                |(_, (raw_internal_subset, data), _)| match data {
                    Some(data) => {
                        entity_references.borrow_mut().insert(
                            (name.clone(), EntitySource::Internal),
                            EntityValue::MarkupDecl(Box::new(data)),
                        );

                        Ok(EntityValue::Value(raw_internal_subset.to_string()))
                    }
                    None => Err(nom::Err::Failure((
                        "No Internal Subset",
                        nom::error::ErrorKind::Fail,
                    ))),
                },
            ),
            map(
                tuple((
                    tag("\""),
                    opt(many1(alt((
                        map(
                            move |i| Reference::parse(i, cloned_entity_source.clone()),
                            EntityValue::Reference,
                        ),
                        map(
                            fold_many1(
                                map(is_not("%&\""), |s: &str| s.to_string()),
                                String::new,
                                |mut acc: String, item: String| {
                                    acc.push_str(&item);
                                    acc
                                },
                            ),
                            EntityValue::Value,
                        ),
                    )))),
                    tag("\""),
                )),
                |(_, maybe_entities, _)| {
                    let mut buffer = String::new();
                    if let Some(entities) = maybe_entities {
                        match entities.as_slice() {
                            [EntityValue::Reference(_)] => return entities[0].clone(),
                            _ => {
                                for entity in entities {
                                    match entity {
                                        EntityValue::Reference(reference) => {
                                            let ref_string = Self::get_reference_value(reference);
                                            buffer.push_str(&ref_string);
                                        }
                                        EntityValue::Value(val) => {
                                            buffer.push_str(&val);
                                        }
                                        _ => {} // Handle other possible variants if needed.
                                    }
                                }
                            }
                        }
                    }
                    EntityValue::Value(buffer)
                },
            ),
            map(
                tuple((
                    tag("\'"),
                    opt(many1(alt((
                        map(
                            move |i| Reference::parse(i, cloned_entity_source2.clone()),
                            EntityValue::Reference,
                        ),
                        map(
                            fold_many1(
                                map(is_not("%&'"), |s: &str| s.to_string()),
                                String::new,
                                |mut acc: String, item: String| {
                                    acc.push_str(&item);
                                    acc
                                },
                            ),
                            EntityValue::Value,
                        ),
                    )))),
                    tag("\'"),
                )),
                |(_, maybe_entities, _)| {
                    let mut buffer = String::new();

                    if let Some(entities) = maybe_entities {
                        match entities.as_slice() {
                            [EntityValue::Reference(_)] => return entities[0].clone(),
                            _ => {
                                for entity in entities {
                                    match entity {
                                        EntityValue::Reference(reference) => {
                                            let ref_string = Self::get_reference_value(reference);
                                            buffer.push_str(&ref_string);
                                        }
                                        EntityValue::Value(val) => {
                                            buffer.push_str(&val);
                                        }
                                        _ => {} // Handle other possible variants if needed.
                                    }
                                }
                            }
                        }
                    }
                    EntityValue::Value(buffer)
                },
            ),
        ))(input)
    }

    fn parse_comment(input: &str) -> IResult<&str, MarkupDeclaration> {
        let (remaining, doc) = Document::parse_comment(input)?;
        match doc {
            Document::Comment(comment) => Ok((
                remaining,
                MarkupDeclaration::Comment(Document::Comment(comment)),
            )),
            e => {
                eprintln!("{e:?}");
                Err(nom::Err::Error(error::Error::NomError(
                    nom::error::Error::new(
                        "parse_comment` unexpected Document".to_string(),
                        nom::error::ErrorKind::Verify,
                    ),
                )))
            }
        }
    }
    fn get_reference_value(reference: Reference) -> String {
        match reference {
            Reference::EntityRef(value) => value.local_part,
            Reference::CharRef(value) => value,
        }
    }
}
