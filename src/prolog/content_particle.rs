use crate::{namespaces::ParseNamespace, parse::Parse, ConditionalState, IResult, Name};
use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, opt},
    multi::{many0, many1},
    sequence::{delimited, tuple},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentParticle {
    Name(Name, ConditionalState),
    Choice(Vec<ContentParticle>, ConditionalState),
    Sequence(Vec<ContentParticle>, ConditionalState),
}
impl<'a> ParseNamespace<'a> for ContentParticle {}
impl<'a> Parse<'a> for ContentParticle {
    type Args = ();
    type Output = IResult<&'a str, Self>;

    // [48] cp ::= (Name | choice | seq) ('?' | '*' | '+')?
    // Namespaces (Third Edition) [18] cp ::= (QName | choice | seq) ('?' | '*' | '+')?
    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
        let (input, res) = alt((
            map(
                tuple((
                    alt((Self::parse_name, Self::parse_qualified_name)),
                    opt(|i| ConditionalState::parse(i, ())),
                )),
                |(name, conditional_state)| {
                    ContentParticle::Name(name, conditional_state.unwrap_or(ConditionalState::None))
                },
            ),
            map(
                tuple((Self::parse_choice, opt(|i| ConditionalState::parse(i, ())))),
                |(choice, conditional_state)| {
                    ContentParticle::Choice(
                        choice,
                        conditional_state.unwrap_or(ConditionalState::None),
                    )
                },
            ),
            map(
                tuple((
                    Self::parse_sequence,
                    opt(|i| ConditionalState::parse(i, ())),
                )),
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
}

impl ContentParticle {
    // [49] choice ::= '(' S? cp ( S? '|' S? cp )+ S? ')'
    pub fn parse_choice(input: &str) -> IResult<&str, Vec<ContentParticle>> {
        map(
            delimited(
                tuple((char('('), Self::parse_multispace0)),
                tuple((
                    |i| Self::parse(i, ()),
                    many1(tuple((
                        tuple((Self::parse_multispace0, char('|'), Self::parse_multispace0)),
                        |i| Self::parse(i, ()),
                    ))),
                )),
                tuple((Self::parse_multispace0, char(')'))),
            ),
            |(first_cp, others)| {
                let mut all_cps = Vec::new();
                all_cps.push(first_cp);
                all_cps.extend(others.into_iter().map(|(_, cp)| cp));
                all_cps
            },
        )(input)
    }

    // [50] seq ::= '(' S? cp ( S? ',' S? cp )* S? ')'
    pub fn parse_sequence(input: &str) -> IResult<&str, Vec<ContentParticle>> {
        map(
            delimited(
                tuple((char('('), Self::parse_multispace0)),
                tuple((
                    |i| Self::parse(i, ()),
                    many0(tuple((
                        tuple((Self::parse_multispace0, char(','), Self::parse_multispace0)),
                        |i| Self::parse(i, ()),
                    ))),
                )),
                tuple((Self::parse_multispace0, char(')'))),
            ),
            |(first_cp, others)| {
                let mut all_cps = Vec::new();
                all_cps.push(first_cp);
                all_cps.extend(others.into_iter().map(|(_, cp)| cp));
                all_cps
            },
        )(input)
    }
}
