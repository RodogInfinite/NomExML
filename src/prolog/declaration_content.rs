use crate::{namespaces::ParseNamespace, parse::Parse, QualifiedName};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

use super::content_particle::ContentParticle;

//TODO: Refactor to better comply with the spec

#[derive(Clone, PartialEq)]
pub enum DeclarationContent<'a> {
    Mixed(Mixed<'a>),
    Children(ContentParticle<'a>),
    Empty,
    Any,
}

impl<'a> Parse<'a> for DeclarationContent<'a> {
    // [46] contentspec ::= 'EMPTY' | 'ANY' | Mixed | children
    fn parse(input: &'a str) -> IResult<&'a str, DeclarationContent<'a>> {
        println!("PARSING DECLARATION CONTENT INPUT: {input}");
        alt((
            map(tag("EMPTY"), |_| Self::Empty),
            map(tag("ANY"), |_| Self::Any),
            map(Mixed::parse, Self::Mixed),
            map(Self::parse_children, Self::Children),
        ))(input)
    }
}
impl<'a> DeclarationContent<'a> {
    // [47] children ::= (choice | seq) ('?' | '*' | '+')?
    fn parse_children(input: &'a str) -> IResult<&'a str, ContentParticle<'a>> {
        let (input, particle) = ContentParticle::parse(input)?;
        Ok((input, particle))
    }
}

#[derive(Clone, PartialEq)]
pub enum Mixed<'a> {
    PCDATA {
        names: Option<Vec<QualifiedName<'a>>>,
        parsed: bool,
        zero_or_more: bool,
    },
}
impl<'a> Parse<'a> for Mixed<'a> {
    // [51] Mixed ::= '(' S? '#PCDATA' (S? '|' S? Name)* S? ')*' | '(' S? '#PCDATA' S? ')'
    fn parse(input: &'a str) -> IResult<&'a str, Mixed<'a>> {
        let (input, _) = tuple((tag("("), Self::parse_multispace0))(input)?;
        let (input, pcdata) = tag("#PCDATA")(input)?;
        let (input, names) = many0(delimited(
            tuple((Self::parse_multispace0, tag("|"), Self::parse_multispace0)),
            Self::parse_name,
            Self::parse_multispace0,
        ))(input)?;
        let names = if !names.is_empty() {
            Some(names.into_iter().collect())
        } else {
            None
        };
        let (input, _) = tuple((Self::parse_multispace0, tag(")")))(input)?;
        let (input, zero_or_more) = opt(tag("*"))(input)?;
        let mixed = if !pcdata.is_empty() {
            Self::PCDATA {
                names,
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

impl<'a> ParseNamespace<'a> for Mixed<'a> {}

impl<'a> Mixed<'a> {
    // Namespaces (Third Edition) [19] Mixed ::= '(' S? '#PCDATA' (S? '|' S? QName)* S? ')*' | '(' S? '#PCDATA' S? ')'
    pub fn parse_qualified(input: &'a str) -> IResult<&'a str, Mixed<'a>> {
        let (input, _) = tuple((tag("("), Self::parse_multispace0))(input)?;
        let (input, pcdata) = tag("#PCDATA")(input)?;
        let (input, names) = many0(delimited(
            tuple((Self::parse_multispace0, tag("|"), Self::parse_multispace0)),
            Self::parse_qualified_name,
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
