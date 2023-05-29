use super::DeclarationContent;
use crate::{attribute::Attribute, document::ProcessingInstruction, parse::Parse};
use nom::{branch::alt, bytes::complete::tag, multi::many0, IResult};
use std::borrow::Cow;

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

    // [29] markupdecl ::= elementdecl | AttlistDecl | EntityDecl | NotationDecl | PI | Comment
    fn parse_markup_decl(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        alt((
            Self::parse_element,
            Self::parse_attlist,
            //Self::parse_entity,
            //Self::parse_notation,
            Self::parse_processing_instruction,
            //Self::parse_comment,
        ))(input)
    }
}

impl<'a> Parse<'a> for InternalSubset<'a> {}
