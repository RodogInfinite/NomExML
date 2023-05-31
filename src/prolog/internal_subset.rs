use super::{external_id::ExternalID, DeclarationContent};
use crate::{
    attribute::Attribute, document::ProcessingInstruction, parse::Parse, reference::Reference,
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

// [71] GEDecl ::= '<!ENTITY' S Name S EntityDef S? '>'
#[derive(Clone, PartialEq)]
pub struct GeneralEntityDeclaration<'a> {
    pub name: Cow<'a, str>,
    pub entity_def: EntityDefinition<'a>,
}

// [73] EntityDef ::= EntityValue | (ExternalID NDataDecl?)
#[derive(Clone, PartialEq)]
pub enum EntityDefinition<'a> {
    EntityValue(EntityValue<'a>),
    External {
        id: ExternalID<'a>,
        n_data: Option<Cow<'a, str>>,
    },
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
// [72] PEDecl ::= '<!ENTITY' S '%' S Name S PEDef S? '>'

#[derive(Clone, PartialEq)]
pub enum InternalSubset<'a> {
    Element {
        name: Cow<'a, str>,
        content_spec: Option<DeclarationContent<'a>>,
    },
    AttList {
        name: Cow<'a, str>,
        att_defs: Option<Vec<Attribute<'a>>>, //Option<Vec<Attribute::Definition>>
    },
    Entity(EntityDeclaration<'a>),
    DeclSep(Cow<'a, str>),
    ProcessingInstruction(ProcessingInstruction<'a>),
}

impl<'a> InternalSubset<'a> {
    // [28b] intSubset ::= (markupdecl | DeclSep)*
    pub fn parse_internal_subset(input: &'a str) -> IResult<&'a str, Vec<InternalSubset<'a>>> {
        many0(alt((Self::parse_markup_decl, Self::parse_decl_sep)))(input)
    }

    // [28a] DeclSep ::= PEReference | S
    // [69] PEReference	::= '%' Name ';'
    fn parse_decl_sep(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        let (input, _) = tag("%")(input)?;
        let (input, name) = Self::parse_name(input)?;
        let (input, _) = tag(";")(input)?;

        let (input, _) = Self::parse_multispace0(input)?;

        Ok((input, InternalSubset::DeclSep(name)))
    }

    // [45] elementdecl	::= '<!ELEMENT' S Name S contentspec S? '>'
    fn parse_element(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        println!("parse_element: {input:?}");
        let (input, _) = tag("<!ELEMENT")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_name(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        // [46]   	contentspec	   ::=   	'EMPTY' | 'ANY' | Mixed | children
        let (input, content_spec) = DeclarationContent::parse_spec(input)?;
        let (input, _) = tag(">")(input)?;
        let (input, _) = Self::parse_multispace0(input)?;

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
    pub fn parse_attlist(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        println!("\n\n\n\nATTLIST PARSE START: {input:?}");
        let (input, _) = tag("<!ATTLIST")(input)?;
        println!("\n\nparse_attlist: {input:?}");
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_name(input)?;
        println!("Parsed name: {:?}", name);

        //// [53] AttDef ::= S Name S AttType S DefaultDecl
        let (input, att_defs) = many0(Attribute::parse_definition)(input)?;
        println!("Parsed attribute definitions: {:?}", att_defs);
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag(">")(input)?;
        println!("\n\n\n\nATTLIST PARSE END: {input:?}");
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
    fn parse_ndata_declaration(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
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
                Reference::EntityRef(value) => value.into_owned(),
                Reference::CharRef { value, .. } => value.into_owned(),
            }),
            map(is_not("%&\"'"), ToString::to_string),
        ))(input)
    }
    // [74] PEDef ::= EntityValue | ExternalID
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
            Self::parse_element,
            Self::parse_attlist,
            Self::parse_entity,
            //Self::parse_notation,
            Self::parse_processing_instruction,
            //Self::parse_comment,
        ))(input)
    }
}

impl<'a> Parse<'a> for InternalSubset<'a> {}
