// processing_instruction.rs

use crate::{parse::Parse, Name};
use nom::{
    bytes::complete::tag,
    combinator::{map, map_res, opt, peek},
    multi::many_till,
    sequence::{preceded, tuple},
    IResult,
};
use std::borrow::Cow;

#[derive(Clone, PartialEq)]
pub struct ProcessingInstruction<'a> {
    pub target: Name<'a>,
    pub data: Option<Cow<'a, str>>,
}

impl<'a> Parse<'a> for ProcessingInstruction<'a> {
    type Args = ();
    type Output = IResult<&'a str, Self>;
    // [16] PI ::= '<?' PITarget (S (Char* - (Char* '?>' Char*)))? '?>'
    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
        map(
            tuple((
                tag("<?"),
                Self::parse_target,
                opt(preceded(
                    Self::parse_multispace1,
                    many_till(Self::parse_char, peek(tag("?>"))),
                )),
                tag("?>"),
            )),
            |(_, target, data_chars, _)| {
                let data: Option<String> = data_chars.map(|(chars, _)| chars.into_iter().collect());
                ProcessingInstruction {
                    target,
                    data: data.map(Cow::Owned),
                }
            },
        )(input)
    }
}

impl<'a> ProcessingInstruction<'a> {
    //[17] PITarget	::= Name - (('X' | 'x') ('M' | 'm') ('L' | 'l'))
    fn parse_target(input: &'a str) -> IResult<&'a str, Name> {
        map_res(Self::parse_name, |name| {
            if name.local_part.eq_ignore_ascii_case("xml") {
                Err(nom::Err::Failure(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Tag,
                )))
            } else {
                Ok(name)
            }
        })(input)
    }
}
