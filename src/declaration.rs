use std::borrow::Cow;

use crate::{ConditionalState, Document};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::{
        complete::{alpha1, multispace0, space0},
        is_alphanumeric,
    },
    combinator::{map, opt, value},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, tuple},
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
        choice: Option<Vec<ContentParticle<'a>>>,
        sequence: Option<Vec<ContentParticle<'a>>>,
        conditional_state: Option<ConditionalState>,
    },
}

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
        conditional_state: ConditionalState,
    },
}

impl<'a> Mixed<'a> {
    // Mixed ::= '(' S? '#PCDATA' (S? '|' S? Name)* S? ')*' | '(' S? '#PCDATA' S? ')'
    pub fn parse(input: &'a str) -> IResult<&'a str, Mixed<'a>> {
        let (input, _) = tuple((tag("("), space0))(input)?;
        let (input, pcdata) = opt(tag("#PCDATA"))(input)?;
        let (input, names) = many0(delimited(
            tuple((space0, tag("|"), space0)),
            ContentParticle::parse_name,
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
            Self::PCDATA {
                names: if names.is_empty() { None } else { Some(names) },
                parsed,
                conditional_state,
            }
        } else {
            Self::PCDATA {
                names: None,
                parsed: false,
                conditional_state: ConditionalState::None,
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
    Required,
    Implied,
    Instance {
        name: Cow<'a, str>,
        value: Cow<'a, str>,
    },
}

impl<'a> Attribute<'a> {
    fn parse_literal(input: &'a str) -> IResult<&'a str, &'a str> {
        delimited(
            tag("\""),
            take_while(|c: char| c != '\"' && c != '<' && c != '&'),
            tag("\""),
        )(input)
    }

    fn parse_definition(input: &'a str) -> IResult<&'a str, Attribute<'a>> {
        let (input, name) = ContentParticle::parse_name(input)?;
        let (input, _) = space0(input)?;
        let (input, att_type) = AttType::parse_att_type(input)?;
        let (input, _) = space0(input)?;
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
        let (input, _) = tag("=")(input)?;
        let (input, value) = Self::parse_literal(input)?;
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
pub enum CharRef<'a> {
    Decimal(Cow<'a, str>),
    Hexadecimal(Cow<'a, str>),
}

#[derive(Clone, PartialEq)]
pub enum Declaration<'a> {
    DocType {
        name: Option<Cow<'a, str>>,
        external_id: Option<ExternalID>,
        int_subset: Option<Vec<Declaration<'a>>>,
    },
    Element {
        name: Option<Cow<'a, str>>,
        content_spec: Option<DeclarationContent<'a>>,
    },
    AttList {
        name: Option<Cow<'a, str>>,
        att_defs: Option<Vec<Attribute<'a>>>, //Attribute::Definition
    },
}

impl<'a> Declaration<'a> {
    fn parse_doctype(input: &'a str) -> IResult<&'a str, Declaration<'a>> {
        let (input, name) = opt(alpha1)(input)?;
        let (input, _) = space0(input)?;
        let (input, external_id) = opt(alt((
            map(tag("SYSTEM"), |_| ExternalID::System),
            map(tag("PUBLIC"), |_| ExternalID::Public),
        )))(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("[")(input)?;

        let (input, _) = multispace0(input)?;
        //ChatGPT HERE, I think int_subset needs to be a list that is
        let (input, int_subset) = opt(many0(alt((Self::parse, Self::parse_attlist))))(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("]")(input)?;
        let (input, _) = tag(">")(input)?;
        if int_subset.is_some() {
            Ok((
                input,
                Self::DocType {
                    name: name.map(|s| s.into()),
                    external_id,
                    int_subset: int_subset,
                },
            ))
        } else {
            Ok((
                input,
                Self::DocType {
                    name: name.map(|s| s.into()),
                    external_id,
                    int_subset: None,
                },
            ))
        }
    }

    fn parse_element(input: &'a str) -> IResult<&'a str, Declaration<'a>> {
        let (input, element_name) = opt(alpha1)(input)?;
        let (input, _) = space0(input)?;

        let (input, content_spec) = DeclarationContent::parse_spec(input)?;
        let (input, _) = tag(">")(input)?;

        Ok((
            input,
            Declaration::Element {
                name: element_name.map(|s| s.into()),
                content_spec: Some(content_spec),
            },
        ))
    }

    pub fn parse_attlist(input: &'a str) -> IResult<&'a str, Declaration<'a>> {
        let (input, _) = preceded(multispace0, tag("<!ATTLIST"))(input)?;
        let (input, _) = space0(input)?;
        let (input, name) = ContentParticle::parse_name(input)?;
        let (input, _) = space0(input)?;

        let (input, att_defs) =
            many0(delimited(space0, Attribute::parse_definition, space0))(input)?;

        let (input, _) = tag(">")(input)?;

        Ok((
            input,
            Declaration::AttList {
                name: Some(name),
                att_defs: Some(att_defs),
            },
        ))
    }

    pub fn parse(input: &'a str) -> IResult<&'a str, Declaration<'a>> {
        let (input, _) = tag("<!")(input)?;
        let (input, decl) = opt(alpha1)(input)?;
        let (input, _) = space0(input)?;
        match decl {
            Some("DOCTYPE") => Self::parse_doctype(input),
            Some("ELEMENT") => Self::parse_element(input),
            Some("ATTLIST") => Self::parse_attlist(input),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            ))),
        }
    }
}
