use crate::{
    namespaces::ParseNamespace, parse::Parse, prolog::internal_subset::EntityValue,
    reference::Reference, Name, QualifiedName,
};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    combinator::{map, map_res, opt, value},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, tuple},
    IResult,
};
use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, PartialEq)]
pub enum Prefix<'a> {
    Default,
    Prefix(Cow<'a, str>),
}

#[derive(Clone, PartialEq)]
pub enum Attribute<'a> {
    Definition {
        name: QualifiedName<'a>,
        att_type: AttType<'a>,
        default_decl: DefaultDecl<'a>,
    },
    Reference(Reference<'a>),
    Instance {
        name: QualifiedName<'a>,
        value: Cow<'a, str>,
    },
    Required,
    Implied,
    Namespace {
        prefix: Prefix<'a>,
        uri: Cow<'a, str>,
    },
}

impl<'a> Parse<'a> for Attribute<'a> {
    type Args = Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>;

    type Output = IResult<&'a str, Self>;
    // [41] Attribute ::= Name Eq AttValue
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        let (input, name) = Self::parse_name(input)?;
        let (input, _) = Self::parse_eq(input)?;
        let (input, value) = Self::parse_attvalue(input, args.clone())?;
        Ok((input, Attribute::Instance { name, value }))
    }
}

impl<'a> ParseNamespace<'a> for Attribute<'a> {}
impl<'a> Attribute<'a> {
    // [53] AttDef ::= S Name S AttType S DefaultDecl
    pub fn parse_definition(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, Attribute<'a>> {
        let (input, (_, name, _, att_type, _, default_decl)) = tuple((
            Self::parse_multispace1,
            Self::parse_name,
            Self::parse_multispace1,
            |i| AttType::parse(i, ()),
            Self::parse_multispace1,
            |i| DefaultDecl::parse(i, entity_references.clone()),
        ))(input)?;

        let attribute = Attribute::Definition {
            name,
            att_type,
            default_decl,
        };
        Ok((input, attribute))
    }

    // Namespaces (Third Edition) [21] AttDef ::= S (QName | NSAttName) S AttType S DefaultDecl
    pub fn parse_qualified_definition(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, Attribute<'a>> {
        let (input, (_, name, _, att_type, _, default_decl)) = tuple((
            Self::parse_multispace1,
            alt((
                Self::parse_qualified_name,
                Self::parse_namespace_attribute_name,
            )),
            Self::parse_multispace1,
            |i| AttType::parse(i, ()),
            Self::parse_multispace1,
            |i| DefaultDecl::parse(i, entity_references.clone()),
        ))(input)?;

        let attribute = Attribute::Definition {
            name,
            att_type,
            default_decl,
        };
        Ok((input, attribute))
    }

