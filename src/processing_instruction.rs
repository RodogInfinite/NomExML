// processing_instruction.rs

use crate::{parse::Parse, Name};
use nom::{
    bytes::complete::tag,
    combinator::{map, map_res, opt, peek},
    multi::many_till,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Clone, PartialEq, Eq)]
pub struct ProcessingInstruction {
    pub target: Name,
    pub data: Option<String>,
}

impl<'a> Parse<'a> for ProcessingInstruction {
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
            |(_open_tag, target, data_chars_opt, _close_tag)| {
                let data = data_chars_opt.map(|(chars, _)| chars.into_iter().collect::<String>());
                ProcessingInstruction { target, data }
            },
        )(input)
    }
}

impl ProcessingInstruction {
    //[17] PITarget	::= Name - (('X' | 'x') ('M' | 'm') ('L' | 'l'))
    fn parse_target(input: &str) -> IResult<&str, Name> {
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
