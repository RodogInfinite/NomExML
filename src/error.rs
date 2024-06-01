use std::fmt::{self, Debug, Display};

use nom::error::ParseError;

#[macro_export]
macro_rules! warnln {
    ($($arg:tt)*) => ({
        eprintln!("\x1B[33mWARNING:\x1B[0m {}", format!($($arg)*));
    });
}

#[derive(Debug)]
pub enum Error<E>
where
    E: Debug + Display + 'static,
{
    NomError(nom::error::Error<E>), // Error message and input string
    IoError(std::io::Error),
    UserAbort(String),
}

impl<E> Display for Error<E>
where
    E: Debug + Display + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NomError(e) => write!(f, "NomError: {e}"),
            Error::IoError(e) => write!(f, "IoError: {e}"),
            Error::UserAbort(e) => write!(f, "UserAbort: {e}"),
        }
    }
}

impl<E> std::error::Error for Error<E>
where
    E: Debug + Display + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::NomError(e) => Some(e),
            Error::IoError(e) => Some(e),
            Error::UserAbort(_) => None,
        }
    }
}

impl<E> From<std::io::Error> for Error<E>
where
    E: Debug + Display + 'static,
{
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl<E> From<nom::Err<nom::error::Error<&str>>> for Error<E>
where
    E: Debug + Display + From<String> + 'static,
{
    fn from(error: nom::Err<nom::error::Error<&str>>) -> Self {
        match error {
            nom::Err::Error(err) => Error::from(err),
            nom::Err::Failure(err) => Error::from(err),
            nom::Err::Incomplete(_) => Error::NomError(nom::error::Error::new(
                "Incomplete parsing".to_string().into(),
                nom::error::ErrorKind::Fail,
            )),
        }
    }
}

impl<E> From<Error<E>> for nom::Err<nom::error::Error<String>>
where
    E: Debug + Display + From<E> + 'static,
    std::string::String: std::convert::From<E>,
{
    fn from(error: Error<E>) -> Self {
        match error {
            Error::NomError(e) => nom::Err::Error(nom::error::Error::new(e.input.into(), e.code)),
            Error::IoError(_) => nom::Err::Error(nom::error::Error::new(
                "IO error".into(),
                nom::error::ErrorKind::Fail,
            )),
            Error::UserAbort(e) => nom::Err::Error(nom::error::Error::new(
                e.into(),
                nom::error::ErrorKind::Fail,
            )),
        }
    }
}

impl<E> From<Error<E>> for nom::Err<Error<E>>
where
    E: Debug + Display + 'static,
{
    fn from(error: Error<E>) -> Self {
        nom::Err::Error(error)
    }
}

impl<E> From<Error<E>> for nom::Err<Box<dyn std::error::Error>>
where
    E: Debug + Display + 'static,
{
    fn from(error: Error<E>) -> Self {
        nom::Err::Error(Box::new(error))
    }
}
impl<I> nom::error::ParseError<I> for Error<String>
where
    I: Debug,
{
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        Error::NomError(nom::error::Error::new(format!("{:?}", input).into(), kind))
    }

    fn append(input: I, kind: nom::error::ErrorKind, other: Self) -> Self {
        Error::NomError(nom::error::Error::new(format!("{:?}", input).into(), kind))
    }

    fn from_char(input: I, _: char) -> Self {
        Error::NomError(nom::error::Error::new(
            format!("{:?}", input).into(),
            nom::error::ErrorKind::Char,
        ))
    }

    fn or(self, other: Self) -> Self {
        self
    }
}

impl From<Box<dyn std::error::Error>> for Error<&'static str> {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Error::UserAbort(error.to_string())
    }
}

impl<E> From<nom::error::Error<&str>> for Error<E>
where
    E: Debug + Display + From<String> + 'static,
{
    fn from(error: nom::error::Error<&str>) -> Self {
        let owned_input: E = error.input.to_string().into();
        Error::NomError(nom::error::Error::new(owned_input, error.code))
    }
}

impl<E> From<nom::error::Error<String>> for Error<E>
where
    E: Debug + Display + From<String> + 'static,
{
    fn from(error: nom::error::Error<String>) -> Self {
        let owned_input: E = error.input.into();
        Error::NomError(nom::error::Error::new(owned_input, error.code))
    }
}

impl nom::error::ParseError<&str> for Error<&'static str> {
    fn from_error_kind(input: &str, kind: nom::error::ErrorKind) -> Self {
        Error::NomError(nom::error::Error::new("static input", kind))
    }

    fn append(input: &str, kind: nom::error::ErrorKind, other: Self) -> Self {
        Error::NomError(nom::error::Error::new("static input", kind))
    }
}

impl<I, E> nom::error::FromExternalError<I, E> for Error<String>
where
    I: Debug,
    E: Debug + Display,
{
    fn from_external_error(input: I, kind: nom::error::ErrorKind, _e: E) -> Self {
        Error::NomError(nom::error::Error::new(format!("{:?}", input).into(), kind))
    }
}
impl<I, E> nom::error::FromExternalError<I, E> for Error<&'static str>
where
    I: Debug,
    E: Debug + Display,
{
    fn from_external_error(input: I, kind: nom::error::ErrorKind, _e: E) -> Self {
        Error::NomError(nom::error::Error::new("static input", kind))
    }
}

pub fn convert_nom_err(error: nom::Err<nom::error::Error<&str>>) -> nom::Err<Error<String>> {
    match error {
        nom::Err::Error(err) => nom::Err::Error(Error::from(err)),
        nom::Err::Failure(err) => nom::Err::Failure(Error::from(err)),
        nom::Err::Incomplete(needed) => nom::Err::Incomplete(needed),
    }
}

pub fn convert_nom_err_string(error: nom::Err<nom::error::Error<&str>>) -> nom::Err<Error<String>> {
    match error {
        nom::Err::Error(err) => nom::Err::Error(Error::from(nom::error::Error::new(
            "static input",
            err.code,
        ))),
        nom::Err::Failure(err) => nom::Err::Failure(Error::from(nom::error::Error::new(
            "static input",
            err.code,
        ))),
        nom::Err::Incomplete(needed) => nom::Err::Incomplete(needed),
    }
}