    // [10] AttValue ::= '"' ([^<&"] | Reference)* '"'|  "'" ([^<&'] | Reference)* "'"
    pub fn parse_attvalue(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, Cow<'a, str>> {
        alt((
            delimited(
                tag("\""),
                many0(alt((
                    map(is_not("<&\""), Cow::Borrowed),
                    map(
                        |i| Reference::parse(i, entity_references.clone()),
                        |reference| reference.normalize(entity_references.clone()),
                    ),
                ))),
                tag("\""),
            ),
            delimited(
                tag("'"),
                many0(alt((
                    map(is_not("<&'"), Cow::Borrowed),
                    map(
                        |i| Reference::parse(i, entity_references.clone()),
                        |reference| reference.normalize(entity_references.clone()),
                    ),
                ))),
                char('\''),
            ),
        ))(input)
        .map(|(remaining, contents)| {
            println!("buff CONTENTS: {contents:?}"); //TODO: decoding is not working here
            let mut buffer = contents
                .into_iter()
                .flat_map(|cow| cow.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            // End-of-Line Handling: https://www.w3.org/TR/2008/REC-xml-20081126/#sec-line-ends
            let mut i = 0;
            while i < buffer.len() {
                if buffer[i] == '\r' {
                    if i + 1 < buffer.len() && buffer[i + 1] == '\n' {
                        buffer.remove(i);
                    } else {
                        buffer[i] = '\n';
                    }
                }
                i += 1;
            }

            (remaining, Cow::Owned(buffer.into_iter().collect()))
        })
    }

    // Namespaces (Third Edition) [15] Attribute ::= NSAttName Eq AttValue | QName Eq AttValue
    pub fn parse_qualified_attribute(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, Attribute<'a>> {
        alt((
            map_res(
                tuple((Self::parse_namespace_attribute_name, Self::parse_eq, |i| {
                    Attribute::parse_attvalue(i, entity_references.clone())
                })),
                |(name, _, value)| {
                    // If name is "xmlns", it's a default namespace declaration
                    if let Some(prefix) = name.prefix {
                        if &prefix == "xmlns" {
                            Ok(Attribute::Namespace {
                                prefix: Prefix::Default,
                                uri: value,
                            })
                        } else {
                            // Otherwise, it's a prefixed namespace declaration
                            Ok(Attribute::Namespace {
                                prefix: Prefix::Prefix(prefix),
                                uri: value,
                            })
                        }
                    } else {
                        Err(("Attribute without prefix", nom::error::ErrorKind::MapRes))
                    }
                },
            ),
            map(
                tuple((Self::parse_qualified_name, Self::parse_eq, |i| {
                    Self::parse_attvalue(i, entity_references.clone())
                })),
                |(QualifiedName { prefix, local_part }, _, value)| Attribute::Instance {
                    name: QualifiedName { prefix, local_part },
                    value,
                },
            ),
        ))(input)
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
    // [56] TokenizedType ::= 'ID' | 'IDRef' | 'IDREFS | 'ENTITY' | 'ENTITIES' | 'NMTOKEN' | 'NMTOKENS'
    fn parse(input: &str) -> IResult<&str, TokenizedType> {
        alt((
            value(TokenizedType::IDREFS, tag("IDREFS")),
            value(TokenizedType::IDREF, tag("IDREF")),
            value(TokenizedType::ID, tag("ID")),
            value(TokenizedType::ENTITY, tag("ENTITY")),
            value(TokenizedType::ENTITIES, tag("ENTITIES")),
            value(TokenizedType::NMTOKENS, tag("NMTOKENS")),
            value(TokenizedType::NMTOKEN, tag("NMTOKEN")),
        ))(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AttType<'a> {
    CDATA,
    Tokenized(TokenizedType),
    Enumerated {
        notation: Option<Vec<Name<'a>>>,
        enumeration: Option<Vec<Cow<'a, str>>>,
    },
}

impl<'a> Parse<'a> for AttType<'a> {
    type Args = ();
    type Output = IResult<&'a str, Self>;
    //[54] AttType ::=  StringType | TokenizedType | EnumeratedType
    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
        let (input, att_type) = alt((
            // [55] StringType ::= 'CDATA'
            value(AttType::CDATA, tag("CDATA")),
            // [56] TokenizedType ::= 'ID'| 'IDREF' | 'IDREFS' | 'ENTITY' | 'ENTITIES' | 'NMTOKEN' | 'NMTOKENS'
            map(TokenizedType::parse, AttType::Tokenized),
            Self::parse_enumerated_type,
        ))(input)?;
        println!("PARSED ATTRIBUTE TYPE: {:?}", att_type);
        Ok((input, att_type))
    }
}
impl<'a> AttType<'a> {
    // [57] EnumeratedType ::= NotationType | Enumeration
    fn parse_enumerated_type(input: &'a str) -> IResult<&'a str, AttType<'a>> {
        alt((Self::parse_notation_type, Self::parse_enumeration))(input)
    }

    // [58] NotationType ::= 'NOTATION' S '(' S? Name (S? '|' S? Name)* S? ')'
    fn parse_notation_type(input: &'a str) -> IResult<&'a str, AttType<'a>> {
        let (input, (_, _, names)) = tuple((
            tag("NOTATION"),
            Self::parse_multispace1,
            delimited(
                char('('),
                delimited(
                    Self::parse_multispace0,
                    separated_list1(
                        delimited(Self::parse_multispace0, char('|'), Self::parse_multispace0),
                        Self::parse_name,
                    ),
                    Self::parse_multispace0,
                ),
                char(')'),
            ),
        ))(input)?;

        //let names = names.into_iter().collect();

        Ok((
            input,
            AttType::Enumerated {
                notation: Some(names),
                enumeration: None,
            },
        ))
    }

    // [59] Enumeration ::= '(' S? Nmtoken (S? '|' S? Nmtoken)* S? ')'
    fn parse_enumeration(input: &'a str) -> IResult<&'a str, AttType<'a>> {
        let mut parser = delimited(
            char('('),
            separated_list1(
                tuple((Self::parse_multispace0, char('|'), Self::parse_multispace0)),
                Self::parse_nmtoken,
            ),
            char(')'),
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
}

#[derive(Clone, Debug, PartialEq)]
pub enum DefaultDecl<'a> {
    Required,
    Implied,
    Fixed(Cow<'a, str>),
    Value(Cow<'a, str>),
}

impl<'a> Parse<'a> for DefaultDecl<'a> {
    type Args = Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>;
    type Output = IResult<&'a str, Self>;
    // [60] DefaultDecl ::= '#REQUIRED' | '#IMPLIED' | (('#FIXED' S)? AttValue)
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        alt((
            value(DefaultDecl::Required, tag("#REQUIRED")),
            value(DefaultDecl::Implied, tag("#IMPLIED")),
            map(
                pair(opt(tuple((tag("#FIXED"), Self::parse_multispace1))), |i| {
                    Attribute::parse_attvalue(i, args.clone())
                }),
                |(fixed, attvalue)| match fixed {
                    Some(_) => DefaultDecl::Fixed(attvalue),
                    None => DefaultDecl::Value(attvalue),
                },
            ),
        ))(input)
    }
}
