use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::alpha1,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, preceded, tuple},
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
                    // Look for an optional namespace prefix and the local name of the tag.
                    tuple((
                        opt(preceded(alpha1, tag(":"))),
                        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
                    )),
                    tag(">"),
                ),
                // Use the create_ns_tag helper function to create an opening tag (Tag::Open).
                |(prefix, local_name)| create_ns_tag(prefix, local_name, Tag::Open),
            ),
            // Parse closing tags
            map(
                delimited(
                    tag("</"),
                    // Look for an optional namespace prefix and the local name of the tag.
                    tuple((
                        opt(preceded(alpha1, tag(":"))),
                        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
                    )),
                    tag(">"),
                ),
                // Use the create_ns_tag helper function to create a closing tag (Tag::Close).
                |(prefix, local_name)| create_ns_tag(prefix, local_name, Tag::Close),
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

        match (&open_tag, &close_tag) {
            (Tag::Open(open_name), Tag::Close(close_name))
            | (
                Tag::NS(Namespace::Prefix(open_name), _),
                Tag::NS(Namespace::Prefix(close_name), _),
            ) if open_name == close_name => {
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
        }
    }
}

fn main() {
    let input = "<root><inner_tag1>inner_tag1 content</inner_tag1><inner_tag2>2</inner_tag2><tst:inner_tag3>3</tst:inner_tag3><tst:inner_tag4><inner_inner_tag1>inner_inner_tag1 content</inner_inner_tag1><header>header contents></header><inner_inner_tag1>inner_inner_tag1 content2</inner_inner_tag1></tst:inner_tag4></root>";
    let (tail, result) = Element::parse_xml_str(input).unwrap();
    println!("result: {:#?}", result);
    println!("tail: {:?}", tail);
}
