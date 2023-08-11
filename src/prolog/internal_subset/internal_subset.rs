use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
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

use super::{
    entity_declaration::{EntityDeclaration, ParameterEntityDefinition},
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
        dbg!(&input, "InternalSubset::parse input");

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

        dbg!(&consolidated);
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
        dbg!(&input, "parse_element_declaration input");
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
        dbg!(&input, "parse_notation input");

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
        dbg!(&input, "parse_processing_instruction input");
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
        dbg!(&input, "parse_general_entity_declaration input");

        let (input, (_start, _whitespace1, name, _whitespace2, entity_def, _whitespace3, _close)) =
            tuple((
                tag("<!ENTITY"),
                Self::parse_multispace1,
                Self::parse_name,
                Self::parse_multispace1,
                move |i| Self::parse_entity_def(i, entity_references.clone()),
                Self::parse_multispace0,
                tag(">"),
            ))(input)?;

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
        dbg!(&input, "parse_parameter_entity_declaration input");

        let (
            input,
            (
                _start,
                _whitespace1,
                _percent,
                _whitespace2,
                _name, // Note: We can reintroduce the handling of this value if needed in the future.
                _whitespace3,
                pedef,
                _whitespace4,
                _close,
            ),
        ) = tuple((
            tag("<!ENTITY"),
            Self::parse_multispace1,
            tag("%"),
            Self::parse_multispace1,
            Self::parse_name,
            Self::parse_multispace1,
            move |i| Self::parse_parameter_definition(i, entity_references.clone()),
            Self::parse_multispace0,
            tag(">"),
        ))(input)?;

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
        dbg!(&input, "parse_entity_def input");
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
        dbg!(&input, "parse_entity_value input");
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

    //TODO: figure out how to integrate this
    // [74] PEDef ::= EntityValue | ExternalID
    fn _parse_perameter_definition(
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
        dbg!(&input, "parse_markup_decl input");
        alt((
            Self::parse_element_declaration,
            |i| Self::parse_attlist_declaration(i, entity_references.clone()),
            |i| Self::parse_entity(i, entity_references.clone()),
            Self::parse_notation,
            Self::parse_processing_instruction,
            Self::parse_comment,
        ))(input)
    }
}
