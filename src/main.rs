use nom::{
    bytes::complete::{tag, take_until, take_till},
    character::complete::alphanumeric1,
    multi::many0,
    sequence::{delimited, tuple, preceded, pair},
    IResult, branch::alt, combinator::{map},
};

#[derive(Debug)]
enum Namespace<'a> {
    Prefix(&'a str),
    LocalName(&'a str),
    URI(&'a str),
}

#[derive(Debug)]
enum Element<'a> {
    OpenTag(&'a str),
    CloseTag(&'a str),
    Node(&'a str, Vec<Element<'a>>, &'a str),
    Text(&'a str),
    NS(Namespace<'a>)
}

fn parse_processing_instructions(input: &str) -> IResult<&str, &str> {
    delimited(tag("<?"), take_until("?>"), tag("?>"))(input)
}

fn parse_tag(input: &str) -> IResult<&str, Element> {
    alt((
        map(
            delimited(
                alt((tag("</"), tag("<"))),
                pair(
                    alphanumeric1,
                    preceded(
                        tag(":"),
                        take_till(|c| c == '>' || c == '<'),
                    ),
                ),
                tag(">"),
            ),
            |(prefix, tag_name)| {
                Element::NS(Namespace::Prefix(prefix))
            },
        ),
        map(delimited(tag("</"), take_until(">"), tag(">")), |tag_name| {
            Element::CloseTag(tag_name)
        }),
        map(delimited(tag("<"), take_until(">"), tag(">")), |tag_name| {
            Element::OpenTag(tag_name)
        }),
    ))(input)
}

fn parse_content(input: &str) -> IResult<&str, &str> {
    take_until("<")(input)
}

fn parse_recursive(input: &str) -> IResult<&str, Vec<Element>> {
    many0(alt((
        parse_tag,
        map(tuple((parse_tag, parse_recursive, parse_tag)), |(open_tag, nodes, close_tag)| {
            match (open_tag, close_tag) {
                (Element::OpenTag(open), Element::CloseTag(close)) => Element::Node(open, nodes, close),
                _ => unreachable!(),
            }
        }),
        map(parse_content, |content| {
            Element::Text(content)
        }),
    )))(input)
}

fn parse(input: &str) -> IResult<&str, Element> {
    let (tail, mut result) = parse_recursive(input)?;

    let tag_element = match result.remove(0) {
        Element::OpenTag(tag) => tag,
        _ => unreachable!(),
    };

    Ok((tail, Element::Node(tag_element, result, "")))
}

fn main() {
    let input = "<root><inner_tag1>inner_tag1 content</inner_tag1><inner_tag2>2</inner_tag2><tst:inner_tag3>3</tst:inner_tag3></root>";
    let (tail, result) = parse(input).unwrap();
    println!("result: {:?}", result);
    println!("tail: {:?}", tail);
    if let Element::Node(tag, children, _) = result {
        println!("Tag element: {:?}", tag);
        for res in children {
            println!("{:?}", res);
        }
    }
}
