use std::borrow::Cow;

use crate::{ConditionalState, Document};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::{
        complete::{alpha1, space0},
        is_alphanumeric,
    },
    combinator::{map, opt, recognize, value},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ExternalID {
    Public,
    System,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContentParticle<'a> {
    Particle {
        names: Option<Vec<Cow<'a, str>>>,
        choice: Option<Vec<ContentParticle<'a>>>, // Vec(Particles)
        sequence: Option<Vec<ContentParticle<'a>>>, // Vec(Particles)
        conditional_state: Option<ConditionalState>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mixed<'a> {
    PCDATA {
        names: Option<Vec<Cow<'a, str>>>,
        parsed: bool,
        conditional_state: ConditionalState,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum DeclarationContent<'a> {
    Spec {
        mixed: Mixed<'a>,
        children: Option<Vec<ContentParticle<'a>>>, // Vec(Particles)
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Declaration<'a> {
    DocType {
        name: Option<Cow<'a, str>>,
        external_id: Option<ExternalID>,
        int_subset: Option<Vec<Declaration<'a>>>, // Some(Vec<Box<Declaration::Element>>)
    },
    Element {
        name: Option<Cow<'a, str>>,
        content_spec: Option<DeclarationContent<'a>>,
    },
}

impl<'a> Document<'a> {
    fn parse_declaration(input: &'a str) -> IResult<&'a str, Declaration<'a>> {
        let (input, _) = tag("<!")(input)?;
        let (input, decl) = opt(alpha1)(input)?;
        println!("Decl?{decl:?} {input:?}");
        let (input, _) = space0(input)?;

        match decl {
            Some("DOCTYPE") => {
                let (input, name) = opt(alpha1)(input)?;
                let (input, _) = space0(input)?;
                let (input, external_id) = opt(alt((
                    map(tag("SYSTEM"), |_| ExternalID::System),
                    map(tag("PUBLIC"), |_| ExternalID::Public),
                )))(input)?;
                let (input, _) = space0(input)?;
                let (input, _) = tag("[")(input)?;
                let (input, _) = space0(input)?;
                let (input, int_subset) = opt(many0(Self::parse_declaration))(input)?;
                println!("HERE INPUT: {input:?}"); // prints: "<!ELEMENT doc (#PCDATA|TEST) >]><doc></doc>\n        "
                let (input, _) = tag("]")(input)?;
                println!("Never Reaches here"); // never reaches here
                let (input, _) = tag(">")(input)?;
                if int_subset.is_some() {
                    Ok((
                        input,
                        Declaration::DocType {
                            name: name.map(|s| s.into()),
                            external_id,
                            int_subset: int_subset,
                        },
                    ))
                } else {
                    Ok((
                        input,
                        Declaration::DocType {
                            name: name.map(|s| s.into()),
                            external_id,
                            int_subset: None,
                        },
                    ))
                }
            }
            Some("ELEMENT") => {
                let (input, element_name) = opt(alpha1)(input)?;
                println!("ELEMENT?{element_name:?} {input:?}");
                let (input, _) = space0(input)?;

                let (input, content_spec) = Self::parse_spec(input)?;
                let (input, _) = tag(">")(input)?;

                Ok((
                    input,
                    Declaration::Element {
                        name: element_name.map(|s| s.into()),
                        content_spec: Some(content_spec),
                    },
                ))
            }
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            ))),
        }
    }

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

    fn parse_conditional_state(input: &'a str) -> IResult<&'a str, ConditionalState> {
        alt((
            value(ConditionalState::Optional, tag("?")),
            value(ConditionalState::ZeroOrMore, tag("*")),
            value(ConditionalState::OneOrMore, tag("+")),
        ))(input)
    }

    fn is_name_char(c: char) -> bool {
        is_alphanumeric(c as u8) || c == '_'
    }

    fn parse_name(input: &'a str) -> IResult<&'a str, Cow<str>> {
        let (input, name) =
            map(take_while1(Self::is_name_char), |s: &str| Cow::Borrowed(s))(input)?;
        Ok((input, name))
    }

    // choice ::= '(' S? cp ( S? '|' S? cp )+ S? ')'
    fn parse_choice(input: &'a str) -> IResult<&'a str, Vec<ContentParticle<'a>>> {
        let inner = separated_list1(
            tuple((space0, tag("|"), space0)),
            Self::parse_content_particle,
        );
        let mut parser = delimited(tuple((tag("("), space0)), inner, tuple((space0, tag(")"))));
        let (input, choice) = parser(input)?;
        Ok((input, choice))
    }

    // seq ::= '(' S? cp ( S? ',' S? cp )* S? ')'
    fn parse_seq(input: &'a str) -> IResult<&'a str, Vec<ContentParticle<'a>>> {
        let inner = separated_list1(
            tuple((space0, tag(","), space0)),
            Self::parse_content_particle,
        );
        let mut parser = delimited(tuple((tag("("), space0)), inner, tuple((space0, tag(")"))));
        let (input, sequence) = parser(input)?;
        Ok((input, sequence))
    }

    //  children ::= (choice | seq) ('?' | '*' | '+')?
    fn parse_children(
        input: &'a str,
    ) -> IResult<&'a str, (Vec<ContentParticle<'a>>, Option<&'a str>)> {
        let (input, particles) = many0(Self::parse_content_particle)(input)?;
        let (input, quantifier) = opt(alt((tag("?"), tag("*"), tag("+"))))(input)?;
        Ok((input, (particles, quantifier)))
    }

    // Mixed ::= '(' S? '#PCDATA' (S? '|' S? Name)* S? ')*' | '(' S? '#PCDATA' S? ')'
    pub fn parse_mixed(input: &'a str) -> IResult<&'a str, Mixed<'a>> {
        let (input, _) = tuple((tag("("), space0))(input)?;
        let (input, pcdata) = opt(tag("#PCDATA"))(input)?;
        let (input, names) = many0(delimited(
            tuple((space0, tag("|"), space0)),
            Self::parse_name,
            space0,
        ))(input)?;
        let (input, condition) = tuple((space0, tag(")")))(input)?;
        let (input, quantifier) = opt(tag("*"))(input)?;

        let conditional_state = if quantifier.is_some() {
            ConditionalState::ZeroOrMore
        } else {
            ConditionalState::None
        };

        let parsed = pcdata.is_some();

        let mixed = if parsed {
            Mixed::PCDATA {
                names: if names.is_empty() { None } else { Some(names) },
                parsed,
                conditional_state,
            }
        } else {
            Mixed::PCDATA {
                names: None,
                parsed: false,
                conditional_state: ConditionalState::None,
            }
        };

        Ok((input, mixed))
    }

    pub fn parse_spec(input: &'a str) -> IResult<&'a str, DeclarationContent<'a>> {
        let (input, mixed_content) = Self::parse_mixed(input)?;
        println!("MC: {mixed_content:?}");
        let (input, children) = opt(Self::parse_children)(input)?;
        Ok((
            input,
            DeclarationContent::Spec {
                mixed: mixed_content,
                children: children.map(|(particles, _)| particles),
            },
        ))
    }
}
