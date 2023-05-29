use std::borrow::Cow;

use crate::{attribute::Attribute, tag::ConditionalState, utils::Parse, document::ProcessingInstruction};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit1},
    combinator::{map, opt, value},
    multi::{many0, separated_list1, many1},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ExternalID {
    Public,
    System,
}

// #[derive(Clone, Debug, PartialEq)]
// pub enum ExternalID<'a> {
//     System(Cow<'a, str>),
//     Public {
//         pubid: Cow<'a, str>,
//         system_identifier: Cow<'a, str>,
//     },
//     NData(Cow<'a, str>),
// }

#[derive(Clone, Debug, PartialEq)]
pub enum ContentParticle<'a> {
    Particle {
        names: Option<Vec<Cow<'a, str>>>,
        choice: Option<Vec<ContentParticle<'a>>>,
        sequence: Option<Vec<ContentParticle<'a>>>,
        conditional_state: Option<ConditionalState>,
    },
}

impl<'a> Parse<'a> for ContentParticle<'a> {}

impl<'a> ContentParticle<'a> {
    // cp ::= (Name | choice | seq) ('?' | '*' | '+')?
    fn parse_content_particle(input: &'a str) -> IResult<&'a str, ContentParticle<'a>> {
        let (input, names) = opt(many0(Self::parse_name))(input)?;
        let (input, choice) = opt(Self::parse_choice)(input)?;
        let (input, sequence) = opt(Self::parse_seq)(input)?;
        let (input, conditional_state) = opt(Self::parse_conditional_state)(input)?;

        let content_particle = ContentParticle::Particle {
            names,
            choice,
            sequence,
            conditional_state,
        };

        Ok((input, content_particle))
    }

    // choice ::= '(' S? cp ( S? '|' S? cp )+ S? ')'
    fn parse_choice(input: &'a str) -> IResult<&'a str, Vec<ContentParticle<'a>>> {
        let inner = separated_list1(
            tuple((Self::parse_multispace0, tag("|"), Self::parse_multispace0)),
            Self::parse_content_particle,
        );
        let mut parser = delimited(
            tuple((tag("("), Self::parse_multispace0)),
            inner,
            tuple((Self::parse_multispace0, tag(")"))),
        );
        let (input, choice) = parser(input)?;
        Ok((input, choice))
    }

    // seq ::= '(' S? cp ( S? ',' S? cp )* S? ')'
    fn parse_seq(input: &'a str) -> IResult<&'a str, Vec<ContentParticle<'a>>> {
        let inner = separated_list1(
            tuple((Self::parse_multispace0, tag(","), Self::parse_multispace0)),
            Self::parse_content_particle,
        );
        let mut parser = delimited(
            tuple((tag("("), Self::parse_multispace0)),
            inner,
            tuple((Self::parse_multispace0, tag(")"))),
        );
        let (input, sequence) = parser(input)?;
        Ok((input, sequence))
    }

    fn parse_conditional_state(input: &'a str) -> IResult<&'a str, ConditionalState> {
        alt((
            value(ConditionalState::Optional, tag("?")),
            value(ConditionalState::ZeroOrMore, tag("*")),
            value(ConditionalState::OneOrMore, tag("+")),
        ))(input)
    }
}

#[derive(Clone, PartialEq)]
pub enum Mixed<'a> {
    PCDATA {
        names: Option<Vec<Cow<'a, str>>>,
        parsed: bool,
        zero_or_more: bool,
    },
}
impl<'a> Parse<'a> for Mixed<'a> {}

impl<'a> Mixed<'a> {
    // [51] Mixed ::= '(' S? '#PCDATA' (S? '|' S? Name)* S? ')*' | '(' S? '#PCDATA' S? ')'
    pub fn parse(input: &'a str) -> IResult<&'a str, Mixed<'a>> {
        let (input, _) = tuple((tag("("), Self::parse_multispace0))(input)?;
        let (input, pcdata) = tag("#PCDATA")(input)?;
        let (input, names) = many0(delimited(
            tuple((Self::parse_multispace0, tag("|"), Self::parse_multispace0)),
            Self::parse_name,
            Self::parse_multispace0,
        ))(input)?;
        let (input, _) = tuple((Self::parse_multispace0, tag(")")))(input)?;
        let (input, zero_or_more) = opt(tag("*"))(input)?;
        let mixed = if !pcdata.is_empty() {
            Self::PCDATA {
                names: if names.is_empty() { None } else { Some(names) },
                parsed: true,
                zero_or_more: zero_or_more.is_some(),
            }
        } else {
            Self::PCDATA {
                names: None,
                parsed: false,
                zero_or_more: false,
            }
        };
        Ok((input, mixed))
    }
}

