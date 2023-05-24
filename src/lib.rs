pub mod attribute;
mod debug;
pub mod declaration;
pub mod decode;
pub mod document;
mod error;
pub mod tag;
pub mod utils;


use document::Document;
use error::CustomError;

use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{io::Error as IoError, path::Path};
use std::{
    fs::{self, File},
    io::Read,
};
use tag::Tag;

pub struct Elements<'a> {
    tags: Vec<&'a Document<'a>>,
}

impl<'a> Elements<'a> {
    pub fn extract_content(&self) -> Vec<Option<&'a str>> {
        self.tags.iter().map(|tag| tag.extract_content()).collect()
    }
}

pub fn read_file(file: &mut File) -> Result<String, IoError> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn parse_file(file: &mut File) -> Result<Document<'static>, CustomError> {
    let content = read_file(file)?;
    let content = Box::leak(content.into_boxed_str());
    let (_, document) = Document::parse_xml_str(content).map_err(|err| match err {
        nom::Err::Error(e) | nom::Err::Failure(e) => {
            CustomError::NomError(format!("error: {:?}, input: {}", e.code, e.input))
        }
        nom::Err::Incomplete(_) => CustomError::NomError("Incomplete parsing".to_string()),
    })?;
    Ok(document)
}

pub fn parse_directory(path: &Path) -> Result<Vec<Result<Document, CustomError>>, CustomError> {
    let entries = fs::read_dir(path)?;
    let results = entries
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("xml")) // Fix the E0369 error by adding `to_str()` here.
        .map(|entry| {
            let mut file = File::open(entry.path())?;
            parse_file(&mut file)
        })
        .collect::<Vec<_>>();
    Ok(results)
}
