use std::borrow::Cow;

use crate::{declaration::ContentParticle, Document};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
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
        default_decl: DefaultDecl<'a>, // Attribute::DefaultDecl<Attribute::Value> || Attribute::Reference
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

impl<'a> Attribute<'a> {
    fn parse_literal(input: &'a str) -> IResult<&'a str, &'a str> {
        delimited(
            alt((tag("'"), tag("\""))),
            take_while(|c: char| c != '\'' && c != '\"' && c != '<' && c != '&'),
            alt((tag("'"), tag("\""))),
        )(input)
    }

    pub fn parse_definition(input: &'a str) -> IResult<&'a str, Attribute<'a>> {
        let (input, name) = ContentParticle::parse_name(input)?;
        let (input, att_type) = Document::parse_with_whitespace(input,AttType::parse_att_type)?;
        let (input, default_decl) = DefaultDecl::parse(input)?;
        let attribute = Attribute::Definition {
            name: Cow::Owned(name.to_string()), // Change this line
            att_type,
            default_decl: default_decl,
        };
        Ok((input, attribute))
    }

    pub fn parse_attribute_instance(input: &'a str) -> IResult<&'a str, Attribute<'a>> {
        let (input, name) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;
        let (input, _) = Document::parse_with_whitespace(input, tag("="))?;
        let (input, value) = Document::parse_with_whitespace(input,Self::parse_literal)?;
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
            separated_list1(
                tuple((space0, tag("|"), space0)),
                ContentParticle::parse_name,
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

    fn parse_att_type(input: &'a str) -> IResult<&'a str, AttType<'a>> {
        let (input, att_type) = alt((
            value(AttType::CDATA, tag("CDATA")),
            map(TokenizedType::parse, AttType::Tokenized),
            Self::parse_enumerated_type,
        ))(input)?;
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

impl<'a> DefaultDecl<'a> {
    // https://www.w3.org/TR/2008/REC-xml-20081126/#NT-DefaultDecl
    fn parse(input: &'a str) -> IResult<&'a str, DefaultDecl<'a>> {
        alt((
            value(DefaultDecl::Required, tag("#REQUIRED")),
            value(DefaultDecl::Implied, tag("#IMPLIED")),
            map(
                tuple((tag("#FIXED"), space0, Attribute::parse_literal)),
                |(_, _, lit)| DefaultDecl::Fixed(Cow::Borrowed(lit)),
            ),
            map(Attribute::parse_literal, |lit| {
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
