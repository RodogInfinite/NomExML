// decode.rs

use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    combinator::opt,
    sequence::delimited,
    IResult,
};

pub trait Decode<'a> {
    fn decode(&self) -> Option<String>;
}

impl<'a> Decode<'a> for String {
    fn decode(&self) -> Option<String> {
        match self.as_ref() {
            "amp" => Some("&".to_string()),
            "lt" => Some("<".to_string()),
            "gt" => Some(">".to_string()),
            "quot" => Some("\"".to_string()),
            "apos" => Some("'".to_string()),
            _ => None,
        }
    }
}

pub fn decode_hex<'a>(input: &'a str, code: &'a str) -> IResult<&'a str, Cow<'a, str>> {
    let decoded_entity = match u32::from_str_radix(code, 16) {
        Ok(n) => match char::from_u32(n) {
            Some(c) => Cow::Owned(c.to_string()),
            None => Cow::Owned(format!("Invalid Unicode scalar value: {}", n)),
        },
        Err(_) => Cow::Owned(format!("Invalid hexadecimal number: {}", code)),
    };
    if input == decoded_entity {
        Ok((input, Cow::Borrowed(input)))
    } else {
        Ok((input, decoded_entity))
    }
}

pub fn decode_digit<'a>(input: &'a str, code: &'a str) -> IResult<&'a str, Cow<'a, str>> {
    let decoded_entity = match code.parse::<u32>() {
        Ok(n) => match char::from_u32(n) {
            Some(c) => Cow::Owned(c.to_string()),
            None => Cow::Owned(format!("Invalid Unicode scalar value: {}", n)),
        },
        Err(_) => Cow::Owned(format!("Invalid decimal number: {}", code)),
    };
    if input == decoded_entity {
        Ok((input, Cow::Borrowed(input)))
    } else {
        Ok((input, decoded_entity))
    }
}
