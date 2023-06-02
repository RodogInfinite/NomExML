use std::borrow::Cow;

use crate::parse::Parse;

use crate::processing_instruction::ProcessingInstruction;
use crate::prolog::doctype::DocType;
use crate::prolog::xmldecl::XmlDecl;
use crate::reference::Reference;
use crate::{
    decode::decode_entities,
    tag::Tag,
    //utils::parse_with_whitespace,
    Elements,
};
use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::combinator::{not, peek, verify};
use nom::multi::{many1, many_till};
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
pub enum MiscState {
    BeforeDoctype,
    AfterDoctype,
}

#[derive(Clone, PartialEq)]
pub struct Misc<'a> {
    pub content: Box<Document<'a>>, // Document::Comment | Document::ProcessingInstruction>
    pub state: MiscState,
}

impl<'a> Parse<'a> for Misc<'a> {}

impl<'a> Misc<'a> {
    //[27] Misc ::= Comment | PI | S
    fn parse(input: &'a str, state: MiscState) -> IResult<&'a str, Self> {
        let mut input_remaining = input;
        let mut content_vec: Vec<Document<'a>> = vec![];

        loop {
            let parse_result = alt((
                Document::parse_comment,
                map(ProcessingInstruction::parse, |pi| {
                    Document::ProcessingInstruction(pi)
                }),
                map(Self::parse_multispace1, |_| Document::Empty),
            ))(input_remaining);

            match parse_result {
                Ok((remaining, document)) => {
                    match document {
                        Document::Empty => {} // Don't add Document::Empty types to content_vec
                        _ => content_vec.push(document),
                    }
                    input_remaining = remaining;
                }
                Err(nom::Err::Incomplete(_)) => continue,
                Err(_) => {
                    if !content_vec.is_empty() {
                        break;
                    } else {
                        return Err(nom::Err::Error(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Many0,
                        )));
                    }
                }
            }
        }

        let content = Box::new(Document::Nested(content_vec));

        Ok((input_remaining, Misc { content, state }))
    }
}

