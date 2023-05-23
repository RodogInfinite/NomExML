pub mod attribute;
mod debug;
pub mod declaration;
mod error;

use attribute::Attribute;
use declaration::Declaration;
use error::CustomError;
use nom::{
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{alpha1, multispace0},
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{borrow::Cow, io::Error as IoError, path::Path};
use std::{
    fs::{self, File},
    io::Read,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Namespace<'a> {
    pub declaration: Option<Cow<'a, str>>,
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
pub struct Tag<'a> {
    pub name: Cow<'a, str>,
    pub namespace: Option<Namespace<'a>>,
    pub attributes: Option<Vec<Attribute<'a>>>, // Attribute::Instance
    pub state: TagState,
}

impl<'a> Tag<'a> {
    fn parse_attributes(input: &'a str) -> IResult<&'a str, Option<Vec<Attribute<'a>>>> {
        let mut parser = many0(Attribute::parse_attribute_instance);
        let (input, attributes) = parser(input)?;
        if attributes.is_empty() {
            Ok((input, None))
        } else {
            Ok((input, Some(attributes)))
        }
    }

    fn parse_start_tag(input: &'a str) -> IResult<&'a str, Self> {
        map(
            delimited(
                tag("<"),
                tuple((
                    delimited(multispace0, Document::parse_tag_and_namespace, multispace0),
                    Self::parse_attributes,
                )),
                tag(">"),
            ),
            |((name, namespace), attributes)| Self {
                name,
                namespace,
                attributes,
                state: TagState::Start,
            },
        )(input)
    }

    fn parse_end_tag(input: &'a str) -> IResult<&'a str, Self> {
        map(
            delimited(
                preceded(tag("</"), multispace0),
                Document::parse_tag_and_namespace,
                preceded(multispace0, tag(">")),
            ),
            |(name, namespace)| Self {
                name,
                namespace,
                attributes: None, // Attributes are not parsed for end tags
                state: TagState::End,
            },
        )(input)
    }
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
    fn parse_tag_and_namespace(
        input: &'a str,
    ) -> IResult<&'a str, (Cow<'a, str>, Option<Namespace<'a>>)> {
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
                            declaration: None,
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

    fn parse_content(input: &'a str) -> IResult<&'a str, Option<Cow<'a, str>>> {
        let (tail, content) = take_until("</")(input)?;
        if content.is_empty() {
            Ok((tail, None))
        } else {
            let (_, content) = Self::decode_entities(content)?;
            Ok((tail, Some(content)))
        }
    }
    fn decode_entities(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, code) = opt(delimited(tag("&#"), take_while1(|c: char| c.is_numeric()), tag(";")))(input)?;
        println!("inputbefore: {:?}", input);
    
        if let Some(code) = code {
            println!("Code: {:?}", code);
            let decoded_entity = match code.parse::<u32>() {
                Ok(n) => match char::from_u32(n) {
                    Some(c) => Cow::Owned(c.to_string()),
                    None => Cow::Owned(format!("Invalid Unicode scalar value: {}", n)),
                },
                Err(_) => Cow::Owned(format!("Invalid decimal number: {}", code)),
            };
            println!("Decoded entity: {}", decoded_entity);
            println!("Input: {}", input);
            Ok((input, decoded_entity))
        }
        else {
            Ok((input, Cow::Borrowed(input)))
        }
    }

    // Helper function to combine parsing and ignoring whitespace
    fn parse_with_whitespace<F, O>(input: &'a str, mut parser: F) -> IResult<&'a str, O>
    where
        F: FnMut(&'a str) -> IResult<&'a str, O>,
    {
        let (input, _) = multispace0(input)?;
        let (input, result) = parser(input)?;
        let (input, _) = multispace0(input)?;
        Ok((input, result))
    }

    pub fn parse_xml_str(input: &'a str) -> IResult<&'a str, Document<'a>> {
        let (input, declaration) = Self::parse_declaration(input)?;
        let (input, start_tag) = Tag::parse_start_tag(input)?;
        let (input, children) = Self::parse_children(input)?;
        let (input, content) = Self::parse_content(input)?;
        println!("HERE: {content:?}");
        println!("HEREINPUT: {input:?}");
        let (input, end_tag) = Tag::parse_end_tag(input)?;

        Self::construct_document(input, declaration, start_tag, children, content, end_tag)
    }

    fn parse_declaration(input: &'a str) -> IResult<&'a str, Option<Declaration<'a>>> {
        Self::parse_with_whitespace(input, opt(Declaration::parse))
    }

    fn parse_children(input: &'a str) -> IResult<&'a str, Vec<Document<'a>>> {
        Self::parse_with_whitespace(input, many0(Self::parse_xml_str))
    }

    fn construct_document_with_declaration(
        declaration: Option<Declaration<'a>>,
        start_tag: &Tag<'a>,
        child_document: Document<'a>,
        end_tag: &Tag<'a>,
    ) -> Document<'a> {
        Document::Nested(vec![
            Document::Declaration(declaration),
            Document::Element(start_tag.clone(), Box::new(child_document), end_tag.clone()),
        ])
    }

    fn construct_element(
        start_tag: &Tag<'a>,
        child_document: Document<'a>,
        end_tag: &Tag<'a>,
    ) -> Document<'a> {
        Document::Element(start_tag.clone(), Box::new(child_document), end_tag.clone())
    }

    fn construct_document(
        input: &'a str,
        declaration: Option<Declaration<'a>>,
        start_tag: Tag<'a>,
        children: Vec<Document<'a>>,
        content: Option<Cow<'a, str>>,
        end_tag: Tag<'a>,
    ) -> IResult<&'a str, Document<'a>> {
        println!("Constructing document: {start_tag:#?} {end_tag:#?}");
        match (&start_tag, &end_tag) {
            (
                Tag {
                    name: start_name,
                    namespace: start_namespace,
                    ..
                },
                Tag {
                    name: end_name,
                    namespace: end_namespace,
                    ..
                },
            ) if start_name == end_name && start_namespace == end_namespace => {
                let child_document = determine_child_document(content, children).map_err(|e| {
                    nom::Err::Failure(nom::error::Error::new(e, nom::error::ErrorKind::Verify))
                })?;
                println!("Child document: {child_document:#?}");
                let document = if let Some(declaration) = declaration {
                    Self::construct_document_with_declaration(
                        Some(declaration),
                        &start_tag,
                        child_document,
                        &end_tag,
                    )
                } else {
                    Self::construct_element(&start_tag, child_document, &end_tag)
                };
                Ok((input, document))
            }
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            ))),
        }
    }

    pub fn get_tags(&'a self, tag_name: &'a str) -> Elements<'a> {
        let mut results = Vec::new();
        self.get_internal_tags(tag_name, &mut results);
        Elements { tags: results }
    }

    pub fn get_internal_tags(&'a self, tag_name: &str, results: &mut Vec<&'a Self>) {
        match self {
            Document::Element(
                Tag {
                    name, namespace, ..
                },
                content,
                _,
            ) => {
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
fn determine_child_document<'a>(
    content: Option<Cow<'a, str>>,
    children: Vec<Document<'a>>,
) -> Result<Document<'a>, &'static str> {
    if let Some(content) = content {
        Ok(Document::Content(Some(Cow::Owned(content.as_ref().to_string()))))
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

pub fn read_file(file: &mut File) -> Result<String, IoError> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn parse_file(file: &mut File) -> Result<Document<'static>, CustomError> {
    let content = read_file(file)?;
    let content = Box::leak(content.into_boxed_str());
    let (_, document) = Document::parse_xml_str(content).map_err(|err| match err {
        nom::Err::Error(e) | nom::Err::Failure(e) => {
            CustomError::NomError(format!("error: {:?}, input: {}", e.code, e.input))
        }
        nom::Err::Incomplete(_) => CustomError::NomError("Incomplete parsing".to_string()),
    })?;
    Ok(document)
}

pub fn parse_directory(path: &Path) -> Result<Vec<Result<Document, CustomError>>, CustomError> {
    let entries = fs::read_dir(path)?;
    let results = entries
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("xml")) // Fix the E0369 error by adding `to_str()` here.
        .map(|entry| {
            let mut file = File::open(entry.path())?;
            parse_file(&mut file)
        })
        .collect::<Vec<_>>();
    Ok(results)
}
