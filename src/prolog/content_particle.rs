use crate::{namespaces::ParseNamespace, parse::Parse, ConditionalState, QualifiedName};
use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ContentParticle<'a> {
    Name(QualifiedName<'a>, ConditionalState),
    Choice(Vec<ContentParticle<'a>>, ConditionalState),
    Sequence(Vec<ContentParticle<'a>>, ConditionalState),
}

impl<'a> Parse<'a> for ContentParticle<'a> {}
impl<'a> ParseNamespace<'a> for ContentParticle<'a> {}

impl<'a> ContentParticle<'a> {
    // [48] cp ::= (Name | choice | seq) ('?' | '*' | '+')?
    // Namespaces (Third Edition) [18] cp ::= (QName | choice | seq) ('?' | '*' | '+')?
    pub fn parse(input: &'a str) -> IResult<&'a str, ContentParticle<'a>> {
        let (input, res) = alt((
            map(
                tuple((
                    opt(char('(')),
                    alt((Self::parse_name, Self::parse_qualified_name)),
                    opt(ConditionalState::parse),
                    opt(char(')')),
                )),
                |(_, name, conditional_state, _)| {
                    ContentParticle::Name(name, conditional_state.unwrap_or(ConditionalState::None))
                },
            ),
            map(
                tuple((Self::parse_choice, opt(ConditionalState::parse))),
                |(choice, conditional_state)| {
                    ContentParticle::Choice(
                        choice,
                        conditional_state.unwrap_or(ConditionalState::None),
                    )
                },
            ),
            map(
                tuple((Self::parse_seq, opt(ConditionalState::parse))),
                |(sequence, conditional_state)| {
                    ContentParticle::Sequence(
                        sequence,
                        conditional_state.unwrap_or(ConditionalState::None),
                    )
                },
            ),
        ))(input)?;

        Ok((input, res))
    }

    // [49] choice ::= '(' S? cp ( S? '|' S? cp )+ S? ')'
    fn parse_choice(input: &'a str) -> IResult<&'a str, Vec<ContentParticle<'a>>> {
        let inner = separated_list1(
            tuple((Self::parse_multispace0, char('|'), Self::parse_multispace0)),
            Self::parse,
        );
        let mut parser = delimited(
            tuple((char('('), Self::parse_multispace0)),
            inner,
            tuple((Self::parse_multispace0, char(')'))),
        );
        let (input, choice) = parser(input)?;
        Ok((input, choice))
    }

    // [50] seq ::= '(' S? cp ( S? ',' S? cp )* S? ')'
    fn parse_seq(input: &'a str) -> IResult<&'a str, Vec<ContentParticle<'a>>> {
        let inner = separated_list1(
            tuple((Self::parse_multispace0, char(','), Self::parse_multispace0)),
            Self::parse,
        );
        let mut parser = delimited(
            tuple((char('('), Self::parse_multispace0)),
            inner,
            tuple((Self::parse_multispace0, char(')'))),
        );
        let (input, sequence) = parser(input)?;
        Ok((input, sequence))
    }
}
