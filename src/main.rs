use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::alpha1,
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, preceded, tuple, pair},
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
enum Namespace<'ns> {
    Prefix(&'ns str),
    URI(&'ns str),
}

#[derive(Clone, Debug, PartialEq)]
enum Tag<'a> {
    Open(&'a str),
    Close(&'a str),
    NS(Namespace<'a>, Box<Tag<'a>>), // NS(Prefix, Tag::Open | Tag::Close)
}

fn create_ns_tag<'a>(
    prefix: Option<&'a str>,
    local_name: &'a str,
    tag_type: fn(&'a str) -> Tag<'a>,
) -> Tag<'a> {
    match prefix {
        Some(p) => Tag::NS(Namespace::Prefix(p), Box::new(tag_type(local_name))),
        None => tag_type(local_name),
    }
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

#[derive(Clone, Debug, PartialEq)]
enum Element<'a> {
    Node(Tag<'a>, Box<Element<'a>>, Tag<'a>),
    Content(&'a str),
    Nested(Vec<Element<'a>>),
}

impl<'a> Element<'a> {
    fn parse_content(input: &'a str) -> IResult<&'a str, &'a str> {
        take_until("</")(input)
    }

    fn parse_xml_str(input: &'a str) -> IResult<&'a str, Self> {
        let (input, open_tag) = Tag::parse(input)?;
        let (input, children) = many0(Self::parse_xml_str)(input)?;
        let (input, content) = Self::parse_content(input)?;
        let (input, close_tag) = Tag::parse(input)?;
    
        if tags_match(&open_tag, &close_tag) {
            let child_element = determine_child_element(&content, children);
            Ok((input, Element::Node(open_tag, Box::new(child_element), close_tag)))
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )))
        }
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
        | (
            Tag::NS(Namespace::Prefix(open_name), _),
            Tag::NS(Namespace::Prefix(close_name), _),
        ) => open_name == close_name,
        _ => false,
    }
}


fn main() {
    let input = "<root><inner_tag1>inner_tag1 content</inner_tag1><inner_tag2>2</inner_tag2><tst:inner_tag3>3</tst:inner_tag3><tst:inner_tag4><inner_inner_tag1>inner_inner_tag1 content</inner_inner_tag1><header>header contents></header><inner_inner_tag1>inner_inner_tag1 content2</inner_inner_tag1></tst:inner_tag4></root>";
    let (tail, result) = Element::parse_xml_str(input).unwrap();
    println!("result: {:#?}", result);
    println!("tail: {:?}", tail);
}
