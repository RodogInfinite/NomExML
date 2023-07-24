use super::{declaration_content::DeclarationContent, external_id::ExternalID};
use crate::{
    attribute::Attribute,
    namespaces::ParseNamespace,
    parse::Parse,
    processing_instruction::ProcessingInstruction,
    reference::{ParseReference, Reference},
    Name, QualifiedName,
};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};
use std::borrow::Cow;

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

// [71] GEDecl ::= '<!ENTITY' S Name S EntityDef S? '>'
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

// [73] EntityDef ::= EntityValue | (ExternalID NDataDecl?)
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

// [74] PEDef ::= EntityValue | ExternalID
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
}

impl<'a> InternalSubset<'a> {
    pub fn get_entity(&self) -> Option<&EntityDeclaration<'a>> {
        match self {
            InternalSubset::Entity(decl) => Some(decl),
            _ => None,
        }
    }
}

impl<'a> Parse<'a> for InternalSubset<'a> {}
impl<'a> ParseNamespace<'a> for InternalSubset<'a> {}

impl<'a> InternalSubset<'a> {
    // [28b] intSubset ::= (markupdecl | DeclSep)*
    pub fn parse_internal_subset(input: &'a str) -> IResult<&'a str, Vec<InternalSubset<'a>>> {
        let mut parsed: Vec<InternalSubset<'a>> = Vec::new();
        let mut current_input = input;

        loop {
            let original_input = current_input;

            match Self::parse_markup_decl(current_input) {
                Ok((new_input, markup_decl)) => {
                    parsed.push(markup_decl);
                    current_input = new_input;
                }
                Err(nom::Err::Error(_)) => match Self::parse_decl_sep(current_input) {
                    Ok((new_input, maybe_decl_sep)) => {
                        if let Some(decl_sep) = maybe_decl_sep {
                            parsed.push(decl_sep);
                        }
                        current_input = new_input;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                },
                Err(e) => {
                    return Err(e);
                }
            }
            if current_input == original_input {
                break;
            }
        }

        Ok((current_input, parsed))
    }

    // [28a] DeclSep ::=  S | PEReference
    fn parse_decl_sep(input: &'a str) -> IResult<&'a str, Option<InternalSubset<'a>>> {
        match Self::parse_multispace0(input) {
            Ok((new_input, _)) => Ok((new_input, None)),
            Err(nom::Err::Error(_)) => match Reference::parse_parameter_reference(input) {
                Ok((new_input, reference)) => {
                    Ok((new_input, Some(InternalSubset::DeclSep(reference))))
                }
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    // [45] elementdecl	::= '<!ELEMENT' S Name S contentspec S? '>'
    fn parse_element_declaration(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        let (input, _) = tag("<!ELEMENT")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_name(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        // [46] contentspec	::= 'EMPTY' | 'ANY' | Mixed | children
        let (input, content_spec) = DeclarationContent::parse_spec(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag(">")(input)?;
        Ok((
            input,
            InternalSubset::Element {
                name,
                content_spec: Some(content_spec),
            },
        ))
    }

    // Namespaces (Third Edition) [17] elementdecl	::= '<!ELEMENT' S QName S contentspec S? '>'
    fn parse_qualified_element_declaration(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        let (input, _) = tag("<!ELEMENT")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_qualified_name(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, content_spec) = DeclarationContent::parse_spec(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag(">")(input)?;
        Ok((
            input,
            InternalSubset::Element {
                name,
                content_spec: Some(content_spec),
            },
        ))
    }
    fn parse_processing_instruction(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        let (input, processing_instruction) = ProcessingInstruction::parse(input)?;
        Ok((
            input,
            InternalSubset::ProcessingInstruction(processing_instruction),
        ))
    }
    // [52] AttlistDecl ::= '<!ATTLIST' S Name AttDef* S? '>'
    pub fn parse_attlist_declaration(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        let (input, _) = tag("<!ATTLIST")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_name(input)?;
        let (input, att_defs) = many0(Attribute::parse_definition)(input)?;
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

    // Namespaces (Third Edition) [20] AttlistDecl ::= '<!ATTLIST' S QName AttDef* S? '>'
    pub fn parse_qualified_attlist_declaration(
        input: &'a str,
    ) -> IResult<&'a str, InternalSubset<'a>> {
        let (input, _) = tag("<!ATTLIST")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_qualified_name(input)?;
        let (input, att_defs) = many0(Attribute::parse_definition)(input)?;
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

    // [70]   	EntityDecl	   ::=   	GEDecl | PEDecl
    fn parse_entity(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        alt((
            Self::parse_general_entity_declaration,
            Self::parse_parameter_entity_declaration,
        ))(input)
    }
    // [71]   	GEDecl	   ::=   	'<!ENTITY' S Name S EntityDef S? '>'
    fn parse_general_entity_declaration(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        let (input, _) = tag("<!ENTITY")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_name(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, entity_def) = Self::parse_entity_def(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag(">")(input)?;
        Ok((
            input,
            InternalSubset::Entity(EntityDeclaration::General(GeneralEntityDeclaration {
                name,
                entity_def,
            })),
        ))
    }

    // [72]    PEDecl ::=    '<!ENTITY' S '%' S Name S PEDef S? '>'
    fn parse_parameter_entity_declaration(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        let (input, _) = tag("<!ENTITY")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, _) = tag("%")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_name(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, pedef) = Self::parse_parameter_definition(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag(">")(input)?;

        Ok((
            input,
            InternalSubset::Entity(EntityDeclaration::Parameter(pedef)),
        ))
    }

    // [74] PEDef ::= EntityValue | ExternalID
    fn parse_parameter_definition(
        input: &'a str,
    ) -> IResult<&'a str, ParameterEntityDefinition<'a>> {
        alt((
            map(
                Self::parse_entity_value,
                ParameterEntityDefinition::EntityValue,
            ),
            map(ExternalID::parse, ParameterEntityDefinition::ExternalID),
        ))(input)
    }

    // [73]   	EntityDef	   ::=   	EntityValue | (ExternalID NDataDecl?)
    fn parse_entity_def(input: &'a str) -> IResult<&'a str, EntityDefinition<'a>> {
        alt((
            map(Self::parse_entity_value, EntityDefinition::EntityValue),
            map(
                tuple((ExternalID::parse, opt(Self::parse_ndata_declaration))),
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
    fn parse_entity_value(input: &'a str) -> IResult<&'a str, EntityValue<'a>> {
        let (input, data) = alt((
            delimited(tag("\""), many0(Self::parse_entity_content), tag("\"")),
            delimited(tag("\'"), many0(Self::parse_entity_content), tag("\'")),
        ))(input)?;

        let value = data.into_iter().collect::<String>();
        Ok((input, EntityValue::Value(Cow::Owned(value))))
    }

    fn parse_entity_content(input: &'a str) -> IResult<&'a str, String> {
        alt((
            map(Reference::parse, |reference| match reference {
                Reference::EntityRef(value) => value.local_part.into_owned(),
                Reference::CharRef { value, .. } => value.into_owned(),
            }),
            map(is_not("%&\"'"), ToString::to_string),
        ))(input)
    }

    // [74] PEDef ::= EntityValue | ExternalID
    fn parse_perameter_definition(
        input: &'a str,
    ) -> IResult<&'a str, ParameterEntityDefinition<'a>> {
        alt((
            map(
                Self::parse_entity_value,
                ParameterEntityDefinition::EntityValue,
            ),
            map(ExternalID::parse, ParameterEntityDefinition::ExternalID),
        ))(input)
    }

    // [29] markupdecl ::= elementdecl | AttlistDecl | EntityDecl | NotationDecl | PI | Comment
    fn parse_markup_decl(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        alt((
            Self::parse_element_declaration,
            Self::parse_attlist_declaration,
            Self::parse_entity,
            //Self::parse_notation,
            Self::parse_processing_instruction,
            //Self::parse_comment,
        ))(input)
    }
}
