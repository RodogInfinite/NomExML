// decode.rs

use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    combinator::opt,
    sequence::delimited,
    IResult,
};

pub fn decode_entity(input: &str) -> IResult<&str, Cow<str>> {
    if input.is_empty() {
        return Ok((input, Cow::Borrowed(input)));
    }
    let (input, digit_code) = opt(delimited(
        tag("&#"),
        take_while1(|c: char| c.is_numeric()),
        tag(";"),
    ))(input)?;

    let (input, hex_code) = opt(delimited(
        alt((tag("&#x"), tag("&#X"))),
        take_while1(|c: char| c.is_numeric()),
        tag(";"),
    ))(input)?;

    if let Some(code) = digit_code {
        decode_digit(input, code)
    } else if let Some(code) = hex_code {
        decode_hex(input, code)
    } else {
        let (input, entity) = opt(delimited(tag("&"), take_until(";"), tag(";")))(input)?;

        if let Some(entity) = entity {
            let decoded_entity = match entity {
                "amp" => Cow::Borrowed("&"),
                "lt" => Cow::Borrowed("<"),
                "gt" => Cow::Borrowed(">"),
                "quot" => Cow::Borrowed("\""),
                "apos" => Cow::Borrowed("'"),
                _ => Cow::Borrowed(input),
            };
            if input == decoded_entity {
                Ok((input, Cow::Borrowed(input)))
            } else {
                Ok((input, decoded_entity))
            }
        } else {
            Ok((input, Cow::Borrowed(input)))
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

pub fn decode_entities(input: &str) -> IResult<&str, Cow<str>> {
    let mut output = String::new();
    let mut input = input;
    loop {
        if input.is_empty() {
            break;
        }
        let (tail, decoded_entity) = decode_entity(input)?;
        if decoded_entity == Cow::Borrowed(input) {
            output.push_str(input);
            input = "";
        } else if decoded_entity.is_empty() || tail == input {
            break;
        } else {
            output.push_str(&decoded_entity);
            input = tail;
        }
    }
    Ok((input, Cow::Owned(output)))
}
