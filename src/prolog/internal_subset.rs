use super::{declaration_content::DeclarationContent, external_id::ExternalID};
use crate::{
    attribute::Attribute,
    namespaces::ParseNamespace,
    parse::Parse,
    processing_instruction::ProcessingInstruction,
    reference::{ParseReference, Reference},
    Document, Name, QualifiedName,
};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};
use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, PartialEq)]
pub enum EntityValue<'a> {
    Value(Cow<'a, str>),
    Reference(Reference<'a>),
    PerameterReference(Reference<'a>),
}

impl<'a> EntityValue<'a> {
    pub fn get_value(&self) -> Option<Cow<'a, str>> {
        match self {
            EntityValue::Value(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_reference(&self) -> Option<&Reference<'a>> {
        if let EntityValue::Reference(reference) = self {
            Some(reference)
        } else {
            None
        }
    }

    pub fn get_perameter_reference(&self) -> Option<&Reference<'a>> {
        if let EntityValue::PerameterReference(reference) = self {
            Some(reference)
        } else {
            None
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct GeneralEntityDeclaration<'a> {
    pub name: Name<'a>,
    pub entity_def: EntityDefinition<'a>,
}

impl<'a> GeneralEntityDeclaration<'a> {
    pub fn find_name(&self, name: Name<'a>) -> Option<&GeneralEntityDeclaration<'a>> {
        if self.name == name {
            Some(self)
        } else {
            None
        }
    }

    pub fn get_name(&self) -> &Name<'a> {
        &self.name
    }

    pub fn get_entity_def(&self) -> &EntityDefinition<'a> {
        &self.entity_def
    }
}

#[derive(Clone, PartialEq)]
pub enum EntityDefinition<'a> {
    EntityValue(EntityValue<'a>),
    External {
        id: ExternalID<'a>,
        n_data: Option<Name<'a>>,
    },
}

impl<'a> EntityDefinition<'a> {
    pub fn get_entity_value(&self) -> Option<&EntityValue<'a>> {
        if let EntityDefinition::EntityValue(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_external_id(&self) -> Option<&ExternalID<'a>> {
        if let EntityDefinition::External { id, .. } = self {
            Some(id)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParameterEntityDefinition<'a> {
    EntityValue(EntityValue<'a>),
    ExternalID(ExternalID<'a>),
}

#[derive(Clone, PartialEq)]
pub enum EntityDeclaration<'a> {
    General(GeneralEntityDeclaration<'a>),
    Parameter(ParameterEntityDefinition<'a>),
}

impl<'a> EntityDeclaration<'a> {
    pub fn get_general(&self) -> Option<&GeneralEntityDeclaration<'a>> {
        match self {
            EntityDeclaration::General(decl) => Some(decl),
            _ => None,
        }
    }

    pub fn get_parameter(&self) -> Option<&ParameterEntityDefinition<'a>> {
        match self {
            EntityDeclaration::Parameter(decl) => Some(decl),
            _ => None,
        }
    }

    pub fn find_general_entity_value(&self, name: Name<'a>) -> Option<&EntityValue<'a>> {
        if let EntityDeclaration::General(decl) = self {
            if decl.name == name {
                return decl.entity_def.get_entity_value();
            }
        }
        None
    }
}

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
    Entity(EntityDeclaration<'a>),
    DeclSep(Reference<'a>),
    ProcessingInstruction(ProcessingInstruction<'a>),
    Comment(Document<'a>),
}

impl<'a> InternalSubset<'a> {
    pub fn get_entity(&self) -> Option<&EntityDeclaration<'a>> {
        match self {
            InternalSubset::Entity(decl) => Some(decl),
            _ => None,
        }
    }
}

impl<'a> ParseNamespace<'a> for InternalSubset<'a> {}

impl<'a> Parse<'a> for InternalSubset<'a> {
    type Args = Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>;
    type Output = IResult<&'a str, Vec<InternalSubset<'a>>>;
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        println!("\n-----\nPARSING INTERNAL SUBSET");
        println!("INITIAL PARSE INTERNAL SUBSET INPUT: {}", input);

        let (input, parsed) = many0(tuple((
            |i| Self::parse_markup_decl(i, args.clone()),
            opt(Self::parse_decl_sep),
        )))(input)?;

        let mut consolidated: Vec<InternalSubset<'a>> = vec![];
        for (markup, opt_decl_sep) in parsed {
            if let InternalSubset::AttList {
                name,
                att_defs: Some(new_defs),
            } = &markup
            {
                if let Some(existing) = consolidated.iter_mut().find(|i| {
                    matches!(i, InternalSubset::AttList { name: existing_name, .. } if *existing_name == *name)
                }) {
                    if let InternalSubset::AttList { att_defs: Some(existing_defs), .. } = existing {
                        existing_defs.extend(new_defs.clone());  // note that you might need to clone new_defs
                    }
                    continue;
                }
            }
            consolidated.push(markup);
            if let Some(Some(decl_sep)) = opt_decl_sep {
                consolidated.push(decl_sep);
            }
        }

        println!("PARSED INTERNAL SUBSET: {:?}", consolidated);
        println!("INPUT AFTER PARSED INTERNAL SUBSET: {}", input);
        Ok((input, consolidated))
    }
}
impl<'a> InternalSubset<'a> {
    // [28a] DeclSep ::=  S | PEReference
    fn parse_decl_sep(input: &'a str) -> IResult<&'a str, Option<InternalSubset<'a>>> {
        alt((
            map(Self::parse_multispace0, |_| None),
            map(Reference::parse_parameter_reference, |reference| {
                Some(InternalSubset::DeclSep(reference))
            }),
        ))(input)
    }

    // [45] elementdecl	::= '<!ELEMENT' S Name S contentspec S? '>'
    // Namespaces (Third Edition) [17] elementdecl	::= '<!ELEMENT' S QName S contentspec S? '>'
    fn parse_element_declaration(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        println!("\n-----\nPARSING ELEMENT DECLARATION");
        let (input, _) = tag("<!ELEMENT")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = alt((Self::parse_name, Self::parse_qualified_name))(input)?;
        println!("PARSED ELEMENT DECL NAME: {name:?}");
        let (input, _) = Self::parse_multispace1(input)?;
        println!("BEFORE CONTENT SPEC INPUT: {input}");
        let (input, content_spec) = DeclarationContent::parse(input, ())?;
        println!("PARSED CONTENT SPEC: {content_spec:?}");
        let (input, _) = Self::parse_multispace0(input)?;
        println!("BEFORE TAG INPUT: {input}");
        let (input, _) = tag(">")(input)?;
        println!("PARSED ELEMENT DECLARATION");
        Ok((
            input,
            InternalSubset::Element {
                name,
                content_spec: Some(content_spec),
            },
        ))
    }

    fn parse_processing_instruction(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        println!("\n-----\nPARSING PROCESSING INSTRUCTION");
        let (input, processing_instruction) = ProcessingInstruction::parse(input, ())?;
        println!("PARSED PROCESSING INSTRUCTION\n");
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
        let (input, _) = tag("<!ATTLIST")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = alt((Self::parse_name, Self::parse_qualified_name))(input)?;
        let (input, att_defs) =
            many0(|i| Attribute::parse_definition(i, entity_references.clone()))(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag(">")(input)?;
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
        println!("\n-----\nPARSING GENERAL ENTITY DECLARATION");
        println!("GEN ENTITY DECL INPUT: {input:?}\n");
        let (input, _) = tag("<!ENTITY")(input)?;
        println!("PARSED <!ENTITY");
        let (input, _) = Self::parse_multispace1(input)?;
        println!("\nHERE\n");
        let (input, name) = Self::parse_name(input)?;
        println!("GENERAL ENTITY DECL NAME: {name:?}");
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, entity_def) = Self::parse_entity_def(input, entity_references.clone())?;
        println!("PARSED ENTITY DEF");
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag(">")(input)?;
        println!("PARSED GENERAL ENTITY DECLARATION");
        Ok((
            input,
            InternalSubset::Entity(EntityDeclaration::General(GeneralEntityDeclaration {
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
        println!("\n-----\nPARSING PARAMETER ENTITY DECLARATION");
        let (input, _) = tag("<!ENTITY")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, _) = tag("%")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_name(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, pedef) = Self::parse_parameter_definition(input, entity_references.clone())?;
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag(">")(input)?;
        println!("PARSED PARAMETER ENTITY DECLARATION");
        Ok((
            input,
            InternalSubset::Entity(EntityDeclaration::Parameter(pedef)),
        ))
    }

    // [74] PEDef ::= EntityValue | ExternalID
    fn parse_parameter_definition(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, ParameterEntityDefinition<'a>> {
        alt((
            map(
                |i| Self::parse_entity_value(i, entity_references.clone()),
                ParameterEntityDefinition::EntityValue,
            ),
            map(
                |i| ExternalID::parse(i, ()),
                ParameterEntityDefinition::ExternalID,
            ),
        ))(input)
    }

    //TODO: dig into this, this is probably causing the failure
    // [73] EntityDef ::= EntityValue | (ExternalID NDataDecl?)
    fn parse_entity_def(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, EntityDefinition<'a>> {
        println!("PARSING ENTITY DEF");
        alt((
            map(
                |i| Self::parse_entity_value(i, entity_references.clone()),
                EntityDefinition::EntityValue,
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
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, EntityValue<'a>> {
        println!("\n-----\nPARSING ENTITY VALUE");
        let (input, data) = alt((
            delimited(
                tag("\""),
                many0(alt((map(is_not("%&\""), ToString::to_string), |i| {
                    Self::parse_entity_content(i, entity_references.clone())
                }))),
                tag("\""),
            ),
            delimited(
                tag("\'"),
                many0(alt((map(is_not("%&'"), ToString::to_string), |i| {
                    Self::parse_entity_content(i, entity_references.clone())
                }))),
                tag("\'"),
            ),
        ))(input)?;

        let value = data.into_iter().collect::<String>();
        Ok((input, EntityValue::Value(Cow::Owned(value))))
    }

    fn parse_entity_content(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, String> {
        let (input, reference) = Reference::parse(input, entity_references.clone())?;
        let result = match reference {
            Reference::EntityRef(value) => value.local_part.into_owned(),
            Reference::CharRef { value, .. } => value.into_owned(),
        };
        Ok((input, result))
    }

    // [74] PEDef ::= EntityValue | ExternalID
    fn parse_perameter_definition(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, ParameterEntityDefinition<'a>> {
        alt((
            map(
                |i| Self::parse_entity_value(i, entity_references.clone()),
                ParameterEntityDefinition::EntityValue,
            ),
            map(
                |i| ExternalID::parse(i, ()),
                ParameterEntityDefinition::ExternalID,
            ),
        ))(input)
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
    ) -> IResult<&'a str, InternalSubset<'a>> {
        println!("PARSING MARKUP DECL INPUT: {input}");
        alt((
            Self::parse_element_declaration,
            |i| Self::parse_attlist_declaration(i, entity_references.clone()),
            |i| Self::parse_entity(i, entity_references.clone()),
            //Self::parse_notation,
            Self::parse_processing_instruction,
            Self::parse_comment,
        ))(input)
    }
}
