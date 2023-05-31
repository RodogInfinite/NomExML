pub mod content_particle;
pub mod doctype;
pub mod external_id;
pub mod internal_subset;
pub mod xmldecl;

use std::borrow::Cow;

use crate::{parse::Parse, tag::ConditionalState};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{opt, value},
    multi::{many0, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};

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
