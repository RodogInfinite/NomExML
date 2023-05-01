mod debug;
mod declaration;

use declaration::Declaration;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::{
        complete::{alpha1, space0}
    },
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, pair},
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
    Content(Option<Cow<'a, str>>),
    Nested(Vec<Document<'a>>),
    Empty,
    Comment(Option<Cow<'a, str>>),
}

impl<'a> Document<'a> {
    fn parse_tag_and_namespace(input: &'a str) -> IResult<&'a str, (Cow<'a, str>, Option<Namespace<'a>>)> {
        map(
            recognize(pair(
                // Look for an optional namespace prefix
                opt(pair(alpha1, tag(":"))),
                take_while1(|c: char| c.is_alphanumeric() || c == '_'),
            )),
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
        )(input)
    }
    fn parse_tag_name(input: &'a str) -> IResult<&'a str, (Cow<'a, str>, Option<Namespace<'a>>)> {
        alt((
            // Parse starting tags
            map(
                delimited(
                    tag("<"),
                    Self::parse_tag_and_namespace,
                    tag(">"),
                ),
                |(tag_name, namespace)| {
                    (tag_name, namespace)
                },
            ),
            // Parse closing tags
            map(
                delimited(
                    tag("</"),
                    Self::parse_tag_and_namespace,
                    tag(">"),
                ),
                |(tag_name, namespace)| {
                    (tag_name, namespace)
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
        let (input, start_tag) = Self::parse_tag_name(input)?;
        let (input, _) = space0(input)?;
        let (input, children) =
            Self::parse_with_whitespace(input, |i| many0(Self::parse_xml_str)(i))?;
        let (input, content) = Self::parse_content(input)?;
        let (input, _) = space0(input)?;
        let (input, end_tag) = Self::parse_tag_name(input)?;
    
        match (start_tag, end_tag) {
            ((start_name, start_namespace), (end_name, end_namespace)) => {
                if start_name == end_name && start_namespace == end_namespace {
                    let child_document = determine_child_document(content, children).map_err(|e| {
                        nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Verify))
                    })?;
    
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
    

    pub fn parse_tag(
        input: &'a str,
        xml_tag: &'a str,
    ) -> IResult<&'a str, Vec<Document<'a>>> {
        let start_tag = format!("<{xml_tag}");
        let end_tag = format!("</{xml_tag}");
        let (input, _) = take_until(start_tag.as_str())(input)?;

        Self::parse_with_whitespace(input, |i| many0(Self::parse_xml_str)(i))
    }
    
    pub fn get_tags(&'a self, tag_name: &'a str) -> Elements<'a> {
        let mut results = Vec::new();
        self.get_internal_tags(tag_name, &mut results);
        Elements { tags: results }
    }

    fn get_internal_tags(&'a self, tag_name: &str, results: &mut Vec<&'a Self>) {
        match self {
            Document::Element(Tag::Tag { name, namespace, .. }, content, _) => {
                if let Some(namespace) = namespace {
                    if tag_name == &(namespace.prefix.to_string() + ":" + name) {
                        results.push(self);
                    }
                } else if name == tag_name {
                    results.push(self);
                }
                content.get_internal_tags(tag_name, results);
            }
            Document::Nested(docs) => {
                let mut docs_iter = docs.iter();
                while let Some(doc) = docs_iter.next() {
                    doc.get_internal_tags(tag_name, results);
                }
            }
            _ => (),
        }
    }
    
    fn extract_content(&'a self) -> Option<&'a str> {
        match self {
            Document::Element(_, content, _) => content.extract_content(),
            Document::Content(Some(content)) => Some(content),
            _ => None,
        }
    }
    
}

pub struct Elements<'a> {
    tags: Vec<&'a Document<'a>>,
}

impl<'a> Elements<'a> {
    pub fn extract_content(&self) -> Vec<Option<&'a str>> {
        self.tags.iter().map(|tag| tag.extract_content()).collect()
    }
}


// Helper function to determine the child Document type
// Helper function to determine the child Document type
fn determine_child_document<'a>(
    content: Option<&'a str>,
    children: Vec<Document<'a>>,
) -> Result<Document<'a>, &'static str> {
    if let Some(content) = content {
        Ok(Document::Content(Some(Cow::Borrowed(content))))
    } else if children.is_empty() {
        Ok(Document::Empty)
    } else if children.len() == 1 {
        match children.into_iter().next() {
            Some(child) => Ok(child),
            None => Err("Unexpected error: no child found in non-empty children vector"),
        }
    } else {
        Ok(Document::Nested(children))
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
