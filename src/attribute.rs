use crate::{
    namespaces::ParseNamespace, parse::Parse, prolog::subset::entity_value::EntityValue,
    reference::Reference, Name, QualifiedName,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::char,
    combinator::{map, map_res, opt, value},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, tuple},
    IResult,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, PartialEq)]
pub enum Prefix {
    Default,
    Prefix(String),
}
#[derive(Clone, PartialEq)]
pub enum AttributeValue {
    Value(String),
    Values(Vec<AttributeValue>),
    Reference(Reference),
}

#[derive(Clone, PartialEq)]
pub enum Attribute {
    Definition {
        name: QualifiedName,
        att_type: AttType,
        default_decl: DefaultDecl,
    },
    Reference(Reference),
    Instance {
        name: QualifiedName,
        value: AttributeValue,
    },
    Required,
    Implied,
    Namespace {
        prefix: Prefix,
        uri: AttributeValue,
    },
}

impl<'a> Parse<'a> for Attribute {
    type Args = Rc<RefCell<HashMap<Name, EntityValue>>>;
    type Output = IResult<&'a str, Self>;

    // [41] Attribute ::= Name Eq AttValue
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        map(
            tuple((Self::parse_name, Self::parse_eq, |i| {
                Self::parse_attvalue(i, args.clone())
            })),
            |(name, _eq, value)| Attribute::Instance { name, value },
        )(input)
    }
}

