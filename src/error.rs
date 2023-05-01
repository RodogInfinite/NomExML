use std::{
    error::Error,
    fmt::{self, Display},
    io::Error as IoError,
};

// New custom error type to handle both NomError and IoError
#[derive(Debug)]
pub enum CustomError {
    NomError(String),
    IoError(IoError),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::NomError(msg) => write!(f, "NomError: {}", msg),
            CustomError::IoError(e) => write!(f, "IoError: {}", e),
        }
    }
}

impl Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CustomError::NomError(_) => None,
            CustomError::IoError(e) => Some(e),
        }
    }
}

impl From<nom::error::Error<&'static str>> for CustomError {
    fn from(error: nom::error::Error<&'static str>) -> Self {
        CustomError::NomError(format!("error: {:?}, input: {}", error.code, error.input))
    }
}

impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError::IoError(error)
    }
}
