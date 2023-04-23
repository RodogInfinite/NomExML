use nom::{
    bytes::complete::{tag, take_until, take_till, take_while1},
    character::complete::{alphanumeric1, alpha1},
    multi::many0,
    sequence::{delimited, tuple, preceded, pair},
    IResult, branch::alt, combinator::{map, opt},
};

#[derive(Clone,Debug,PartialEq)]
enum Namespace<'a> {
    Prefix(&'a str),
    LocalName(&'a str),
    URI(&'a str),
}

#[derive(Clone,Debug,PartialEq)]
enum Tag<'a> {
    Open(&'a str),
    Close(&'a str),
    NS(Namespace<'a>)
}

#[derive(Debug,PartialEq)]
enum Element<'a> {
    Node(Tag<'a>, Vec<Element<'a>>, Tag<'a>),
    Content(&'a str),
    Tag(Tag<'a>)
}


fn parse_processing_instructions(input: &str) -> IResult<&str, &str> {
    delimited(tag("<?"), take_until("?>"), tag("?>"))(input)
}

fn parse_tag(input: &str) -> IResult<&str, Tag> {
    alt((
        map(
            delimited(
                tag("<"),
                pair(
                    alpha1,
                    opt(preceded(
                        tag(":"),
                        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
                    )),
                ),
                tag(">"),
            ),
            |(prefix, opt_tag_name)| match opt_tag_name {
                Some(tag_name) => Tag::NS(Namespace::Prefix(prefix)),
                None => Tag::Open(prefix),
            },
        ),
        map(delimited(tag("</"), take_until(">"), tag(">")), |tag_name| {
            Tag::Close(tag_name)
        }),
        map(delimited(tag("<"), take_until(">"), tag(">")), |tag_name| {
            Tag::Open(tag_name)
        }),
    ))(input)
}

fn parse_content(input: &str) -> IResult<&str, &str> {
    take_until("<")(input)
}

fn parse_recursive(input: &str) -> IResult<&str, Vec<Element>> {
    many0(alt((
        map(parse_tag, |tag| {
            Element::Node(tag.clone(), vec![], tag)
        }),
        map(tuple((parse_tag, parse_recursive, parse_tag)), |(open_tag, nodes, close_tag)| {
            match (open_tag, close_tag) {
                (Tag::Open(open), Tag::Close(close)) => {
                    Element::Node(Tag::Open(open), nodes, Tag::Close(close))
                }
                _ => unreachable!(),
            }
        }),
        map(parse_content, |content| {
            Element::Content(content)
        }),
    )))(input)
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
