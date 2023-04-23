use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{alpha1},
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
enum Namespace<'ns> {
    Prefix(&'ns str),
    URI(&'ns str),
}

#[derive(Clone, Debug, PartialEq)]
enum Tag<'tag, 'ns> {
    Open(&'tag str),
    Close(&'tag str),
    // NS(Prefix, Tag::Open or Tag::Close)
    NS(Namespace<'ns>, Box<Tag<'tag, 'ns>>),
}

#[derive(Clone, Debug, PartialEq)]
enum Element<'tag, 'ns, 'elem> {
    Node(
        Tag<'tag, 'ns>,
        Box<Element<'tag, 'ns, 'elem>>,
        Tag<'tag, 'ns>,
    ),
    Content(&'elem str),
    Nested(Vec<Element<'tag, 'ns, 'elem>>),
}

// Parse XML tags (both opening and closing)
fn parse_tag(input: &str) -> IResult<&str, Tag> {
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

fn parse_content(input: &str) -> IResult<&str, &str> {
    take_until("<")(input)
}

// Modify parse_recursive function to handle Element::Node structure correctly
fn parse_recursive(input: &str) -> IResult<&str, Element, nom::error::Error<&str>> {
    let (input, open_tag) = parse_tag(input)?;
    let (input, children) = many0(parse_recursive)(input)?;
    let (input, content) = parse_content(input)?;

    let (input, close_tag) = parse_tag(input)?;

    let node = match (&open_tag, &close_tag) {
        (Tag::Open(open_name), Tag::Close(close_name))
        | (Tag::NS(Namespace::Prefix(open_name), _), Tag::NS(Namespace::Prefix(close_name), _))
            if open_name == close_name =>
        {
            let child_element = if !content.is_empty() {
                Element::Content(content)
            } else if children.len() == 1 {
                children.into_iter().next().unwrap()
            } else {
                Element::Nested(children)
            };

            Ok((
                input,
                Element::Node(open_tag, Box::new(child_element), close_tag),
            ))
        }
        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Verify,
        ))),
    };

    node
}

fn main() {
    let input = "<root><inner_tag1>inner_tag1 content</inner_tag1><inner_tag2>2</inner_tag2><tst:inner_tag3>3</tst:inner_tag3><tst:inner_tag4><inner_inner_tag1>inner_inner_tag1 content</inner_inner_tag1><header>header contents></header><inner_inner_tag1>inner_inner_tag1 content2</inner_inner_tag1></tst:inner_tag4></root>";
    let (tail, result) = parse_recursive(input).unwrap();
    println!("result: {:#?}", result);
    println!("tail: {:?}", tail);
}
