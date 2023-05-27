use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, multispace0, satisfy},
    combinator::{opt, recognize},
    multi::{many1, separated_list1},
    sequence::delimited,
    IResult,
};

pub trait Parse<'a>: Sized {
    fn parse(_input: &'a str) -> IResult<&'a str, Self> {
        unimplemented!()
    }

    // [3] S ::= (#x20 | #x9 | #xD | #xA)+
    // [3] S ::= (' '  | '\t' | '\r' | '\n')+
    fn parse_whitespace(input: &str) -> IResult<&str, &str> {
        take_while(|c: char| matches!(c, ' ' | '\t' | '\r' | '\n'))(input)
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
        matches!(c, ':'             |'A'..='Z'                  | '_'           | 'a'..='z' |
            '\u{C0}'..='\u{D6}'     | '\u{D8}'..='\u{F6}'       | '\u{F8}'..='\u{2FF}'      |
            '\u{370}'..='\u{37D}'   | '\u{37F}'..='\u{1FFF}'    | '\u{200C}'..='\u{200D}'   |
            '\u{2070}'..='\u{218F}' | '\u{2C00}'..='\u{2FEF}'   | '\u{3001}'..='\u{D7FF}'   |
            '\u{F900}'..='\u{FDCF}' | '\u{FDF0}'..='\u{FFFD}'   | '\u{10000}'..='\u{EFFFF}')
    }

    /*[4a] NameChar ::=
    NameStartChar |
    "-" | "." | [0-9] | #xB7 |
    [#x0300-#x036F] | [#x203F-#x2040] */
    fn is_name_char(c: char) -> bool {
        Self::is_name_start_char(c)
            || matches!(c, '-' |'.' |'0'..='9' | '\u{B7}' |
            '\u{0300}'..='\u{036F}' | '\u{203F}'..='\u{2040}')
    }

    fn parse_name_char(input: &'a str) -> IResult<&'a str, char> {
        satisfy(Self::is_name_char)(input)
    }

    fn parse_name_start_char(input: &'a str) -> IResult<&'a str, char> {
        satisfy(Self::is_name_start_char)(input)
    }

    // [7] Nmtoken ::= (NameChar)+
    fn parse_nmtoken(input: &'a str) -> IResult<&'a str, &'a str> {
        recognize(many1(Self::parse_name_char))(input)
    }

    // [8] Nmtokens ::= Nmtoken (#x20 Nmtoken)*
    fn parse_nmtokens(input: &'a str) -> IResult<&'a str, Vec<&'a str>> {
        separated_list1(char(' '), Self::parse_nmtoken)(input)
    }

    // [5] Name	::= NameStartChar (NameChar)*
    fn parse_name(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, start_char) = Self::parse_name_start_char(input)?;
        let (input, rest_chars) = opt(Self::parse_nmtoken)(input)?;

        let mut name = start_char.to_string();
        if let Some(rest) = rest_chars {
            name.push_str(rest);
        }
        Ok((input, Cow::Owned(name)))
    }

    // [6] Names ::= Name (#x20 Name)*
    fn parse_names(input: &'a str) -> IResult<&'a str, Vec<Cow<'a, str>>> {
        separated_list1(char(' '), Self::parse_name)(input)
    }

    fn parse_with_whitespace<F, O>(input: &'a str, mut parser: F) -> IResult<&'a str, O>
    where
        F: FnMut(&'a str) -> IResult<&'a str, O>,
    {
        let (input, _) = Self::parse_whitespace(input)?;
        let (input, result) = parser(input)?;
        let (input, _) = Self::parse_whitespace(input)?;
        Ok((input, result))
    }

    fn parse_literal(input: &'a str) -> IResult<&'a str, &'a str> {
        delimited(
            alt((tag("'"), tag("\""))),
            take_while(|c: char| c != '\'' && c != '\"' && c != '<' && c != '&'),
            alt((tag("'"), tag("\""))),
        )(input)
    }
}
