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

#[derive(Clone, PartialEq)]
pub enum DeclarationContent<'a> {
    Mixed(Mixed<'a>),
    Children(ContentParticle<'a>),
    Empty,
    Any,
}

impl<'a> Parse<'a> for DeclarationContent<'a> {
    type Args = ();
    type Output = IResult<&'a str, Self>;
    // [46] contentspec ::= 'EMPTY' | 'ANY' | Mixed | children
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        dbg!(&input, "DeclarationContent::parse input");
        alt((
            map(tag("EMPTY"), |_| Self::Empty),
            map(tag("ANY"), |_| Self::Any),
            map(|i| Mixed::parse(i, args), Self::Mixed),
            map(Self::parse_children, Self::Children),
        ))(input)
    }
}
impl<'a> DeclarationContent<'a> {
    // [47] children ::= (choice | seq) ('?' | '*' | '+')?
    fn parse_children(input: &'a str) -> IResult<&'a str, ContentParticle<'a>> {
        let (input, particle) = ContentParticle::parse(input, ())?;
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
    type Args = ();
    type Output = IResult<&'a str, Self>;
    // [51] Mixed ::= '(' S? '#PCDATA' (S? '|' S? Name)* S? ')*' | '(' S? '#PCDATA' S? ')'
    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
        dbg!(&input, "Mixed::parse input");
        map(
            tuple((
                tag("("),
                Self::parse_multispace0,
                tag("#PCDATA"),
                many0(delimited(
                    tuple((Self::parse_multispace0, tag("|"), Self::parse_multispace0)),
                    Self::parse_name,
                    Self::parse_multispace0,
                )),
                Self::parse_multispace0,
                tag(")"),
                opt(tag("*")),
            )),
            |(
                _open_bracket,
                _whitespace1,
                pcdata,
                names,
                _whitespace2,
                _close_bracket,
                zero_or_more,
            )| {
                if !pcdata.is_empty() {
                    Self::PCDATA {
                        names: if !names.is_empty() {
                            Some(names.into_iter().collect())
                        } else {
                            None
                        },
                        parsed: true,
                        zero_or_more: zero_or_more.is_some(),
                    }
                } else {
                    Self::PCDATA {
                        names: None,
                        parsed: false,
                        zero_or_more: false,
                    }
                }
            },
        )(input)
    }
}

impl<'a> ParseNamespace<'a> for Mixed<'a> {}

impl<'a> Mixed<'a> {
    // Namespaces (Third Edition) [19] Mixed ::= '(' S? '#PCDATA' (S? '|' S? QName)* S? ')*' | '(' S? '#PCDATA' S? ')'
    pub fn parse_qualified(input: &'a str) -> IResult<&'a str, Mixed<'a>> {
        map(
            tuple((
                tag("("),
                Self::parse_multispace0,
                tag("#PCDATA"),
                many0(delimited(
                    tuple((Self::parse_multispace0, tag("|"), Self::parse_multispace0)),
                    Self::parse_qualified_name,
                    Self::parse_multispace0,
                )),
                Self::parse_multispace0,
                tag(")"),
                opt(tag("*")),
            )),
            |(
                _open_bracket,
                _whitespace1,
                pcdata,
                names,
                _whitespace2,
                _close_bracket,
                zero_or_more,
            )| {
                if !pcdata.is_empty() {
                    Mixed::PCDATA {
                        names: if names.is_empty() { None } else { Some(names) },
                        parsed: true,
                        zero_or_more: zero_or_more.is_some(),
                    }
                } else {
                    Mixed::PCDATA {
                        names: None,
                        parsed: false,
                        zero_or_more: false,
                    }
                }
            },
        )(input)
    }
}
