use std::{
    error::Error,
    fmt::{self, Display},
    io::Error as IoError,
};

#[macro_export]
macro_rules! warnln {
    ($($arg:tt)*) => ({
        eprintln!("\x1B[33mWARNING:\x1B[0m {}", format!($($arg)*));
    });
}

#[derive(Debug)]
pub enum CustomError {
    NomError(String, String), // Error message and input string
    IoError(IoError),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::NomError(msg, input) => write!(f, "NomError: {}, input: {}", msg, input),
            CustomError::IoError(e) => write!(f, "IoError: {}", e),
        }
    }
}

impl Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CustomError::NomError(_, _) => None,
            CustomError::IoError(e) => Some(e),
        }
    }
}

impl From<nom::error::Error<&str>> for CustomError {
    fn from(error: nom::error::Error<&str>) -> Self {
        CustomError::NomError(format!("error: {:?}", error.code), error.input.to_string())
    }
}

impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError::IoError(error)
    }
}

impl From<nom::Err<nom::error::Error<&str>>> for CustomError {
    fn from(error: nom::Err<nom::error::Error<&str>>) -> Self {
        match error {
            nom::Err::Error(err) => CustomError::from(err),
            nom::Err::Failure(err) => CustomError::from(err),
            nom::Err::Incomplete(_) => {
                CustomError::NomError("Incomplete parsing".to_string(), "".to_string())
            }
        }
    }
}

impl From<CustomError> for nom::Err<nom::error::Error<String>> {
    fn from(error: CustomError) -> Self {
        match error {
            CustomError::NomError(_msg, input) => {
                // Pass the owned string directly
                nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Fail))
            }
            CustomError::IoError(_) => {
                // For IO errors, use a generic message
                nom::Err::Error(nom::error::Error::new(
                    "IO error".to_string(),
                    nom::error::ErrorKind::Fail,
                ))
            }
        }
    }
}

impl From<CustomError> for nom::Err<CustomError> {
    fn from(error: CustomError) -> Self {
        nom::Err::Error(error)
    }
}

// impl From<CustomError> for nom::Err<nom::error::Error<String>> {
//     fn from(error: CustomError) -> Self {
//         match error {
//             CustomError::NomError(msg, input) => {
//                 // Now handling both the error message and the input string
//                 let combined_message = format!("{}: input {}", msg, input);
//                 nom::Err::Error(nom::error::Error::new(
//                     combined_message,
//                     nom::error::ErrorKind::Fail,
//                 ))
//             }
//             CustomError::IoError(_) => {
//                 // For IO errors, a generic message is used
//                 let io_error_message = "IO error".to_string();
//                 nom::Err::Error(nom::error::Error::new(
//                     io_error_message,
//                     nom::error::ErrorKind::Fail,
//                 ))
//             }
//         }
//     }
//}
