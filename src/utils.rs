use std::borrow::Cow;

use nom::{
    bytes::complete::take_while1, character::complete::multispace0, combinator::map, multi::many0,
    sequence::tuple, IResult,
};

pub trait Parse<'a>: Sized {
    //fn parse(&mut self, input: &'a str) -> IResult<&'a str, Self>;

    fn parse_name(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let parser = take_while1(|c: char| c.is_alphanumeric());
        map(parser, Cow::Borrowed)(input)
    }

    fn parse_with_whitespace<F, O>(input: &'a str, mut parser: F) -> IResult<&'a str, O>
    where
        F: FnMut(&'a str) -> IResult<&'a str, O>,
    {
        let (input, _) = multispace0(input)?;
        let (input, result) = parser(input)?;
        let (input, _) = multispace0(input)?;
        Ok((input, result))
    }
}