#[derive(Clone, PartialEq)]
pub enum DeclarationContent<'a> {
    Spec {
        mixed: Mixed<'a>,
        children: Option<Vec<ContentParticle<'a>>>,
    },
}

impl<'a> DeclarationContent<'a> {
    pub fn parse_spec(input: &'a str) -> IResult<&'a str, DeclarationContent<'a>> {
        let (input, mixed_content) = Mixed::parse(input)?;
        let (input, children) = opt(Self::parse_children)(input)?;
        Ok((
            input,
            DeclarationContent::Spec {
                mixed: mixed_content,
                children: children.map(|(particles, _)| particles),
            },
        ))
    }
    //  children ::= (choice | seq) ('?' | '*' | '+')?
    fn parse_children(
        input: &'a str,
    ) -> IResult<&'a str, (Vec<ContentParticle<'a>>, Option<&'a str>)> {
        let (input, particles) = many0(ContentParticle::parse_content_particle)(input)?;
        let (input, quantifier) = opt(alt((tag("?"), tag("*"), tag("+"))))(input)?;
        Ok((input, (particles, quantifier)))
    }
}

#[derive(Clone, PartialEq)]
pub enum InternalSubset<'a> {
    // MarkupDecl {
    //     element: Box<InternalSubset<'a>>, // InternalSubset::Element
    // },
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
    fn parse_internal_subset(input: &'a str) -> IResult<&'a str, Vec<InternalSubset<'a>>> {
        many0(alt((
            Self::parse_markup_decl,
            Self::parse_decl_sep,
        )))(input)
    }


    fn parse_decl_sep(input: &'a str) -> IResult<&'a str, InternalSubset<'a>> {
        let (input, _) = tag("%")(input)?;
        // [28a] DeclSep ::=   	PEReference | S
        let (input, name) = Self::parse_name(input)?;
        let (input, _) = tag(";")(input)?;
        
        // [69]   	PEReference	   ::=   	'%' Name ';'
        let (input, _) = Self::parse_multispace0(input)?;
        println!("INPUT AFTER PARSE_DECL_SEP: {input:?}");
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
        Ok((input, InternalSubset::ProcessingInstruction(processing_instruction)))
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


#[derive(Clone, PartialEq)]
pub struct XmlDecl<'a> {
    pub version: Cow<'a, str>,
    pub encoding: Option<Cow<'a, str>>,
    pub standalone: Option<Cow<'a, str>>,
}
impl<'a> Parse<'a> for XmlDecl<'a> {
 // [23] XMLDecl	::=  '<?xml' VersionInfo EncodingDecl? SDDecl? S? '?>'
 fn parse(input: &'a str) -> IResult<&'a str, XmlDecl<'a>> {
    println!("\n\nparsing XMLDecl: {input:?}");
    let (input, _) = tag("<?xml")(input)?;
    let (input, _) = Self::parse_multispace1(input)?;
    println!("before parse_version_info: {input:?}");
    let (input, version) = Self::parse_version_info(input)?;
    println!("Parsed version: {version:?}");
    let (input, encoding) = opt(Self::parse_encoding_decl)(input)?;
    let (input, standalone) = opt(Self::parse_sd_decl)(input)?;
    let (input, _) = Self::parse_multispace0(input)?;
    let (input, _) = tag("?>")(input)?;
    Ok((
        input,
        Self {
            version,
            encoding,
            standalone,
        },
    ))
}
}
impl<'a> XmlDecl<'a> {
    // [24] VersionInfo	::= S 'version' Eq ("'" VersionNum "'" | '"' VersionNum '"')
    fn parse_version_info(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, _) = tag("version")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, version) = alt((
            delimited(tag("'"), Self::parse_version_num, tag("'")),
            delimited(tag("\""), Self::parse_version_num, tag("\"")),
        ))(input)?;
        Ok((input, version))
    }

    // [25] Eq	::= S? '=' S?
    fn parse_eq(input: &'a str) -> IResult<&'a str, &'a str> {
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        Ok((input, "="))
    }

    // [26] VersionNum	::= '1.' [0-9]+
    fn parse_version_num(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, _) = tag("1.")(input)?;
        let (input, version) = digit1(input)?;
        Ok((input, version.into()))
    }
    // [80] EncodingDecl	::= S 'encoding' Eq ('"' EncName '"' | "'" EncName "'" )
    fn parse_encoding_decl(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, _) = tag("encoding")(input)?;
        let (input, _) = Self::parse_eq(input)?;
        let (input, encoding) = alt((
            delimited(tag("'"), Self::parse_enc_name, tag("'")),
            delimited(tag("\""), Self::parse_enc_name, tag("\"")),
        ))(input)?;
        Ok((input, encoding))
    }

    // [32] SDDecl	::= S 'standalone' Eq (("'" ('yes' | 'no') "'") | ('"' ('yes' | 'no') '"'))
    fn parse_sd_decl(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, _) = tag("standalone")(input)?;
        let (input, _) = Self::parse_eq(input)?;
        let (input, standalone) = alt((
            delimited(tag("'"), alt((tag("yes"), tag("no"))), tag("'")),
            delimited(tag("\""), alt((tag("yes"), tag("no"))), tag("\"")),
        ))(input)?;
        Ok((input, standalone.into()))
    }
     // [81] EncName	::= [A-Za-z] ([A-Za-z0-9._] | '-')*
     fn parse_enc_name(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, first) = alt((alpha1, tag("-")))(input)?;
        let (input, rest) = many0(alt((alphanumeric1, tag("."), tag("_"), tag("-"))))(input)?;
        Ok((input, format!("{}{}", first, rest.join("")).into()))
    }

}

