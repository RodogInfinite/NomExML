use std::{
    error::Error,
    fmt::{self, Debug, Display},
    io::Error as IoError,
};

use nom::error::VerboseErrorKind;

#[macro_export]
macro_rules! warnln {
    ($($arg:tt)*) => ({
        eprintln!("\x1B[33mWARNING:\x1B[0m {}", format!($($arg)*));
    });
}

#[derive(Debug)]
pub enum CustomError<T> {
    NomError(nom::error::Error<T>),
    NomVerboseError(nom::error::VerboseError<T>),
    NomIncompleteError(nom::Needed),
    IoError(IoError),
}

impl<'a, T> Display for CustomError<T>
where
    T: Debug + Display + From<String> + From<&'a str> + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::NomError(e) => write!(f, "NomError: {}", e),
            CustomError::NomVerboseError(e) => {
                write!(f, "NomVerboseError: ")?;
                for (input, kind) in &e.errors {
                    match kind {
                        VerboseErrorKind::Context(ctx) => {
                            write!(f, "Context({}): {:?}", ctx, input)?
                        }
                        VerboseErrorKind::Char(c) => write!(f, "Char({}): {:?}", c, input)?,
                        VerboseErrorKind::Nom(err) => write!(f, "Nom({:?}): {:?}", err, input)?,
                    }
                }
                Ok(())
            }
            CustomError::NomIncompleteError(e) => write!(f, "IncompleteError: {:?}", e),

            CustomError::IoError(e) => write!(f, "IoError: {}", e),
        }
    }
}

impl<'a, T> Error for CustomError<T>
where
    T: Debug + Display + From<String> + From<&'a str> + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CustomError::NomError(e) => Some(e),
            CustomError::NomVerboseError(e) => Some(e),
            CustomError::IoError(e) => Some(e),
            CustomError::NomIncompleteError(_) => None,
        }
    }
}

impl<'a, T> From<nom::error::Error<T>> for CustomError<T>
where
    T: Debug + Display + From<String> + From<&'a str> + 'static,
{
    fn from(error: nom::error::Error<T>) -> Self {
        CustomError::NomError(error)
    }
}
impl<'a, T> From<nom::error::VerboseError<T>> for CustomError<T>
where
    T: Debug + Display + From<String> + From<&'a str> + 'static,
{
    fn from(error: nom::error::VerboseError<T>) -> Self {
        CustomError::NomVerboseError(error)
    }
}

impl<'a, T> From<nom::Err<nom::error::VerboseError<T>>> for CustomError<T>
where
    T: Debug + Display + From<String> + From<&'a str> + 'static,
{
    fn from(error: nom::Err<nom::error::VerboseError<T>>) -> Self {
        match error {
            nom::Err::Error(err) | nom::Err::Failure(err) => {
                let mut new_err = err;
                new_err.errors.push((
                    T::from("Context: parsing failed".to_string()),
                    VerboseErrorKind::Context("Parsing failed"),
                ));
                CustomError::from(new_err)
            }
            nom::Err::Incomplete(needed) => CustomError::NomIncompleteError(needed),
        }
    }
}
impl<'a, T> From<std::io::Error> for CustomError<T>
where
    T: Debug + Display + From<String> + From<&'a str> + 'static,
{
    fn from(error: std::io::Error) -> Self {
        CustomError::IoError(error)
    }
}

impl<'a, T> From<nom::Err<nom::error::Error<T>>> for CustomError<T>
where
    T: Debug + Display + From<String> + From<&'a str> + 'static,
{
    fn from(error: nom::Err<nom::error::Error<T>>) -> Self {
        match error {
            nom::Err::Error(err) => CustomError::from(err),
            nom::Err::Failure(err) => CustomError::from(err),
            nom::Err::Incomplete(needed) => CustomError::NomIncompleteError(needed),
        }
    }
}
impl<'a, T> From<nom::Needed> for CustomError<T>
where
    T: Debug + Display + From<String> + From<&'a str> + 'static,
{
    fn from(needed: nom::Needed) -> Self {
        CustomError::NomIncompleteError(needed)
    }
}

impl<'a, T> From<CustomError<T>> for nom::Err<Box<dyn Error>>
where
    T: Debug + Display + From<String> + From<&'a str> + 'static,
{
    fn from(error: CustomError<T>) -> Self {
        nom::Err::Error(Box::new(error))
    }
}