#[derive(Clone, PartialEq)]
pub enum Document<'a> {
    Prolog {
        xml_decl: Option<XmlDecl<'a>>,
        misc: Option<Vec<Misc<'a>>>,
        doc_type: Option<DocType<'a>>,
    },
    Element(Tag<'a>, Box<Document<'a>>, Tag<'a>),
    Content(Option<Cow<'a, str>>),
    Nested(Vec<Document<'a>>),
    Empty,
    ProcessingInstruction(ProcessingInstruction<'a>),
    Comment(Cow<'a, str>),
    CDATA(Cow<'a, str>),
}

impl<'a> Parse<'a> for Document<'a> {}

impl<'a> Document<'a> {
    //[22 prolog ::= XMLDecl? Misc* (doctypedecl Misc*)?
    pub fn parse_prolog(input: &'a str) -> IResult<&'a str, Document<'a>> {
        let (input, xml_decl) = opt(XmlDecl::parse)(input)?;
        let (input, misc_before) =
            opt(|input| Misc::parse(input, MiscState::BeforeDoctype))(input)?;
        let (input, doc_type) = opt(DocType::parse)(input)?;
        let (input, misc_after) = match &doc_type {
            Some(_) => opt(|input| Misc::parse(input, MiscState::AfterDoctype))(input)?,
            None => (input, None),
        };

        let miscs: Vec<Option<Misc<'a>>> = vec![misc_before, misc_after];
        let miscs: Vec<Misc<'a>> = miscs.into_iter().flatten().collect();
        let misc = if miscs.is_empty() { None } else { Some(miscs) };

        Ok((
            input,
            Document::Prolog {
                xml_decl,
                misc,
                doc_type,
            },
        ))
    }

    //[18] CDSect ::= CDStart CData CDEnd
    //[19] CDStart ::= '<![CDATA['
    //[20] CData ::= (Char* - (Char* ']]>' Char*))
    fn parse_char_data(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, data) = take_till(|c: char| c == '<' || c == '&')(input)?;
        let (input, _) = not(peek(tag("]]>")))(input)?;
        Ok((input, Cow::Borrowed(data)))
    }

    //[21] CDEnd ::= ']]>'
    fn parse_cdata_section(input: &'a str) -> IResult<&'a str, Document<'a>> {
        let (input, _) = tag("<![CDATA[")(input)?;
        let (input, cdata_content) = Self::parse_char_data(input)?;
        let cdata_string: String = cdata_content.to_string();
        let (input, _) = tag("]]>")(input)?;
        Ok((input, Document::CDATA(Cow::Owned(cdata_string))))
    }

    // [39] element	::= EmptyElemTag | STag content ETag
    pub fn parse_element(input: &'a str) -> IResult<&'a str, Document<'a>> {
        alt((
            map(Tag::parse_empty_element_tag, |tag| {
                Document::Element(tag.clone(), Box::new(Document::Empty), tag.clone())
            }),
            map(
                tuple((
                    Tag::parse_start_tag,
                    Self::parse_content,
                    Tag::parse_end_tag,
                )),
                |(start_tag, content, end_tag)| {
                    Document::Element(start_tag, Box::new(content), end_tag)
                },
            ),
        ))(input)
    }
    // [43] content	::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
    fn parse_content(input: &'a str) -> IResult<&'a str, Document<'a>> {
        let (input, (maybe_chardata, elements)) = tuple((
            opt(Self::parse_char_data),
            many0(pair(
                alt((
                    Self::parse_element,
                    map(Reference::parse, |reference| match reference {
                        Reference::EntityRef(entity) => Document::Content(Some(entity)),
                        Reference::CharRef { value, .. } => Document::Content(Some(value)),
                    }),
                    Self::parse_cdata_section,
                    map(
                        ProcessingInstruction::parse,
                        Document::ProcessingInstruction,
                    ),
                    Self::parse_comment,
                )),
                opt(Self::parse_char_data),
            )),
        ))(input)?;

        let content = elements
            .into_iter()
            .flat_map(|(doc, maybe_chardata)| {
                let mut vec = Vec::new();
                vec.push(doc);
                if let Some(chardata) = maybe_chardata {
                    vec.push(Document::Content(Some(chardata)));
                }
                vec
            })
            .collect();

        Ok((
            input,
            Document::Nested(match maybe_chardata {
                Some(chardata) => {
                    let mut vec = Vec::new();
                    vec.push(Document::Content(Some(chardata)));
                    vec.extend(content);
                    vec
                }
                None => content,
            }),
        ))
    }

    // [15] Comment ::= '<!--' ((Char - '-') | ('-' (Char - '-')))* '-->'
    pub fn parse_comment(input: &'a str) -> IResult<&'a str, Document<'a>> {
        let (input, _) = tag("<!--")(input)?;
        let (input, (comment_content, _)) =
            many_till(verify(Self::parse_char, |&c| c != '-'), tag("-->"))(input)?;
        let comment_string: String = comment_content.into_iter().collect();
        let (input, _) = tag("-->")(input)?;

        Ok((input, Document::Comment(Cow::Owned(comment_string))))
    }

    pub fn parse_xml_str(input: &'a str) -> IResult<&'a str, Document<'a>> {
        let (input, prolog) = opt(Self::parse_prolog)(input)?;
        let (input, start_tag) =
            alt((Tag::parse_qualified_start_tag, Tag::parse_start_tag))(input)?;
        let (input, children) = Self::parse_children(input)?;
        let (input, content) = Self::parse_content(input)?;
        let (input, end_tag) = alt((Tag::parse_qualified_end_tag, Tag::parse_end_tag))(input)?;

        Self::construct_document(input, prolog, start_tag, children, content, end_tag)
    }

    fn parse_children(input: &'a str) -> IResult<&'a str, Vec<Document<'a>>> {
        let (input, _) = Self::parse_multispace0(input)?;
        many0(Self::parse_xml_str)(input)
    }

    fn construct_document_with_prolog(
        prolog: Option<Document<'a>>,
        start_tag: &Tag<'a>,
        child_document: Document<'a>,
        end_tag: &Tag<'a>,
    ) -> Document<'a> {
        let element =
            Document::Element(start_tag.clone(), Box::new(child_document), end_tag.clone());
        match prolog {
            Some(prolog) => Document::Nested(vec![prolog, element]),
            None => element,
        }
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
        prolog: Option<Document<'a>>,
        start_tag: Tag<'a>,
        children: Vec<Document<'a>>,
        content: Document<'a>,
        end_tag: Tag<'a>,
    ) -> IResult<&'a str, Document<'a>> {
        match (&start_tag, &end_tag) {
            (
                Tag {
                    name: start_name, ..
                },
                Tag { name: end_name, .. },
            ) if start_name == end_name => {
                let child_document = determine_child_document(content, children).map_err(|e| {
                    nom::Err::Failure(nom::error::Error::new(e, nom::error::ErrorKind::Verify))
                })?;
                let document = match prolog {
                    Some(prolog) => Self::construct_document_with_prolog(
                        Some(prolog),
                        &start_tag,
                        child_document,
                        &end_tag,
                    ),
                    None => Self::construct_element(&start_tag, child_document, &end_tag),
                };
                Ok((input, document))
            }
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            ))),
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
        Document::ProcessingInstruction(PI) => Ok(Document::ProcessingInstruction(PI)),
        Document::Nested(docs) => Ok(Document::Nested(docs)), // propagate nested documents up
        Document::CDATA(cow) => Ok(Document::CDATA(cow)),
        Document::Comment(cow) => Ok(Document::Comment(cow)),
        _ => Err("Invalid content type in determine_child_document"),
    }
}
