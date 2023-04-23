use nom::{
    bytes::complete::{tag, take_until, take_till, take_while1},
    character::complete::{alphanumeric1, alpha1},
    multi::many0,
    sequence::{delimited, tuple, preceded, pair},
    IResult, branch::alt, combinator::{map, opt, recognize},
};

#[derive(Clone, Debug, PartialEq)]
enum Namespace<'ns> {
    Prefix(&'ns str),
    URI(&'ns str),
}

#[derive(Clone, Debug, PartialEq)]
enum Tag<'tag,'ns> {
    Open(&'tag str),
    Close(&'tag str),
    // NS(Prefix, Tag::Open or Tag::Close)
    NS(Namespace<'ns>, Box<Tag<'tag,'ns>>)
}

#[derive(Clone,Debug, PartialEq)]
enum Element<'tag,'ns, 'elem> {
    Node(Tag<'tag,'ns>, Box<Element<'tag,'ns,'elem>>, Tag<'tag,'ns>),
    Content(&'elem str),
    Nested(Vec<Element<'tag,'ns,'elem>>)
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
                    opt(pair(
                        alpha1,
                        tag(":"),
                    )),
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
            delimited(tag("</"), take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == ':'), tag(">")),
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
fn parse_recursive(mut input: &str) -> IResult<&str, Element> {
    println!("Entering parse_recursive with input: {:?}", input);

    let (tail, open_tag) = parse_tag(input)?;
    println!("Parsed open tag: {:?}", open_tag);
    let mut children = Vec::new();
    input = tail;

    while let Ok((tail, child)) = parse_recursive(input) {
        println!("Parsed child: {:?}", child);
        input = tail;
        children.push(child);
    }

    let (input, content) = parse_content(input)?;
    println!("Parsed content: {:?}", content);

    if !content.is_empty() {
        children.push(Element::Content(content));
    }

    let (input, close_tag) = parse_tag(input)?;
    println!("Parsed close tag: {:?}", close_tag);

    match (&open_tag, &close_tag) {
        (Tag::Open(open_name), Tag::Close(close_name)) | (Tag::NS(Namespace::Prefix(open_name), _), Tag::NS(Namespace::Prefix(close_name), _)) => {
            if open_name == close_name {
                let node = if children.len() == 1 {
                    Element::Node(open_tag, Box::new(children.remove(0)), close_tag)
                } else {
                    Element::Node(open_tag, Box::new(Element::Nested(children)), close_tag)
                };
                println!("Parsed node: {:?}", node);
                Ok((input, node))
            } else {
                let content = Element::Content("Mismatched tags");
                println!("Parsed content: {:?}", content);
                Ok((input, content))
            }
        }
        _ => {
            let content = Element::Content("Invalid tag state");
            println!("Parsed content: {:?}", content);
            Ok((input, content))
        }
    }
}









fn main() {
    let input = "<root><inner_tag1>inner_tag1 content</inner_tag1><inner_tag2>2</inner_tag2><tst:inner_tag3>3</tst:inner_tag3></root>";
    let (tail, result) = parse_recursive(input).unwrap();
    println!("result: {:?}", result);
    println!("tail: {:?}", tail);

    // assert_eq!(result,Element::Node(Tag::Open("root"), vec![
    //     Element::Node(Tag::Open("inner_tag1"), vec![
    //         Element::Content("inner_tag1 content"), 
    //     ], Tag::Close("inner_tag1")),
    //     Element::Node(Tag::Open("inner_tag2"), vec![
    //         Element::Content("2"), 
    //     ], Tag::Close("inner_tag2")),
    //     Element::Node(Tag::Open("inner_tag3"), vec![
    //         Element::NS(Namespace::Prefix("tst")),
    //         Element::Content("3"), 
    //         Element::NS(Namespace::Prefix("tst"))
    //     ], Tag::Close("inner_tag3")),
    //     ], Tag::Close("root"))
    // )
}
