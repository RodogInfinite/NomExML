pub mod entity_declaration;
pub mod entity_definition;
pub mod entity_value;

use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    combinator::{map, opt, map_res},
    multi::{fold_many0, many0, many1, fold_many1},
    sequence::{delimited, tuple},
    IResult, Parser,
};

use crate::{
    attribute::Attribute,
    namespaces::ParseNamespace,
    parse::Parse,
    processing_instruction::ProcessingInstruction,
    prolog::{
        declaration_content::DeclarationContent, external_id::ExternalID, id::ID,
        internal_subset::entity_declaration::GeneralEntityDeclaration,
    },
    reference::{ParseReference, Reference},
    Document, Name, QualifiedName,
};

use self::{
    entity_declaration::{EntityDecl, EntityDeclaration, ParameterEntityDeclaration},
    entity_definition::EntityDefinition,
    entity_value::EntityValue,
};

#[derive(Clone, PartialEq)]
pub enum InternalSubset<'a> {
    Element {
        name: QualifiedName<'a>,
        content_spec: Option<DeclarationContent<'a>>,
    },
    AttList {
        name: QualifiedName<'a>,
        att_defs: Option<Vec<Attribute<'a>>>,
    },
    Notation {
        name: QualifiedName<'a>,
        id: ID<'a>,
    },
    Entity(EntityDecl<'a>),
    Entities(Vec<Box<InternalSubset<'a>>>),
    DeclSep {
        reference: Reference<'a>,
        expansion: Option<Box<InternalSubset<'a>>>,
    },
    ProcessingInstruction(ProcessingInstruction<'a>),
    Comment(Document<'a>),
}

impl<'a> InternalSubset<'a> {
    pub fn get_entity(&self) -> Option<&EntityDeclaration<'a>> {
        match self {
            InternalSubset::Entity(decl) => match decl {
                EntityDecl::General(general_decl) => Some(general_decl),
                EntityDecl::Parameter(parameter_decl) => Some(parameter_decl),
            },
            _ => None,
        }
    }
}

impl<'a> ParseNamespace<'a> for InternalSubset<'a> {}

impl<'a> Parse<'a> for InternalSubset<'a> {
    type Args = Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>;
    type Output = IResult<&'a str, Vec<InternalSubset<'a>>>;

    //[28b]	intSubset ::= (markupdecl | DeclSep)*
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
                
        let (input, parsed) = many0(alt((
            |i| Self::parse_markup_decl(i, args.clone()),
            |i| Self::parse_decl_sep(i,args.clone()),
        )))(input)?;

                                
        let mut consolidated: Vec<InternalSubset<'a>> = vec![];

        for opt_internal_subset in parsed {
            if let Some(InternalSubset::AttList { name, att_defs: Some(new_defs) }) = &opt_internal_subset {
                if let Some(existing) = consolidated.iter_mut().find(|i| {
                    matches!(i, InternalSubset::AttList { name: existing_name, .. } if existing_name == name)
                }) {
                    if let InternalSubset::AttList { att_defs: Some(existing_defs), .. } = existing {
                        existing_defs.extend(new_defs.clone()); 
                    }
                    continue;
                }
            }
            if let Some(internal_subset) = opt_internal_subset.clone() {
                consolidated.push(internal_subset);
            }
        }

                        Ok((input, consolidated))
    }
}


