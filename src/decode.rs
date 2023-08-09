// decode.rs

use nom::{
    error::{Error, ErrorKind},
    IResult,
};
use std::borrow::Cow;

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

pub fn decode_hex<'a>(
    input: &'a str,
    code: &'a str,
) -> IResult<&'a str, Cow<'a, str>, Error<&'a str>> {
    match u32::from_str_radix(code, 16) {
        Ok(n) => match char::from_u32(n) {
            Some(c) => Ok((input, Cow::Owned(c.to_string()))),
            None => Err(nom::Err::Error(Error::new(input, ErrorKind::MapRes))),
        },
        Err(_) => Err(nom::Err::Error(Error::new(input, ErrorKind::MapRes))),
    }
}

pub fn decode_digit<'a>(
    input: &'a str,
    code: &'a str,
) -> IResult<&'a str, Cow<'a, str>, Error<&'a str>> {
    match code.parse::<u32>() {
        Ok(n) => match char::from_u32(n) {
            Some(c) => Ok((input, Cow::Owned(c.to_string()))),
            None => Err(nom::Err::Error(Error::new(input, ErrorKind::MapRes))),
        },
        Err(_) => Err(nom::Err::Error(Error::new(input, ErrorKind::MapRes))),
    }
}
