use nom::{character::complete::multispace0, IResult};

pub fn parse_with_whitespace<'a, F, O>(input: &'a str, mut parser: F) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    let (input, _) = multispace0(input)?;
    let (input, result) = parser(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, result))
}
