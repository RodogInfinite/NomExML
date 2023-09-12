use crate::parse::Parse;
use crate::Config;
// io.rs
use crate::{error::CustomError, Document};
use encoding_rs::*;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::io::BufReader;
use std::path::Path;
use std::{
    fs::{self, File},
    io::Read,
};
fn read_file(file: &mut File) -> std::io::Result<String> {
    let mut reader = BufReader::new(file);
    let mut bytes = vec![];

    reader.read_to_end(&mut bytes)?;

    let (encoding, bom_length) = match Encoding::for_bom(&bytes) {
        Some((enc, len)) => (enc, len),
        None => (UTF_8, 0),
    };
    let (decoded_str, _, _) = encoding.decode(&bytes[bom_length..]);

    Ok(decoded_str.into_owned())
}

pub fn parse_file(file: &mut File, config: Config) -> Result<Document, CustomError> {
    let mut data = read_file(file)?;
    data = data.replace("\r\n", "\n");

    let (_, document) = Document::parse(&mut data, config).map_err(|err| match err {
        nom::Err::Error(e) | nom::Err::Failure(e) => {
            CustomError::NomError(format!("error: {:?}, input: {}", e.code, e.input))
        }
        nom::Err::Incomplete(_) => CustomError::NomError("Incomplete parsing".to_string()),
    })?;

    Ok(document)
}

pub fn parse_directory(
    path: &Path,
    config: Config,
) -> Result<Vec<Result<Document, CustomError>>, CustomError> {
    let entries = fs::read_dir(path)?;
    let results = entries
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("xml"))
        .map(|entry| {
            let mut file = File::open(entry.path())?;
            parse_file(&mut file, config.clone())
        })
        .collect::<Vec<_>>();
    Ok(results)
}
