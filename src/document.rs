use std::borrow::Cow;

use crate::declaration::Declaration;
use crate::utils::Parse;
use crate::{
    decode::decode_entities,
    tag::{Namespace, Tag},
    //utils::parse_with_whitespace,
    Elements,
};
use nom::branch::alt;
use nom::character::complete::space0;
use nom::multi::many1;
use nom::sequence::{delimited, tuple};
use nom::{
    bytes::complete::{tag, take_until, take_while1},
    character::complete::alpha1,
    combinator::{map, opt},
    multi::many0,
    sequence::pair,
    IResult,
};

#[derive(Clone, PartialEq)]
pub enum Document<'a> {
    Declaration(Option<Declaration<'a>>),
    Element(Tag<'a>, Box<Document<'a>>, Tag<'a>),
    Content(Option<Cow<'a, str>>),
    Nested(Vec<Document<'a>>),
    Empty,
    Comment(Option<Cow<'a, str>>),
    ProcessingInstruction {
        target: Cow<'a, str>,
        data: Option<Cow<'a, str>>,
    },
    CDATA(Cow<'a, str>), // CDATA(Document::Content)
}
impl<'a> Document<'a> {
    pub fn parse_tag_and_namespace(
        input: &'a str,
    ) -> IResult<&'a str, (Cow<'a, str>, Option<Namespace<'a>>)> {
        map(
            pair(
                // Look for an optional namespace prefix
                opt(pair(alpha1, tag(":"))),
                take_while1(|c: char| c.is_alphanumeric() || c == '_'),
            ),
            |(prefix, local_name): (Option<(&str, &str)>, &str)| {
                if let Some((prefix, _)) = prefix {
                    (
                        Cow::Borrowed(local_name),
                        Some(Namespace {
                            declaration: None,
                            prefix: Cow::Borrowed(prefix),
                            uri: None,
                        }),
                    )
                } else {
                    (Cow::Borrowed(local_name), None)
                }
            },
        )(input)
    }

    fn parse_content(input: &'a str) -> IResult<&'a str, Document<'a>> {
        alt((
            Self::parse_cdata_section,
            Self::parse_comment,
            |input: &'a str| {
                let (input, docs) = many1(Self::parse_processing_instruction)(input)?;
                if docs.len() > 1 {
                    Ok((input, Document::Nested(docs)))
                } else {
                    Ok((input, docs.into_iter().next().unwrap()))
                }
            },
            |input: &'a str| {
                let (tail, content) = take_until("</")(input)?;
                if content.is_empty() {
                    Ok((tail, Document::Empty))
                } else {
                    let (_, content) = decode_entities(content)?;
                    Ok((tail, Document::Content(Some(content))))
                }
            },
        ))(input)
    }

    fn parse_processing_instruction(input: &'a str) -> IResult<&'a str, Document<'a>> {
        let (input, (target, data)) = delimited(
            delimited(tag("<"), space0, tag("?")),
            tuple((alpha1, opt(take_until("?>")))),
            tag("?>"),
        )(input)?;
        println!("input: {input:?}");
        if target.eq_ignore_ascii_case("xml") {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }

        let data = data
            .map(|d| d.trim())
            .filter(|d| !d.is_empty())
            .map_or(None, |d| Some(Cow::Borrowed(d)));
        println!("DATA: {data:?}");
        Ok((
            input,
            Document::ProcessingInstruction {
                target: Cow::Borrowed(target),
                data,
            },
        ))
    }

    fn parse_cdata_section(input: &'a str) -> IResult<&'a str, Document<'a>> {
        let (input, content) = delimited(tag("<![CDATA["), take_until("]]>"), tag("]]>"))(input)?;

        let content = if content.is_empty() {
            Document::Empty
        } else {
            Document::CDATA(Cow::Borrowed(content))
        };

        Ok((input, content))
    }

    fn parse_comment(input: &'a str) -> IResult<&'a str, Document<'a>> {
        map(
            delimited(tag("<!--"), take_until("-->"), tag("-->")),
            |comment: &'a str| Document::Comment(Some(Cow::Borrowed(comment))),
        )(input)
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
        content: Document<'a>,
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
    content: Document<'a>,
    children: Vec<Document<'a>>,
) -> Result<Document<'a>, &'static str> {
    match content {
        Document::Empty => {
            if children.is_empty() {
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
        Document::Content(Some(cow)) => Ok(Document::Content(Some(Cow::Owned(cow.into_owned())))),
        Document::ProcessingInstruction { target, data } => {
            Ok(Document::ProcessingInstruction { target, data })
        }
        Document::Nested(docs) => Ok(Document::Nested(docs)), // propagate nested documents up
        Document::CDATA(cow) => Ok(Document::CDATA(cow)),
        Document::Comment(cow) => Ok(Document::Comment(cow)),
        _ => Err("Invalid content type in determine_child_document"),
    }
}

impl<'a> Parse<'a> for Document<'a> {}
