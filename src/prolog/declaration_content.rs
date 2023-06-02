use crate::{
    namespaces::{ParseNamespace, QualifiedName},
    parse::Parse,
    tag::ConditionalState,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{opt, value},
    multi::{many0, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};

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
    // [47] children ::= (choice | seq) ('?' | '*' | '+')?
    fn parse_children(
        input: &'a str,
    ) -> IResult<&'a str, (Vec<ContentParticle<'a>>, Option<&'a str>)> {
        let (input, particles) = many0(ContentParticle::parse)(input)?;
        let (input, quantifier) = opt(alt((tag("?"), tag("*"), tag("+"))))(input)?;
        Ok((input, (particles, quantifier)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContentParticle<'a> {
    Particle {
        names: Option<Vec<QualifiedName<'a>>>,
        choice: Option<Vec<ContentParticle<'a>>>,
        sequence: Option<Vec<ContentParticle<'a>>>,
        conditional_state: Option<ConditionalState>,
    },
}

impl<'a> Parse<'a> for ContentParticle<'a> {}
impl<'a> ParseNamespace<'a> for ContentParticle<'a> {}

impl<'a> ContentParticle<'a> {
    // [48] cp ::= (Name | choice | seq) ('?' | '*' | '+')?
    fn parse(input: &'a str) -> IResult<&'a str, ContentParticle<'a>> {
        let (input, names) = opt(many0(Self::parse_name))(input)?;
        let names = names.map(|names| {
            names
                .into_iter()
                .map(|name| QualifiedName {
                    prefix: None,
                    local_part: name,
                })
                .collect()
        });

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

    // Namespaces (Third Edition) [18] cp ::= (QName | choice | seq) ('?' | '*' | '+')?
    fn parse_qualified_content_particle(input: &'a str) -> IResult<&'a str, ContentParticle<'a>> {
        let (input, names) = opt(many0(Self::parse_qualified_name))(input)?;
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

    // [49] choice ::= '(' S? cp ( S? '|' S? cp )+ S? ')'
    fn parse_choice(input: &'a str) -> IResult<&'a str, Vec<ContentParticle<'a>>> {
        let inner = separated_list1(
            tuple((Self::parse_multispace0, tag("|"), Self::parse_multispace0)),
            Self::parse,
        );
        let mut parser = delimited(
            tuple((tag("("), Self::parse_multispace0)),
            inner,
            tuple((Self::parse_multispace0, tag(")"))),
        );
        let (input, choice) = parser(input)?;
        Ok((input, choice))
    }

    // [50] seq ::= '(' S? cp ( S? ',' S? cp )* S? ')'
    fn parse_seq(input: &'a str) -> IResult<&'a str, Vec<ContentParticle<'a>>> {
        let inner = separated_list1(
            tuple((Self::parse_multispace0, tag(","), Self::parse_multispace0)),
            Self::parse,
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
            Some(
                names
                    .into_iter()
                    .map(|name| QualifiedName {
                        prefix: None,
                        local_part: name,
                    })
                    .collect(),
            )
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