#[derive(Clone, PartialEq)]
pub struct DocType<'a> {
    pub name: Cow<'a, str>,
    pub external_id: Option<ExternalID>,
    pub int_subset: Option<Vec<InternalSubset<'a>>>,
} 

impl<'a> Parse<'a> for DocType<'a> {
// [28] doctypedecl	::= '<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>'
fn parse(input: &'a str) -> IResult<&'a str, DocType<'a>> {
    println!("\n\nXXXXXXXXXX\nDOCTYPE?: {input:?}");
    let (input, _) = tag("<!DOCTYPE")(input)?;
    let (input, _) = Self::parse_multispace1(input)?;
    let (input, name) = Self::parse_name(input)?;

    let (input, _) = Self::parse_multispace0(input)?;
    let (input, external_id) = opt(alt((
        map(tag("SYSTEM"), |_| ExternalID::System),
        map(tag("PUBLIC"), |_| ExternalID::Public),
    )))(input)?;

    let (input, _) = Self::parse_multispace0(input)?;

    let (input, _) = tag("[")(input)?;

    let (input, _) = Self::parse_multispace0(input)?;
    println!("\n888888888888\nbefore parse_internal_subset: {input:?}");

    let (input, int_subset) = opt(InternalSubset::parse_internal_subset)(input)?;
    println!("\n888888888888999\nafter parse_internal_subset: {input:?}\n");
    
    let (input, _) = Self::parse_multispace0(input)?;
    println!("int_subset: {int_subset:?}");
    println!("input after int_subset: {input:?}");

    let (input, _) = tag("]")(input)?;
    println!("input2: {input:?}");
    let (input, _) = Self::parse_multispace0(input)?;
    println!("input3: {input:?}");
    let (input, _) = tag(">")(input)?;
    println!("inputx: {input:?}");
    let (input, _) = Self::parse_multispace0(input)?;
    println!("11111111111111111111111111111111111111111111111\n int_subset: {int_subset:?}");
    Ok((
        input,
        Self {
            name,
            external_id,
            int_subset: int_subset,
        },
    ))
}
}


    

