use std::borrow::Cow;

use crate::utils::Parse;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::space0,
    combinator::{map, value},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};
use serde::Serialize;

#[derive(Clone, PartialEq,Serialize)]
pub enum Attribute<'a> {
    Definition {
        name: Cow<'a, str>,
        att_type: AttType<'a>,
        default_decl: DefaultDecl<'a>,
    },
    Reference {
        entity: Cow<'a, str>,
        char: CharRef<'a>,
    },
    Instance {
        name: Cow<'a, str>,
        value: Cow<'a, str>,
    },
    Required,
    Implied,
}
impl<'a> Parse<'a> for Attribute<'a> {}
impl<'a> Attribute<'a> {
    // [53] AttDef ::= S Name S AttType S DefaultDecl
    pub fn parse_definition(input: &'a str) -> IResult<&'a str, Attribute<'a>> {
        let (input, _) = Self::parse_multispace1(input)?;
        println!("Parsed whitespace. input: {input:?}");
        let (input, name) = Self::parse_name(input)?;
        println!("Parsed name: {:?}", name);
        let (input, _) = Self::parse_multispace1(input)?;
        println!("Parsed whitespace 2");
        // [54] AttType ::= StringType | TokenizedType | EnumeratedType
        let (input, att_type) = AttType::parse(input)?;
        println!("Parsed attribute type: {:?}", att_type);
        let (input, _) = Self::parse_multispace1(input)?;
        println!("Parsed whitespace 3");
        let (input, default_decl) = DefaultDecl::parse(input)?;
        println!("Parsed default declaration: {:?}", default_decl);
        let attribute = Attribute::Definition {
            name: Cow::Owned(name.into()),
            att_type,
            default_decl: default_decl,
        };
        Ok((input, attribute))
    }

    //TODO: make this conform to the standard more closely
    pub fn parse_attribute_instance(input: &'a str) -> IResult<&'a str, Attribute<'a>> {
        let valid_chars = ['_', '-', ':', '.'];
        let (input, name) =
            take_while1(|c: char| c.is_alphanumeric() || valid_chars.contains(&c))(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, value) = Self::parse_literal(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        Ok((
            input,
            Attribute::Instance {
                name: Cow::Owned(name.into()),
                value,
            },
        ))
    }
}

#[derive(Clone, Debug, PartialEq,Serialize)]
pub enum TokenizedType {
    ID,
    IDREF,
    IDREFS,
    ENTITY,
    ENTITIES,
    NMTOKEN,
    NMTOKENS,
}

impl TokenizedType {
    // https://www.w3.org/TR/2008/REC-xml-20081126/#NT-TokenizedType
    fn parse(input: &str) -> IResult<&str, TokenizedType> {
        alt((
            value(TokenizedType::ID, tag("ID")),
            value(TokenizedType::IDREF, tag("IDREF")),
            value(TokenizedType::IDREFS, tag("IDREFS")),
            value(TokenizedType::ENTITY, tag("ENTITY")),
            value(TokenizedType::ENTITIES, tag("ENTITIES")),
            value(TokenizedType::NMTOKEN, tag("NMTOKEN")),
            value(TokenizedType::NMTOKENS, tag("NMTOKENS")),
        ))(input)
    }
}

#[derive(Clone, Debug, PartialEq,Serialize)]
pub enum AttType<'a> {
    CDATA,
    Tokenized(TokenizedType),
    Enumerated {
        notation: Option<Vec<Cow<'a, str>>>,
        enumeration: Option<Vec<Cow<'a, str>>>,
    },
}

impl<'a> Parse<'a> for AttType<'a> {
    //[54] AttType ::=  StringType | TokenizedType | EnumeratedType
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (input, att_type) = alt((
            // [55] StringType ::= 'CDATA'
            value(AttType::CDATA, tag("CDATA")),
            // [56] TokenizedType ::= 'ID'| 'IDREF' | 'IDREFS' | 'ENTITY' | 'ENTITIES' | 'NMTOKEN' | 'NMTOKENS'
            map(TokenizedType::parse, AttType::Tokenized),
            Self::parse_enumerated_type,
        ))(input)?;

        Ok((input, att_type))
    }
}
impl<'a> AttType<'a> {
    // [57] EnumeratedType ::= NotationType | Enumeration
    fn parse_enumerated_type(input: &'a str) -> IResult<&'a str, AttType<'a>> {
        alt((Self::parse_notation_type, Self::parse_enumeration))(input)
    }

    // [58] NotationType ::= 'NOTATION' S '(' S? Name (S? '|' S? Name)* S? ')'
    fn parse_notation_type(input: &'a str) -> IResult<&'a str, AttType<'a>> {
        let (input, _) = tag("NOTATION")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let mut parser = delimited(
            tag("("),
            separated_list0(
                tuple((Self::parse_multispace0, tag("|"), Self::parse_multispace0)),
                Self::parse_name,
            ),
            tag(")"),
        );
        let (input, notation) = parser(input)?;
        Ok((
            input,
            AttType::Enumerated {
                notation: Some(notation),
                enumeration: None,
            },
        ))
    }

    // [59] Enumeration ::= '(' S? Nmtoken (S? '|' S? Nmtoken)* S? ')'
    fn parse_enumeration(input: &'a str) -> IResult<&'a str, AttType<'a>> {
        let mut parser = delimited(
            tag("("),
            separated_list1(
                tuple((Self::parse_multispace0, tag("|"), Self::parse_multispace0)),
                Self::parse_nmtoken,
            ),
            tag(")"),
        );
        let (input, enumeration) = parser(input)?;
        Ok((
            input,
            AttType::Enumerated {
                notation: None,
                enumeration: Some(enumeration),
            },
        ))
    }
}

#[derive(Clone, Debug, PartialEq,Serialize)]
pub enum DefaultDecl<'a> {
    Required,
    Implied,
    Fixed(Cow<'a, str>),
    Value(Cow<'a, str>),
}

impl<'a> Parse<'a> for DefaultDecl<'a> {
    // https://www.w3.org/TR/2008/REC-xml-20081126/#NT-DefaultDecl
    // [60] DefaultDecl	::= '#REQUIRED' | '#IMPLIED' | (('#FIXED' S)? AttValue)
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        alt((
            value(DefaultDecl::Required, tag("#REQUIRED")),
            value(DefaultDecl::Implied, tag("#IMPLIED")),
            map(
                tuple((tag("#FIXED"), Self::parse_multispace1, Self::parse_literal)),
                |(_, _, lit)| DefaultDecl::Fixed(lit),
            ),
            map(Self::parse_literal, |lit| DefaultDecl::Value(lit)),
        ))(input)
    }
}

#[derive(Clone, Debug, PartialEq,Serialize)]
pub enum CharRef<'a> {
    Decimal(Cow<'a, str>),
    Hexadecimal(Cow<'a, str>),
}
