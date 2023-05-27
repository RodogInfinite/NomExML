use std::borrow::Cow;

use crate::utils::Parse;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::space0,
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Clone, PartialEq)]
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
    pub fn parse_definition(input: &'a str) -> IResult<&'a str, Attribute<'a>> {
        let (input, name) = Self::parse_name(input)?;
        let (input, att_type) = AttType::parse(input)?;
        let (input, default_decl) = DefaultDecl::parse(input)?;
        let attribute = Attribute::Definition {
            name: Cow::Owned(name.into()),
            att_type,
            default_decl: default_decl,
        };
        Ok((input, attribute))
    }

    pub fn parse_attribute_instance(input: &'a str) -> IResult<&'a str, Attribute<'a>> {
        let valid_chars = ['_', '-', ':', '.'];
        let (input, name) =
            take_while1(|c: char| c.is_alphanumeric() || valid_chars.contains(&c))(input)?;
        let (input, _) = Self::parse_with_whitespace(input, tag("="))?;
        let (input, value) = Self::parse_with_whitespace(input, Self::parse_literal)?;
        Ok((
            input,
            Attribute::Instance {
                name: Cow::Borrowed(name),
                value: Cow::Borrowed(value),
            },
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum AttType<'a> {
    CDATA,
    Tokenized(TokenizedType),
    Enumerated {
        notation: Option<Vec<Cow<'a, str>>>,
        enumeration: Option<Vec<Cow<'a, str>>>,
    },
}

impl<'a> AttType<'a> {
    fn parse_enumerated_type(input: &'a str) -> IResult<&'a str, AttType<'a>> {
        let mut parser = delimited(
            tag("("),
            separated_list1(tuple((space0, tag("|"), space0)), Self::parse_name),
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
impl<'a> Parse<'a> for AttType<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (input, att_type) = Self::parse_with_whitespace(
            input,
            alt((
                value(AttType::CDATA, tag("CDATA")),
                map(TokenizedType::parse, AttType::Tokenized),
                Self::parse_enumerated_type,
            )),
        )?;
        Ok((input, att_type))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DefaultDecl<'a> {
    Required,
    Implied,
    Fixed(Cow<'a, str>),
    Value(Cow<'a, str>),
}

impl<'a> Parse<'a> for DefaultDecl<'a> {
    // https://www.w3.org/TR/2008/REC-xml-20081126/#NT-DefaultDecl
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        alt((
            value(DefaultDecl::Required, tag("#REQUIRED")),
            value(DefaultDecl::Implied, tag("#IMPLIED")),
            map(
                tuple((tag("#FIXED"), space0, Self::parse_literal)),
                |(_, _, lit)| DefaultDecl::Fixed(Cow::Borrowed(lit)),
            ),
            map(Self::parse_literal, |lit| {
                DefaultDecl::Value(Cow::Borrowed(lit))
            }),
        ))(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CharRef<'a> {
    Decimal(Cow<'a, str>),
    Hexadecimal(Cow<'a, str>),
}