impl<'a> ParseNamespace<'a> for Attribute {}
impl Attribute {
    // [53] AttDef ::= S Name S AttType S DefaultDecl
    pub fn parse_definition(
        input: &str,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> IResult<&str, Attribute> {
        map(
            tuple((
                Self::parse_multispace1,
                Self::parse_name,
                Self::parse_multispace1,
                |i| AttType::parse(i, ()),
                Self::parse_multispace1,
                |i| DefaultDecl::parse(i, entity_references.clone()),
            )),
            |(_whitespace1, name, _whitespace2, att_type, _whitespace3, default_decl)| {
                Attribute::Definition {
                    name,
                    att_type,
                    default_decl,
                }
            },
        )(input)
    }

    // Namespaces (Third Edition) [21] AttDef ::= S (QName | NSAttName) S AttType S DefaultDecl
    pub fn parse_qualified_definition(
        input: &str,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> IResult<&str, Attribute> {
        map(
            tuple((
                Self::parse_multispace1,
                alt((
                    Self::parse_qualified_name,
                    Self::parse_namespace_attribute_name,
                )),
                Self::parse_multispace1,
                |i| AttType::parse(i, ()),
                Self::parse_multispace1,
                |i| DefaultDecl::parse(i, entity_references.clone()),
            )),
            |(_whitespace1, name, _whtiespace2, att_type, _whtiespace3, default_decl)| {
                Attribute::Definition {
                    name,
                    att_type,
                    default_decl,
                }
            },
        )(input)
    }

    // [10] AttValue ::= '"' ([^<&"] | Reference)* '"'|  "'" ([^<&'] | Reference)* "'"
    pub fn parse_attvalue(
        input: &str,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> IResult<&str, AttributeValue> {
        map(
            alt((
                delimited(
                    tag("\""),
                    many0(alt((
                        map(
                            take_till1(|c| c == '<' || c == '&' || c == '\"'),
                            |s: &str| AttributeValue::Value(s.into()),
                        ),
                        map(
                            |i| Reference::parse(i, entity_references.clone()),
                            |reference| reference.normalize_attribute(entity_references.clone()),
                        ),
                    ))),
                    tag("\""),
                ),
                delimited(
                    tag("'"),
                    many0(alt((
                        map(
                            take_till1(|c| c == '<' || c == '&' || c == '\''),
                            |s: &str| AttributeValue::Value(s.into()),
                        ),
                        map(
                            |i| Reference::parse(i, entity_references.clone()),
                            |reference| reference.normalize_attribute(entity_references.clone()),
                        ),
                    ))),
                    tag("'"),
                ),
            )),
            |contents: Vec<AttributeValue>| {
                let mut buffer = String::new();

                for content in contents {
                    if let AttributeValue::Value(mut value) = content {
                        // End-of-Line Handling for each value
                        let mut chars: Vec<char> = value.chars().collect();
                        let mut i = 0;
                        while i < chars.len() {
                            if chars[i] == '\r' {
                                if i + 1 < chars.len() && chars[i + 1] == '\n' {
                                    chars.remove(i);
                                } else {
                                    chars[i] = '\n';
                                }
                            }
                            i += 1;
                        }
                        value = chars.into_iter().collect();
                        buffer.push_str(&value);
                    }
                }

                AttributeValue::Value(buffer)
            },
        )(input)
    }

    // Namespaces (Third Edition) [15] Attribute ::= NSAttName Eq AttValue | QName Eq AttValue
    pub fn parse_attribute(
        input: &str,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> IResult<&str, Attribute> {
        map(
            alt((
                tuple((Self::parse_namespace_attribute_name, Self::parse_eq, |i| {
                    Attribute::parse_attvalue(i, entity_references.clone())
                })),
                tuple((Self::parse_qualified_name, Self::parse_eq, |i| {
                    Self::parse_attvalue(i, entity_references.clone())
                })),
            )),
            |result| match result {
                (name, _eq, value) if name.prefix.is_some() => {
                    let prefix = name.prefix.unwrap();

                    if &prefix == "xmlns" {
                        Attribute::Namespace {
                            prefix: Prefix::Default,
                            uri: value,
                        }
                    } else {
                        Attribute::Namespace {
                            prefix: Prefix::Prefix(prefix),
                            uri: value,
                        }
                    }
                }
                (QualifiedName { prefix, local_part }, _eq, value) => Attribute::Instance {
                    name: QualifiedName { prefix, local_part },
                    value,
                },
            },
        )(input)
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
pub enum AttType {
    CDATA,
    Tokenized(TokenizedType),
    Enumerated {
        notation: Option<Vec<Name>>,
        enumeration: Option<Vec<String>>,
    },
}

impl<'a> Parse<'a> for AttType {
    type Args = ();
    type Output = IResult<&'a str, Self>;
    //[54] AttType ::=  StringType | TokenizedType | EnumeratedType
    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
        let (input, att_type) = map(
            alt((
                // [55] StringType ::= 'CDATA'
                value(AttType::CDATA, tag("CDATA")),
                // [56] TokenizedType ::= 'ID'| 'IDREF' | 'IDREFS' | 'ENTITY' | 'ENTITIES' | 'NMTOKEN' | 'NMTOKENS'
                map(TokenizedType::parse, AttType::Tokenized),
                Self::parse_enumerated_type,
            )),
            |parsed_att_type| parsed_att_type,
        )(input)?;

        Ok((input, att_type))
    }
}
impl AttType {
    // [57] EnumeratedType ::= NotationType | Enumeration
    fn parse_enumerated_type(input: &str) -> IResult<&str, AttType> {
        alt((Self::parse_notation_type, Self::parse_enumeration))(input)
    }

    // [58] NotationType ::= 'NOTATION' S '(' S? Name (S? '|' S? Name)* S? ')'
    fn parse_notation_type(input: &str) -> IResult<&str, AttType> {
        map(
            tuple((
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
            )),
            |(_notation_literal, _whitespace, names)| AttType::Enumerated {
                notation: Some(names),
                enumeration: None,
            },
        )(input)
    }

    // [59] Enumeration ::= '(' S? Nmtoken (S? '|' S? Nmtoken)* S? ')'
    fn parse_enumeration(input: &str) -> IResult<&str, AttType> {
        map(
            delimited(
                char('('),
                separated_list1(
                    tuple((Self::parse_multispace0, char('|'), Self::parse_multispace0)),
                    Self::parse_nmtoken,
                ),
                char(')'),
            ),
            |enumeration| AttType::Enumerated {
                notation: None,
                enumeration: Some(enumeration),
            },
        )(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DefaultDecl {
    Required,
    Implied,
    Fixed(String),
    Value(String),
}

impl<'a> Parse<'a> for DefaultDecl {
    type Args = Rc<RefCell<HashMap<Name, EntityValue>>>;
    type Output = IResult<&'a str, Self>;
    // [60] DefaultDecl ::= '#REQUIRED' | '#IMPLIED' | (('#FIXED' S)? AttValue)
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        alt((
            value(DefaultDecl::Required, tag("#REQUIRED")),
            value(DefaultDecl::Implied, tag("#IMPLIED")),
            map_res(
                pair(opt(tuple((tag("#FIXED"), Self::parse_multispace1))), |i| {
                    Attribute::parse_attvalue(i, args.clone())
                }),
                |(fixed, attvalue)| {
                    if let AttributeValue::Value(value) = attvalue {
                        match fixed {
                            Some(_) => Ok(DefaultDecl::Fixed(value)),
                            None => Ok(DefaultDecl::Value(value)),
                        }
                    } else {
                        Err(nom::Err::Failure(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Fail,
                        )))
                    }
                },
            ),
        ))(input)
    }
}
