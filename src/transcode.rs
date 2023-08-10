// transcode.rs

use nom::{
    error::{Error, ErrorKind},
    IResult,
};
use std::borrow::Cow;

pub trait Decode {
    fn as_str(&self) -> &str;

    fn decode(&self) -> Result<Cow<str>, Box<dyn std::error::Error + '_>> {
        match self.as_str() {
            "amp" => Ok(Cow::Borrowed("&")),
            "lt" => Ok(Cow::Borrowed("<")),
            "gt" => Ok(Cow::Borrowed(">")),
            "quot" => Ok(Cow::Borrowed("\"")),
            "apos" => Ok(Cow::Borrowed("'")),
            s if s.starts_with("&#x") && s.ends_with(";") => {
                let code = &s[3..s.len() - 1]; // slice to get content inside &#x...;
                match self.decode_hex(code) {
                    Ok((_, cow)) => Ok(cow),
                    Err(e) => Err(Box::new(e)),
                }
            }
            s if s.starts_with("&#") && s.ends_with(";") => {
                let code = &s[2..s.len() - 1]; // slice to get content inside &#...;
                match self.decode_digit(code) {
                    Ok((_, cow)) => Ok(cow),
                    Err(e) => Err(Box::new(e)),
                }
            }
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to decode",
            ))),
        }
    }

    fn decode_hex(&self, code: &str) -> IResult<&str, Cow<str>, Error<&str>> {
        match u32::from_str_radix(code, 16) {
            Ok(n) => match char::from_u32(n) {
                Some(c) => Ok((self.as_str(), Cow::Owned(c.to_string()))),
                None => Err(nom::Err::Error(Error::new(
                    self.as_str(),
                    ErrorKind::MapRes,
                ))),
            },
            Err(_) => Err(nom::Err::Error(Error::new(
                self.as_str(),
                ErrorKind::MapRes,
            ))),
        }
    }

    fn decode_digit(&self, code: &str) -> IResult<&str, Cow<str>, Error<&str>> {
        match code.parse::<u32>() {
            Ok(n) => match char::from_u32(n) {
                Some(c) => Ok((self.as_str(), Cow::Owned(c.to_string()))),
                None => Err(nom::Err::Error(Error::new(
                    self.as_str(),
                    ErrorKind::MapRes,
                ))),
            },
            Err(_) => Err(nom::Err::Error(Error::new(
                self.as_str(),
                ErrorKind::MapRes,
            ))),
        }
    }
}

impl Decode for String {
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl Decode for &str {
    fn as_str(&self) -> &str {
        self
    }
}

pub trait Encode {
    fn as_str(&self) -> &str;

    fn encode(&self) -> Result<String, Box<dyn std::error::Error + '_>> {
        match self.as_str() {
            "&" => Ok("amp".to_string()),
            "<" => Ok("lt".to_string()),
            ">" => Ok("gt".to_string()),
            "\"" => Ok("quot".to_string()),
            "'" => Ok("apos".to_string()),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unsupported character for encoding",
            ))),
        }
    }

    fn encode_hex(&self, ch: char) -> String {
        format!("&#x{:X};", ch as u32)
    }

    fn encode_digit(&self, ch: char) -> String {
        format!("&#{};", ch as u32)
    }
}

impl Encode for String {
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl Encode for &str {
    fn as_str(&self) -> &str {
        self
    }
}
