mod debug;
mod declaration;

use declaration::Declaration;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::{
        complete::{alpha1, space0},
        is_alphanumeric,
    },
    combinator::{map, opt, recognize, value},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::path::Path;
use std::{borrow::Cow, io::Error as IoError};
use std::{
    fs::{self, File},
    io::Read,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Namespace<'a> {
    pub prefix: Cow<'a, str>,
    pub uri: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConditionalState {
    None,
    Optional,
    ZeroOrMore,
    OneOrMore,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TagState {
    Start,
    End,
}

#[derive(Clone, PartialEq)]
pub enum Tag<'a> {
    Tag {
        name: Cow<'a, str>,
        namespace: Option<Namespace<'a>>,
        state: TagState,
    },
}

#[derive(Clone, PartialEq)]
pub enum Document<'a> {
    Declaration(Option<Declaration<'a>>),
    Element(Tag<'a>, Box<Document<'a>>, Tag<'a>),
    Text(Cow<'a, str>),
    Content(Option<Cow<'a, str>>),
    Nested(Vec<Document<'a>>),
    Empty,
    Comment(Option<Cow<'a, str>>),
}

impl<'a> Document<'a> {
    fn parse_tag(input: &'a str) -> IResult<&'a str, (Cow<'a, str>, Option<Namespace<'a>>)> {
        alt((
            // Parse starting tags
            map(
                delimited(
                    tag("<"),
                    recognize(pair(
                        // Look for an optional namespace prefix
                        opt(pair(alpha1, tag(":"))),
                        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
                    )),
                    tag(">"),
                ),
                |tag_name: &str| {
                    // Check if there's a namespace prefix
                    let mut parts = tag_name.split(':');
                    if let (Some(prefix), Some(local_name)) = (parts.next(), parts.next()) {
                        (
                            Cow::Borrowed(local_name),
                            Some(Namespace {
                                prefix: Cow::Borrowed(prefix),
                                uri: None,
                            }),
                        )
                    } else {
                        (Cow::Borrowed(tag_name), None)
                    }
                },
            ),
            // Parse closing tags
            map(
                delimited(
                    tag("</"),
                    take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == ':'),
                    tag(">"),
                ),
                |tag_name: &str| {
                    let mut parts = tag_name.split(':');
                    if let (Some(prefix), Some(local_name)) = (parts.next(), parts.next()) {
                        (
                            Cow::Borrowed(local_name),
                            Some(Namespace {
                                prefix: Cow::Borrowed(prefix),
                                uri: None,
                            }),
                        )
                    } else {
                        (Cow::Borrowed(tag_name), None)
                    }
                },
            ),
        ))(input)
    }
    fn parse_content(input: &'a str) -> IResult<&'a str, Option<&'a str>> {
        let (tail, content) = take_until("</")(input)?;
        if content.is_empty() {
            Ok((tail, None))
        } else {
            Ok((tail, Some(content)))
        }
    }

    // Helper function to combine parsing and ignoring whitespace
    fn parse_with_whitespace<F, O>(input: &'a str, mut parser: F) -> IResult<&'a str, O>
    where
        F: FnMut(&'a str) -> IResult<&'a str, O>,
    {
        let (input, _) = space0(input)?;
        let (input, result) = parser(input)?;
        let (input, _) = space0(input)?;
        Ok((input, result))
    }

    pub fn parse_xml_str(input: &'a str) -> IResult<&'a str, Document<'a>> {
        //let (input, declaration) = Self::parse_with_whitespace(input, opt(Self::parse_declaration))?;
        //println!("ParseDeclaration {declaration:?}");
        let (input, start_tag) = Self::parse_tag(input)?;
        let (input, _) = space0(input)?;
        let (input, children) =
            Self::parse_with_whitespace(input, |i| many0(Self::parse_xml_str)(i))?;
        let (input, content) = Self::parse_content(input)?;
        let (input, _) = space0(input)?;
        let (input, end_tag) = Self::parse_tag(input)?;

        match (start_tag, end_tag) {
            ((start_name, start_namespace), (end_name, end_namespace)) => {
                if start_name == end_name && start_namespace == end_namespace {
                    let child_document = determine_child_document(content, children);

                    let element = Document::Element(
                        Tag::Tag {
                            name: start_name,
                            namespace: start_namespace,
                            state: TagState::Start,
                        },
                        Box::new(child_document),
                        Tag::Tag {
                            name: end_name,
                            namespace: end_namespace,
                            state: TagState::End,
                        },
                    );
                    Ok((input, element))
                } else {
                    Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Verify,
                    )))
                }
            }
        }
    }

    pub fn parse_tag_contents(
        input: &'a str,
        xml_tag: &'a str,
    ) -> IResult<&'a str, Vec<Document<'a>>> {
        let start_tag = format!("<{xml_tag}");
        let end_tag = format!("</{xml_tag}");
        let (input, _) = take_until(start_tag.as_str())(input)?;

        Self::parse_with_whitespace(input, |i| many0(Self::parse_xml_str)(i))
    }
}

// Helper function to determine the child Document type
fn determine_child_document<'a>(
    content: Option<&'a str>,
    children: Vec<Document<'a>>,
) -> Document<'a> {
    if let Some(content) = content {
        Document::Content(Some(Cow::Borrowed(content)))
    } else if children.is_empty() {
        Document::Empty
    } else if children.len() == 1 {
        children.into_iter().next().unwrap()
    } else {
        Document::Nested(children)
    }
}

pub fn parse_file(
    file: &mut File,
) -> Result<Document<'static>, nom::Err<nom::error::Error<String>>> {
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let content = Box::leak(content.into_boxed_str());
    let (_, document) = Document::parse_xml_str(content).map_err(|err| {
        err.map(|inner_err| nom::error::Error::new(inner_err.input.to_string(), inner_err.code))
    })?;
    Ok(document)
}

pub fn parse_directory(
    path: &Path,
) -> Result<Vec<Result<Document, nom::Err<nom::error::Error<String>>>>, IoError> {
    let entries = fs::read_dir(path)?;
    let results = entries
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("xml")) // Fix the E0369 error by adding `to_str()` here.
        .map(|entry| {
            let mut file = File::open(entry.path()).unwrap();
            parse_file(&mut file)
        })
        .collect::<Vec<_>>();
    Ok(results)
}
