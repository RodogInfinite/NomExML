mod debug;

use std::{fs::File, io::Read};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::alpha1,
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};
use std::io::BufReader;

#[derive(Clone, Debug, PartialEq)]
pub enum Namespace<'ns> {
    Prefix(&'ns str),
    URI(&'ns str),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tag<'a> {
    Open(&'a str),
    Close(&'a str),
    NS(Namespace<'a>, Box<Tag<'a>>), // NS(Prefix, Tag::Open | Tag::Close)
}

impl<'a> Tag<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
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
                        Tag::NS(Namespace::Prefix(prefix), Box::new(Tag::Open(local_name)))
                    } else {
                        Tag::Open(tag_name)
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
                        Tag::NS(Namespace::Prefix(prefix), Box::new(Tag::Close(local_name)))
                    } else {
                        Tag::Close(tag_name)
                    }
                },
            ),
        ))(input)
    }
}

#[derive(Clone, PartialEq)]
pub enum Element<'a> {
    Node(Tag<'a>, Box<Element<'a>>, Tag<'a>),
    Content(&'a str),
    Nested(Vec<Element<'a>>),
}

impl<'a> Element<'a> {
    fn parse_content(input: &'a str) -> IResult<&'a str, &'a str> {
        take_until("</")(input)
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
        let (input, open_tag) = Self::parse_element_with_whitespace(input, Tag::parse)?;
        let (input, children) =
            Self::parse_element_with_whitespace(input, |i| many0(Self::parse_xml_str)(i))?;
        let (input, content) = Self::parse_element_with_whitespace(input, Self::parse_content)?;
        let (input, close_tag) = Self::parse_element_with_whitespace(input, Tag::parse)?;

        if tags_match(&open_tag, &close_tag) {
            let child_element = determine_child_element(&content, children);
            Ok((
                input,
                Element::Node(open_tag, Box::new(child_element), close_tag),
            ))
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )))
        }
    }

    pub fn parse_file(file: &mut File, buffer: &'a mut String) -> IResult<&'a str, Self> {
        let mut reader = BufReader::new(file);

        // Read the entire file content into the buffer
        if let Err(e) = reader.read_to_string(buffer) {
            return Err(nom::Err::Failure(nom::error::Error::new(
                "",
                nom::error::ErrorKind::Verify,
            )));
        }

        // Parse the XML string using the parse_xml_str function
        Self::parse_xml_str(buffer)
    }
}

// Helper function to determine the child element type
fn determine_child_element<'a>(content: &'a str, children: Vec<Element<'a>>) -> Element<'a> {
    if !content.is_empty() {
        Element::Content(content)
    } else if children.len() == 1 {
        children.into_iter().next().unwrap()
    } else {
        Element::Nested(children)
    }
}
// Helper function to verify if open and close tags match
fn tags_match(open_tag: &Tag, close_tag: &Tag) -> bool {
    match (open_tag, close_tag) {
        (Tag::Open(open_name), Tag::Close(close_name))
        | (Tag::NS(Namespace::Prefix(open_name), _), Tag::NS(Namespace::Prefix(close_name), _)) => {
            open_name == close_name
        }
        _ => false,
    }
}
