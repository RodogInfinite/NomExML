use crate::{namespaces::ParseNamespace, parse::Parse, ConditionalState, QualifiedName};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::{many0, many1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use super::content_particle::ContentParticle;

#[derive(Clone, PartialEq)]
pub enum DeclarationContent {
    Mixed(Mixed),
    Children(ContentParticle),
    Empty,
    Any,
}

impl<'a> Parse<'a> for DeclarationContent {
    type Args = ();
    type Output = IResult<&'a str, Self>;
    // [46] contentspec ::= 'EMPTY' | 'ANY' | Mixed | children
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        alt((
            map(tag("EMPTY"), |_| Self::Empty),
            map(tag("ANY"), |_| Self::Any),
            map(|i| Mixed::parse(i, args), Self::Mixed),
            map(Self::parse_children, Self::Children),
        ))(input)
    }
}
impl<'a> ParseNamespace<'a> for DeclarationContent {}
impl DeclarationContent {
    // [47] children ::= (choice | seq) ('?' | '*' | '+')?
    fn parse_children(input: &str) -> IResult<&str, ContentParticle> {
        let (input, particle) = alt((
            map(
                tuple((
                    ContentParticle::parse_choice,
                    opt(|i| ConditionalState::parse(i, ())),
                )),
                |(choice, state)| {
                    ContentParticle::Choice(choice, state.unwrap_or(ConditionalState::None))
                },
            ),
            map(
                tuple((
                    ContentParticle::parse_sequence,
                    opt(|i| ConditionalState::parse(i, ())),
                )),
                |(seq, state)| {
                    ContentParticle::Sequence(seq, state.unwrap_or(ConditionalState::None))
                },
            ),
        ))(input)?;
        Ok((input, particle))
    }
}

#[derive(Clone, PartialEq)]
pub enum Mixed {
    PCDATA,
    Names(Vec<QualifiedName>),
}

impl<'a> ParseNamespace<'a> for Mixed {}
impl<'a> Parse<'a> for Mixed {
    type Args = ();
    type Output = IResult<&'a str, Self>;
    // [51] Mixed ::= '(' S? '#PCDATA' (S? '|' S? Name)* S? ')*' | '(' S? '#PCDATA' S? ')'
    // Namespaces (Third Edition) [19] Mixed ::= '(' S? '#PCDATA' (S? '|' S? QName)* S? ')*' | '(' S? '#PCDATA' S? ')'

    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
        alt((
            map(
                tuple((
                    tag("("),
                    Self::parse_multispace0,
                    tag("#PCDATA"),
                    many1(preceded(
                        tuple((Self::parse_multispace0, tag("|"), Self::parse_multispace0)),
                        alt((Self::parse_name, Self::parse_qualified_name)),
                    )),
                    Self::parse_multispace0,
                    tag(")*"),
                )),
                |(_, _, _, names, _, _)| Mixed::Names(names),
            ),
            map(
                tuple((
                    tag("("),
                    Self::parse_multispace0,
                    tag("#PCDATA"),
                    Self::parse_multispace0,
                    tag(")"),
                )),
                |_| Mixed::PCDATA,
            ),
        ))(input)
    }
}
