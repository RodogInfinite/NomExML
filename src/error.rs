use std::fmt::{self, Debug, Display};

#[macro_export]
macro_rules! warnln {
    ($($arg:tt)*) => ({
        eprintln!("\x1B[33mWARNING:\x1B[0m {}", format!($($arg)*));
    });
}

#[derive(Debug)]
pub enum Error {
    NomError(nom::error::Error<String>),
    IoError(std::io::Error),
    UserAbort(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NomError(e) => write!(f, "NomError: {}", e),
            Error::IoError(e) => write!(f, "IoError: {}", e),
            Error::UserAbort(e) => write!(f, "UserAbort: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::NomError(e) => Some(e),
            Error::IoError(e) => Some(e),
            Error::UserAbort(_) => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<nom::error::Error<&str>> for Error {
    fn from(error: nom::error::Error<&str>) -> Self {
        let converted_error = nom::error::Error::new(error.input.to_string(), error.code);
        Error::NomError(converted_error)
    }
}

impl From<nom::error::Error<String>> for Error {
    fn from(error: nom::error::Error<String>) -> Self {
        Error::NomError(error)
    }
}

impl From<Error> for nom::Err<Error> {
    fn from(error: Error) -> Self {
        nom::Err::Failure(error)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Error::NomError(nom::error::Error::new(
            error.to_string(),
            nom::error::ErrorKind::Fail,
        ))
    }
}

impl<I> nom::error::ParseError<I> for Error
where
    I: Debug,
{
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        Error::NomError(nom::error::Error::new(format!("{:?}", input), kind))
    }

    fn append(input: I, kind: nom::error::ErrorKind, _other: Self) -> Self {
        Error::NomError(nom::error::Error::new(format!("{:?}", input), kind))
    }

    fn from_char(input: I, _: char) -> Self {
        Error::NomError(nom::error::Error::new(
            format!("{:?}", input),
            nom::error::ErrorKind::Char,
        ))
    }

    fn or(self, _other: Self) -> Self {
        self
    }
}

impl<I, E> nom::error::FromExternalError<I, E> for Error
where
    I: Debug,
    E: Debug + Display,
{
    fn from_external_error(input: I, kind: nom::error::ErrorKind, _e: E) -> Self {
        Error::NomError(nom::error::Error::new(format!("{:?}", input), kind))
    }
}
