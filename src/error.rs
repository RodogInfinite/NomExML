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
    NomErrorFast(nom::error::ErrorKind),
    IoError(std::io::Error),
    UserAbort(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NomError(e) => write!(f, "NomError: {}", e),
            Error::NomErrorFast(kind) => write!(f, "NomErrorFast: {:?}", kind),
            Error::IoError(e) => write!(f, "IoError: {}", e),
            Error::UserAbort(e) => write!(f, "UserAbort: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::NomError(_) => None,
            Error::NomErrorFast(_) => None,
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
        Error::NomError(nom::error::Error::new(error.input.into(), error.code))
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
    I: Debug + ToString,
{
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        if cfg!(debug_assertions) {
            Error::NomError(nom::error::Error::new(input.to_string(), kind))
        } else {
            Error::NomErrorFast(kind)
        }
    }

    fn append(input: I, kind: nom::error::ErrorKind, _other: Self) -> Self {
        if cfg!(debug_assertions) {
            Error::NomError(nom::error::Error::new(input.to_string(), kind))
        } else {
            Error::NomErrorFast(kind)
        }
    }

    fn from_char(input: I, _: char) -> Self {
        if cfg!(debug_assertions) {
            Error::NomError(nom::error::Error::new(
                input.to_string(),
                nom::error::ErrorKind::Char,
            ))
        } else {
            Error::NomErrorFast(nom::error::ErrorKind::Char)
        }
    }

    fn or(self, _other: Self) -> Self {
        self
    }
}

impl<I, E> nom::error::FromExternalError<I, E> for Error
where
    I: Debug + ToString,
    E: Debug + Display,
{
    fn from_external_error(input: I, kind: nom::error::ErrorKind, _e: E) -> Self {
        if cfg!(debug_assertions) {
            Error::NomError(nom::error::Error::new(input.to_string(), kind))
        } else {
            Error::NomErrorFast(kind)
        }
    }
}

pub trait ConvertNomError<E> {
    fn convert_nom_error(self) -> nom::Err<Error>;
}

impl<E> ConvertNomError<nom::error::Error<E>> for nom::Err<nom::error::Error<E>>
where
    E: Debug + ToString,
{
    fn convert_nom_error(self) -> nom::Err<Error> {
        match self {
            nom::Err::Error(e) => {
                let nom::error::Error { input, code } = e;
                nom::Err::Error(if cfg!(debug_assertions) {
                    Error::NomError(nom::error::Error::new(input.to_string(), code))
                } else {
                    Error::NomErrorFast(code)
                })
            }
            nom::Err::Failure(e) => {
                let nom::error::Error { input, code } = e;
                nom::Err::Failure(if cfg!(debug_assertions) {
                    Error::NomError(nom::error::Error::new(input.to_string(), code))
                } else {
                    Error::NomErrorFast(code)
                })
            }
            nom::Err::Incomplete(needed) => nom::Err::Incomplete(needed),
        }
    }
}
