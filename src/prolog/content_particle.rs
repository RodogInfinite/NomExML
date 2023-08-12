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
impl<'a> ParseNamespace<'a> for ContentParticle<'a> {}
impl<'a> Parse<'a> for ContentParticle<'a> {
    type Args = ();
    type Output = IResult<&'a str, Self>;

    // [48] cp ::= (Name | choice | seq) ('?' | '*' | '+')?
    // Namespaces (Third Edition) [18] cp ::= (QName | choice | seq) ('?' | '*' | '+')?
    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
        let (input, res) = alt((
            map(
                tuple((
                    opt(char('(')),
                    alt((Self::parse_name, Self::parse_qualified_name)),
                    opt(|i| ConditionalState::parse(i, ())),
                    opt(char(')')),
                    opt(|i| ConditionalState::parse(i, ())), //TODO: verify that this should be here
                )),
                |(
                    _open_bracket,
                    name,
                    conditional_state,
                    _close_bracket,
                    conditional_state_outter,
                )| match (conditional_state, conditional_state_outter) {
                    (Some(conditional_state), None) => {
                        ContentParticle::Name(name, conditional_state)
                    }
                    (None, Some(conditional_state_outter)) => {
                        ContentParticle::Name(name, conditional_state_outter)
                    }
                    (None, None) => ContentParticle::Name(name, ConditionalState::None),
                    _ => panic!("parsing of ContentParticle failed"),
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
                tuple((Self::parse_seq, opt(|i| ConditionalState::parse(i, ())))),
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

impl<'a> ContentParticle<'a> {
    // [49] choice ::= '(' S? cp ( S? '|' S? cp )+ S? ')'
    fn parse_choice(input: &'a str) -> IResult<&'a str, Vec<ContentParticle<'a>>> {
        let inner = separated_list1(
            tuple((Self::parse_multispace0, char('|'), Self::parse_multispace0)),
            |i| Self::parse(i, ()),
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
            |i| Self::parse(i, ()),
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
