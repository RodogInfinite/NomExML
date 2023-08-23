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

pub fn parse_file<'a>(
    file: &mut File,
    buffer: &'a mut String,
) -> Result<Document<'a>, CustomError> {
    buffer.clear();
    buffer.push_str(&read_file(file)?);
    buffer.replace_range(.., &buffer.replace("\r\n", "\n"));

    let (_, document) = Document::parse_xml_str(buffer).map_err(|err| match err {
        nom::Err::Error(e) | nom::Err::Failure(e) => {
            CustomError::NomError(format!("error: {:?}, input: {}", e.code, e.input))
        }
        nom::Err::Incomplete(_) => CustomError::NomError("Incomplete parsing".to_string()),
    })?;

    Ok(document)
}

// pub fn parse_directory(path: &Path) -> Result<Vec<Result<Document, CustomError>>, CustomError> {
//     let entries = fs::read_dir(path)?;
//     let results = entries
//         .par_bridge()
//         .filter_map(Result::ok)
//         .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("xml")) // Fix the E0369 error by adding `to_str()` here.
//         .map(|entry| {
//             let mut file = File::open(entry.path())?;
//             parse_file(&mut file)
//         })
//         .collect::<Vec<_>>();
//     Ok(results)
// }
