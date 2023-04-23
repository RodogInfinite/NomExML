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
    LocalName(&'ns str),
    URI(&'ns str),
}

#[derive(Clone, Debug, PartialEq)]
enum Tag<'tag> {
    Open(&'tag str),
    Close(&'tag str),
    NS(Namespace<'tag>)
}

#[derive(Debug, PartialEq)]
enum Element<'tag, 'a> {
    Node(Tag<'tag>, Vec<Element<'tag, 'a>>, Tag<'tag>),
    Content(&'a str),
}


fn parse_processing_instructions(input: &str) -> IResult<&str, &str> {
    delimited(tag("<?"), take_until("?>"), tag("?>"))(input)
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
                    Tag::NS(Namespace::Prefix(prefix))
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
                if let (Some(_prefix), Some(local_name)) = (parts.next(), parts.next()) {
                    Tag::Close(local_name)
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

fn parse_recursive(input: &str) -> IResult<&str, Vec<Element>> {
    let mut elements = vec![];

    let (mut input, _) = opt(parse_processing_instructions)(input).unwrap_or((input, None));

    loop {
        println!("Current input: {:?}", input);

        if let Ok((tail, open_tag)) = parse_tag(input) {
            println!("Parsed tag: {:?}", open_tag);
            input = tail;

            let (tail, children) = parse_recursive(input).unwrap_or((tail, vec![]));
            println!("Parsed children: {:?}", children);

            if let Ok((tail, close_tag)) = parse_tag(tail) {
                println!("Parsed closing tag: {:?}", close_tag);
                elements.push(Element::Node(open_tag.clone(), children, close_tag));
                input = tail;
            } else {
                println!("Error parsing closing tag");
                break;
            }
        } else if let Ok((tail, content)) = parse_content(input) {
            println!("Parsed content: {:?}", content);
            input = tail;

            elements.push(Element::Content(content));
        } else {
            println!("Reached the end of the input");
            break;
        }
    }

    Ok((input, elements))
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
