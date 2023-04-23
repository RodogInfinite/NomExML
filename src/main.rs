use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{alpha1},
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, pair, tuple, preceded},
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
    NS(Namespace<'ns>, Box<Tag<'tag, 'ns>>),  // NS(Prefix, Tag::Open | Tag::Close)
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

// A helper function to create a namespaced tag.
fn create_ns_tag<'a>(prefix: Option<&'a str>, local_name: &'a str, tag_type: fn(&'a str) -> Tag<'a, 'a>) -> Tag<'a, 'a> {
    match prefix {
        // If a prefix is present, create a Tag::NS with the Namespace::Prefix and the tag created by calling the tag_type function.
        Some(p) => Tag::NS(Namespace::Prefix(p), Box::new(tag_type(local_name))),
        // If no prefix is present, create the tag directly by calling the tag_type function.
        None => tag_type(local_name),
    }
}

// The parse_tag function is responsible for parsing XML tags (opening and closing).
fn parse_tag<'a>(input: &'a str) -> IResult<&'a str, Tag<'a, 'a>> {
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

fn parse_content(input: &str) -> IResult<&str, &str> {
    take_until("</")(input)
}

fn parse_recursive(input: &str) -> IResult<&str, Element, nom::error::Error<&str>> {
    // Parse the opening tag of the element
    let (input, open_tag) = parse_tag(input)?;
    
    // Recursively parse the children of the current element using many0 combinator
    // many0 applies the parse_recursive function zero or more times until it fails
    let (input, children) = many0(parse_recursive)(input)?;
    
    // Parse the content of the current element (text between tags)
    let (input, content) = parse_content(input)?;

    // Parse the closing tag of the element
    let (input, close_tag) = parse_tag(input)?;

    // Match the opening and closing tags to ensure they are of the same type (Open and Close or NS with same prefixes)
    let node = match (&open_tag, &close_tag) {
        // Check if the opening and closing tags are either both Open/Close or both NS with the same prefix
        (Tag::Open(open_name), Tag::Close(close_name))
        | (Tag::NS(Namespace::Prefix(open_name), _), Tag::NS(Namespace::Prefix(close_name), _))
            if open_name == close_name =>
        {
            // Determine the child element depending on the content and the number of children
            let child_element = if !content.is_empty() {
                // If content is not empty, create an Element::Content variant
                Element::Content(content)
            } else if children.len() == 1 {
                // If there is only one child, use that child as the nested element
                children.into_iter().next().unwrap()
            } else {
                // Otherwise, create an Element::Nested variant with all the children
                Element::Nested(children)
            };

            // Return a new Element::Node variant with the opening tag, the child element, and the closing tag
            Ok((
                input,
                Element::Node(open_tag, Box::new(child_element), close_tag),
            ))
        }
        // If the opening and closing tags don't match or are invalid, return an error with the Verify ErrorKind
        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Verify,
        ))),
    };

    // Return the resulting node
    node
}


fn main() {
    let input = "<root><inner_tag1>inner_tag1 content</inner_tag1><inner_tag2>2</inner_tag2><tst:inner_tag3>3</tst:inner_tag3><tst:inner_tag4><inner_inner_tag1>inner_inner_tag1 content</inner_inner_tag1><header>header contents></header><inner_inner_tag1>inner_inner_tag1 content2</inner_inner_tag1></tst:inner_tag4></root>";
    let (tail, result) = parse_recursive(input).unwrap();
    println!("result: {:#?}", result);
    println!("tail: {:?}", tail);
}