impl<'a> InternalSubset<'a> {
    fn expand_entity(reference: &Reference<'a>, entity_references: &Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>) -> Option<EntityValue<'a>> {
        match reference {
            Reference::EntityRef(name) => {
                let entities = entity_references.borrow();
                entities.get(name).cloned()
            },
            Reference::CharRef(_) => {
                // Handle character references here if needed
                None
            }
        }
    }
    
    // [28a] DeclSep ::=  PEReference | S 
    fn parse_decl_sep(input: &'a str, entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>) -> IResult<&'a str, Option<InternalSubset<'a>>> {
                                        alt((
            map(Reference::parse_parameter_reference, |reference| {
                let expansion = Self::expand_entity(&reference, &entity_references);
                let expanded_internal_subset = match &expansion {
                    Some(EntityValue::InternalSubset(elem)) => Some(elem.clone()),
                    _ => None,
                };
                Some(InternalSubset::DeclSep {
                    reference,
                    expansion: expanded_internal_subset
                })
            }),
            map(Self::parse_multispace1, |_| None),
        ))(input)
    }
    
    

    // [45] elementdecl	::= '<!ELEMENT' S Name S contentspec S? '>'
    // Namespaces (Third Edition) [17] elementdecl	::= '<!ELEMENT' S QName S contentspec S? '>'
    fn parse_element_declaration(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
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
            InternalSubset::Element {
                name,
                content_spec: Some(content_spec),
            },
        ))
    }

    // [82] NotationDecl ::= '<!NOTATION' S Name S (ExternalID | PublicID) S? '>'	[VC: Unique Notation Name]
    fn parse_notation(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
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

        Ok((input, InternalSubset::Notation { name, id }))
    }

    fn parse_processing_instruction(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        let (input, processing_instruction) = ProcessingInstruction::parse(input, ())?;
        Ok((
            input,
            InternalSubset::ProcessingInstruction(processing_instruction),
        ))
    }
    // [52] AttlistDecl ::= '<!ATTLIST' S Name AttDef* S? '>'
    // Namespaces (Third Edition) [20] AttlistDecl ::= '<!ATTLIST' S QName AttDef* S? '>'
    pub fn parse_attlist_declaration(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, InternalSubset<'a>> {
        let (input, (_start, _whitespace1, name, att_defs, _whitespace2, _close)) =
            tuple((
                tag("<!ATTLIST"),
                Self::parse_multispace1,
                alt((Self::parse_name, Self::parse_qualified_name)),
                many0(|i| Attribute::parse_definition(i, entity_references.clone())),
                Self::parse_multispace0,
                tag(">"),
            ))(input)?;

        Ok((
            input,
            InternalSubset::AttList {
                name,
                att_defs: Some(att_defs),
            },
        ))
    }

    // [70] EntityDecl ::= GEDecl | PEDecl
    fn parse_entity(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, InternalSubset<'a>> {
        alt((
            |i| Self::parse_general_entity_declaration(i, entity_references.clone()),
            |i| Self::parse_parameter_entity_declaration(i, entity_references.clone()),
        ))(input)
    }

    // [71] GEDecl ::= '<!ENTITY' S Name S EntityDef S? '>'
    fn parse_general_entity_declaration(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, InternalSubset<'a>> {
        let (input, (_start, _whitespace1, name, _whitespace2)) =
            tuple((
                tag("<!ENTITY"),
                Self::parse_multispace1,
                Self::parse_name,
                Self::parse_multispace1
            ))(input)?;
    
        let (input, (entity_def, _whitespace3, _close)) =
            tuple((
                |i| Self::parse_entity_definition(i, name.clone(), entity_references.clone()),
                Self::parse_multispace0,
                tag(">")
            ))(input)?;
                        Ok((
            input,
            InternalSubset::Entity(EntityDecl::General(GeneralEntityDeclaration {
                name,
                entity_def,
            })),
        ))
    }
    
    

    // [72]    PEDecl ::=    '<!ENTITY' S '%' S Name S PEDef S? '>'
    fn parse_parameter_entity_declaration(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, InternalSubset<'a>> {
                        
        let (input, (_start, _whitespace1, _percent, _whitespace2, name, _whitespace3)) = 
            tuple((
                tag("<!ENTITY"),
                Self::parse_multispace1,
                tag("%"),
                Self::parse_multispace1,
                Self::parse_name,
                Self::parse_multispace1
            ))(input)?;
    
        let (input, (entity_def, _whitespace4, _close)) = 
            tuple((
                |i| Self::parse_parameter_definition(i, name.clone(), entity_references.clone()),
                Self::parse_multispace0,
                tag(">")
            ))(input)?;
    
                            
        Ok((
            input,
            InternalSubset::Entity(EntityDecl::Parameter(ParameterEntityDeclaration {
                name,
                entity_def,
            })),
        ))
    }
    

    // [74] PEDef ::= EntityValue | ExternalID
    fn parse_parameter_definition(
        input: &'a str,
        name: Name<'a>,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, EntityDefinition<'a>> {
                        alt((
            map(
                |i| Self::parse_entity_value(i, name.clone(),entity_references.clone()),
                |val|{
                                                        EntityDefinition::EntityValue(val)}
            ),
            map(
                |i| ExternalID::parse(i, ()),
                |id| EntityDefinition::External { id, n_data: None },
            ),
        ))(input)
    }

    //TODO: dig into this, this is probably causing the failure
    // [73] EntityDef ::= EntityValue | (ExternalID NDataDecl?)
    fn parse_entity_definition(
        input: &'a str,
        name: Name<'a>,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, EntityDefinition<'a>> {
                        alt((
            map(
                |i| Self::parse_entity_value(i, name.clone(), entity_references.clone()), |val|{
                                                            //
                        EntityDefinition::EntityValue(val)
                    //}
                }
            ),
            map(
                tuple((
                    |i| ExternalID::parse(i, ()),
                    opt(Self::parse_ndata_declaration),
                )),
                |(id, n_data)| EntityDefinition::External { id, n_data },
            ),
        ))(input)
    }

    // [76] NDataDecl ::= S 'NDATA' S Name
    fn parse_ndata_declaration(input: &'a str) -> IResult<&'a str, Name<'a>> {
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, _) = tag("NDATA")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_name(input)?;

        Ok((input, name))
    }


    // [9] EntityValue	::= '"' ([^%&"] | PEReference | Reference)* '"'|  "'" ([^%&'] | PEReference | Reference)* "'"
    fn parse_entity_value(
        input: &'a str,
        name: Name<'a>,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, EntityValue<'a>> {
                
        let cloned_references = entity_references.clone();
        let cloned_references2 = entity_references.clone();
        alt((
            alt((
                map(
                    tuple((
                        alt((char('\"'), char('\''))),
                        Self::capture_span(alt((
                            move |i| Document::parse_element(i, cloned_references.clone()),
                            Document::parse_cdata_section
                        ))),
                        alt((char('\"'), char('\''))),
                    )),
                    |(_, (raw_entity_value, doc), _)| {
                                                                        entity_references.borrow_mut().insert(name.clone(), EntityValue::Document(doc));
                
                        // Return the original string
                        EntityValue::Value(Cow::Owned(raw_entity_value.to_string()))
                    },
                ),
                map_res(
                    tuple((
                        alt((char('\"'), char('\''))),
                        Self::capture_span( move |i| Self::parse_markup_decl(i, cloned_references2.clone())),
                        alt((char('\"'), char('\''))),
                    )),
                    |(_, (raw_internal_subset, data), _)| {
                        
                                                                        match data {
                            Some(data) => {
                                entity_references.borrow_mut().insert(name.clone(), EntityValue::InternalSubset(Box::new(data)));
                                Ok(EntityValue::Value(Cow::Owned(raw_internal_subset.to_string())))},
                            None => Err(nom::Err::Failure(("No Internal Subset", nom::error::ErrorKind::Fail))),
                        }
                    }
                ),
                map(
                    tuple((
                        tag("\""),
                        opt(many1(alt((
                            map(|i| Reference::parse(i, entity_references.clone()), EntityValue::Reference),
                            map(
                                fold_many1(
                                    map(is_not("%&\""), |s: &str| s.to_string()),
                                    String::new,
                                    |mut acc: String, item: String| {
                                        acc.push_str(&item);
                                        acc
                                    },
                                ),
                                |data| {
                                                                                                            EntityValue::Value(Cow::Owned(data))
                                }
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
                                            },
                                            EntityValue::Value(val) => {
                                                buffer.push_str(&val);
                                            },
                                            _ => {} // Handle other possible variants if needed.
                                        }
                                    }
                                }
                            }
                        }
                        EntityValue::Value(Cow::Owned(buffer))
                    }
                ),
                
                map(
                    tuple((
                        tag("\'"),
                        opt(many1(alt((
                            map(|i| Reference::parse(i, entity_references.clone()), EntityValue::Reference),
                            map(
                                fold_many1(
                                    map(is_not("%&'"), |s: &str| s.to_string()),
                                    String::new,
                                    |mut acc: String, item: String| {
                                        acc.push_str(&item);
                                        acc
                                    },
                                ),
                                |data| {
                                                                                                            EntityValue::Value(Cow::Owned(data))
                                }
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
                                            },
                                            EntityValue::Value(val) => {
                                                buffer.push_str(&val);
                                            },
                                            _ => {} // Handle other possible variants if needed.
                                        }
                                    }
                                }
                            }
                        }
                        EntityValue::Value(Cow::Owned(buffer))
                    }
                ),
                
                
            )),
            
           

            ))(input)
    }

    fn get_reference_value(reference: Reference<'a>) -> String {
                        match reference {
            Reference::EntityRef(value) => value.local_part.into_owned(),
            Reference::CharRef(value) => value.into_owned(),
        }
    }

    fn parse_comment(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        let (remaining, doc) = Document::parse_comment(input)?;
        match doc {
            Document::Comment(comment) => Ok((
                remaining,
                InternalSubset::Comment(Document::Comment(comment)),
            )),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            ))),
        }
    }

    // [29] markupdecl ::= elementdecl | AttlistDecl | EntityDecl | NotationDecl | PI | Comment
    fn parse_markup_decl(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, Option<InternalSubset<'a>>> {
                        map(
            alt((
                Self::parse_element_declaration,
                |i| Self::parse_attlist_declaration(i, entity_references.clone()),
                |i| Self::parse_entity(i, entity_references.clone()),
                Self::parse_notation,
                Self::parse_processing_instruction,
                Self::parse_comment,
            )),
            Some,
        )(input)
    }
    
}
