// processing_instruction.rs

use std::borrow::Cow;

use nom::{
    bytes::complete::tag,
    combinator::{opt, peek},
    multi::many_till,
    sequence::preceded,
    IResult,
};

use crate::parse::Parse;

#[derive(Clone, PartialEq)]
pub struct ProcessingInstruction<'a> {
    pub target: Cow<'a, str>,
    pub data: Option<Cow<'a, str>>,
}

impl<'a> Parse<'a> for ProcessingInstruction<'a> {
    // [16] PI ::= '<?' PITarget (S (Char* - (Char* '?>' Char*)))? '?>'
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (input, _) = tag("<?")(input)?;

        let (input, target) = Self::parse_target(input)?;

        let (input, data_chars) = opt(preceded(
            Self::parse_multispace1,
            many_till(Self::parse_char, peek(tag("?>"))),
        ))(input)?;

        let data: Option<String> = data_chars.map(|(chars, _)| chars.into_iter().collect());

        let (input, _) = tag("?>")(input)?;

        Ok((
            input,
            ProcessingInstruction {
                target,
                data: data.map(Cow::Owned),
            },
        ))
    }
}

impl<'a> ProcessingInstruction<'a> {
    //[17] PITarget	::= Name - (('X' | 'x') ('M' | 'm') ('L' | 'l'))
    fn parse_target(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, name) = Self::parse_name(input)?;

        if name.eq_ignore_ascii_case("xml") {
            Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        } else {
            Ok((input, name))
        }
    }
}
