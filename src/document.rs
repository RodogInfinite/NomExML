use std::borrow::Cow;

use nom::{

    bytes::complete::{tag, take_until, take_while1},
    character::complete::{alpha1},
    combinator::{ map, opt, recognize},
    multi::many0,
    sequence::pair,
    IResult,
};
use crate::{tag::{Namespace, Tag}, decode::decode_entities, utils::parse_with_whitespace, Elements};
use crate::declaration::Declaration;

// TODO: think about processing instructions: https://www.w3.org/TR/2008/REC-xml-20081126/#sec-pi
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
    pub fn parse_tag_and_namespace(
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
        println!("Content: {:?}", content);
        if content.is_empty() {
            println!("Empty content");
            Ok((tail, None))
        } else {
            let (_, content) = decode_entities(content)?;
            println!("Decoded content: {:?}", content);
            Ok((tail, Some(content)))
            //Ok((tail, Some(Cow::Borrowed(content))))
        }
    }

    pub fn parse_xml_str(input: &'a str) -> IResult<&'a str, Document<'a>> {
        let (input, declaration) = Self::parse_declaration(input)?;
        let (input, start_tag) = Tag::parse_start_tag(input)?;
        let (input, children) = Self::parse_children(input)?;
        let (input, content) = Self::parse_content(input)?;
        let (input, end_tag) = Tag::parse_end_tag(input)?;

        Self::construct_document(input, declaration, start_tag, children, content, end_tag)
    }

    fn parse_declaration(input: &'a str) -> IResult<&'a str, Option<Declaration<'a>>> {
        parse_with_whitespace(input, opt(Declaration::parse))
    }

    fn parse_children(input: &'a str) -> IResult<&'a str, Vec<Document<'a>>> {
        parse_with_whitespace(input, many0(Self::parse_xml_str))
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

    pub fn extract_content(&'a self) -> Option<&'a str> {
        match self {
            Document::Element(_, content, _) => content.extract_content(),
            Document::Content(Some(content)) => Some(content),
            _ => None,
        }
    }
}

fn determine_child_document<'a>(
    content: Option<Cow<'a, str>>,
    children: Vec<Document<'a>>,
) -> Result<Document<'a>, &'static str> {
    if let Some(content) = content {
        Ok(Document::Content(Some(Cow::Owned(
            content.as_ref().to_string(),
        ))))
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