//parse.rs

use crate::{transcode::Decode, Name};
use nom::{
    bytes::complete::tag,
    character::complete::{char, satisfy},
    combinator::{map, opt, recognize},
    multi::{many0, many1, separated_list1},
    sequence::tuple,
    IResult, Offset,
};
use std::borrow::Cow;

pub trait Parse<'a>: Sized {
    type Args;
    type Output; //TODO: refactor this when associated type defaults are stabalized
    fn parse(_input: &'a str, _args: Self::Args) -> Self::Output {
        unimplemented!()
    }

    // [2] Char ::= #x9 | #xA | #xD | [#x20-#xD7FF] | [#xE000-#xFFFD] | [#x10000-#x10FFFF]
    // any Unicode character, excluding the surrogate blocks, FFFE, and FFFF.
    fn is_char(c: char) -> bool {
        matches!(c, '\u{9}' | '\u{A}' | '\u{D}' | '\u{20}'..='\u{D7FF}' | '\u{E000}'..='\u{FFFD}' | '\u{10000}'..='\u{10FFFF}')
    }

    fn parse_char(input: &'a str) -> IResult<&'a str, char> {
        satisfy(Self::is_char)(input)
    }

    // [3] S ::= (#x20 | #x9 | #xD | #xA)+
    // AKA [3] S ::= (' '  | '\t' | '\r' | '\n')+
    fn is_whitespace(c: char) -> bool {
        matches!(c, ' ' | '\t' | '\r' | '\n')
    }

    fn parse_multispace1(input: &'a str) -> IResult<&'a str, ()> {
        let (input, _) = many1(satisfy(Self::is_whitespace))(input)?;
        Ok((input, ()))
    }

    fn parse_multispace0(input: &'a str) -> IResult<&'a str, ()> {
        let (input, _) = many0(satisfy(Self::is_whitespace))(input)?;
        Ok((input, ()))
    }

    /*
    [4] NameStartChar ::=
        ":"                 | [A-Z]             | "_"           | [a-z]
        | [#xC0-#xD6]       | [#xD8-#xF6]       | [#xF8-#x2FF]
        | [#x370-#x37D]     | [#x37F-#x1FFF]    | [#x200C-#x200D]
        | [#x2070-#x218F]   | [#x2C00-#x2FEF]   | [#x3001-#xD7FF]
        | [#xF900-#xFDCF]   | [#xFDF0-#xFFFD]   | [#x10000-#xEFFFF]
    */
    fn is_name_start_char(c: char) -> bool {
        matches!(c, ':' | 'A'..='Z' | '_' | 'a'..='z' |
            '\u{C0}'..='\u{D6}' | '\u{D8}'..='\u{F6}' | '\u{F8}'..='\u{2FF}' |
            '\u{370}'..='\u{37D}' | '\u{37F}'..='\u{1FFF}' | '\u{200C}'..='\u{200D}' |
            '\u{2070}'..='\u{218F}' | '\u{2C00}'..='\u{2FEF}' | '\u{3001}'..='\u{D7FF}' |
            '\u{F900}'..='\u{FDCF}' | '\u{FDF0}'..='\u{FFFD}' | '\u{10000}'..='\u{EFFFF}')
    }

    /*  [4a] NameChar ::=
                NameStartChar |
                "-" | "." | [0-9] | #xB7 |
                [#x0300-#x036F] | [#x203F-#x2040]
    */
    fn is_name_char(c: char) -> bool {
        Self::is_name_start_char(c)
            || matches!(c, '-' | '.' | '0'..='9' | '\u{B7}' |
            '\u{0300}'..='\u{036F}' | '\u{203F}'..='\u{2040}')
    }

    fn parse_name_char(input: &'a str) -> IResult<&'a str, char> {
        satisfy(Self::is_name_char)(input)
    }

    fn parse_name_start_char(input: &'a str) -> IResult<&'a str, char> {
        satisfy(Self::is_name_start_char)(input)
    }

    // [7] Nmtoken ::= (NameChar)+
    fn parse_nmtoken(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, result) = recognize(many1(Self::parse_name_char))(input)?;
        Ok((input, Cow::Borrowed(result)))
    }

    // [8] Nmtokens ::= Nmtoken (#x20 Nmtoken)*
    fn parse_nmtokens(input: &'a str) -> IResult<&'a str, Vec<Cow<'a, str>>> {
        separated_list1(char(' '), Self::parse_nmtoken)(input)
    }

    // [5] Name ::= NameStartChar (NameChar)*
    fn parse_name(input: &'a str) -> IResult<&'a str, Name> {
        map(
            tuple((Self::parse_name_start_char, opt(Self::parse_nmtoken))),
            |(start_char, rest_chars)| {
                let mut name = start_char.to_string();
                if let Some(rest) = rest_chars {
                    name.push_str(&rest);
                }

                let name_clone = name.clone();
                // Attempt to decode the cloned name.
                let local_part = match name_clone.decode() {
                    Ok(decoded) => decoded.into_owned(),
                    Err(_) => name,
                };
                dbg!(&local_part);
                Name {
                    prefix: None,
                    local_part: Cow::Owned(local_part),
                }
            },
        )(input)
    }

    // [6] Names ::= Name (#x20 Name)*
    fn parse_names(input: &'a str) -> IResult<&'a str, Vec<Name>> {
        separated_list1(char(' '), Self::parse_name)(input)
    }

    //[25] Eq ::=  S? '=' S?
    fn parse_eq(input: &'a str) -> IResult<&'a str, ()> {
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        Ok((input, ()))
    }

    fn capture_span<O, F>(
        mut f: F,
    ) -> Box<dyn FnMut(&'a str) -> IResult<&'a str, (&'a str, O)> + 'a>
    where
        F: FnMut(&'a str) -> IResult<&'a str, O> + 'a,
    {
        Box::new(move |input: &'a str| {
            let (remaining, result) = f(input)?;
            let offset = input.offset(remaining);
            Ok((remaining, (&input[..offset], result)))
        })
    }
}
