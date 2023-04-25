mod debug;

use std::{fs::File, io::Read};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::alpha1,
    combinator::{map, opt, recognize},
    error::convert_error,
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};

use rayon::prelude::*;
use std::fs::{self, DirEntry};
use std::io::Error as IoError;
use std::path::Path;

use std::io::BufReader;

#[derive(Clone, Debug, PartialEq)]
pub struct Namespace<'a> {
    pub prefix: &'a str,
    pub uri: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExternalID {
    Public,
    System,
}

#[derive(Clone, PartialEq)]
pub enum Element<'a> {
    DocType {
        name: &'a str,
        external_id: Option<ExternalID>,
        int_subset: Option<&'a str>,
    },
    Tag {
        open: bool,
        close: bool,
        name: &'a str,
        namespace: Option<Namespace<'a>>,
    },
    Node(Box<Element<'a>>, Box<Element<'a>>, Box<Element<'a>>),
    Content(Option<&'a str>),
    Nested(Vec<Element<'a>>),
    Comment(Option<&'a str>), //	Comment ::= '<!--' ((Char - '-') | ('-' (Char - '-')))* '-->'
}

impl<'a> Element<'a> {
    fn parse_tag(input: &'a str) -> IResult<&'a str, Element<'a>> {
        alt((
            // Parse opening tags
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
                        Element::Tag {
                            open: true,
                            close: false,
                            name: local_name,
                            namespace: Some(Namespace { prefix, uri: None }),
                        }
                    } else {
                        Element::Tag {
                            open: true,
                            close: false,
                            name: tag_name,
                            namespace: None,
                        }
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
                        Element::Tag {
                            open: false,
                            close: true,
                            name: local_name,
                            namespace: Some(Namespace { prefix, uri: None }),
                        }
                    } else {
                        Element::Tag {
                            open: false,
                            close: true,
                            name: tag_name,
                            namespace: None,
                        }
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

    fn parse_whitespace(input: &str) -> IResult<&str, &str> {
        take_while(|c: char| c.is_whitespace())(input)
    }

    // Helper function to combine parsing and ignoring whitespace
    fn parse_element_with_whitespace<F, O>(input: &'a str, parser: F) -> IResult<&'a str, O>
    where
        F: Fn(&'a str) -> IResult<&'a str, O>,
    {
        let (input, _) = Self::parse_whitespace(input)?;
        let (input, result) = parser(input)?;
        let (input, _) = Self::parse_whitespace(input)?;
        Ok((input, result))
    }

    pub fn parse_xml_str(input: &'a str) -> IResult<&'a str, Self> {
        let (input, open_element) = Self::parse_element_with_whitespace(input, Self::parse_tag)?;
        let (input, children) =
            Self::parse_element_with_whitespace(input, |i| many0(Self::parse_xml_str)(i))?;
        let (input, content) = Self::parse_element_with_whitespace(input, Self::parse_content)?;
        let (input, close_element) = Self::parse_element_with_whitespace(input, Self::parse_tag)?;

        match (&open_element, &close_element) {
            (
                Element::Tag {
                    open: true,
                    close: false,
                    ..
                },
                Element::Tag {
                    open: false,
                    close: true,
                    ..
                },
            ) => {
                let child_element = determine_child_element(content, children);

                // Modify open and close tags to have both open and close set to true
                let modified_open_element = match open_element {
                    Element::Tag {
                        open: true,
                        close: false,
                        name,
                        namespace,
                    } => Element::Tag {
                        open: true,
                        close: true,
                        name,
                        namespace,
                    },
                    _ => unreachable!(),
                };
                let modified_close_element = match close_element {
                    Element::Tag {
                        open: false,
                        close: true,
                        name,
                        namespace,
                    } => Element::Tag {
                        open: true,
                        close: true,
                        name,
                        namespace,
                    },
                    _ => unreachable!(),
                };

                Ok((
                    input,
                    Element::Node(
                        Box::new(modified_open_element),
                        Box::new(child_element),
                        Box::new(modified_close_element),
                    ),
                ))
            }
            (
                Element::Tag {
                    open: false,
                    close: true,
                    ..
                },
                _,
            ) => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            ))),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            ))),
        }
    }
}

// Helper function to determine the child element type
fn determine_child_element<'a>(
    content: Option<&'a str>,
    children: Vec<Element<'a>>,
) -> Element<'a> {
    if content.is_some() {
        Element::Content(content)
    } else if children.len() == 1 {
        children.into_iter().next().unwrap()
    } else {
        Element::Nested(children)
    }
}

pub fn parse_file(
    file: &mut File,
) -> Result<Element<'static>, nom::Err<nom::error::Error<String>>> {
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let content = Box::leak(content.into_boxed_str());
    let (_, element) = Element::parse_xml_str(content).map_err(|err| {
        err.map(|inner_err| nom::error::Error::new(inner_err.input.to_string(), inner_err.code))
    })?;
    Ok(element)
}

pub fn parse_directory(
    path: &Path,
) -> Result<Vec<Result<Element, nom::Err<nom::error::Error<String>>>>, IoError> {
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
