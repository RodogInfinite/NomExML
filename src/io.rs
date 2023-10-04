use crate::parse::Parse;
use crate::prolog::subset::entity_value::EntityValue;
use crate::prolog::subset::internal::InternalSubset;
use crate::prolog::subset::markup_declaration::MarkupDeclaration;
use crate::{Config, ExternalEntityParseConfig, Name};
// io.rs
use crate::{error::CustomError, Document};
use encoding_rs::*;
use nom::branch::alt;
use nom::combinator::{map, map_res};
use nom::multi::{many0, many1};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::BufReader;
use std::path::Path;
use std::rc::Rc;
use std::{
    fs::{self, File},
    io::Read,
};
pub fn read_file(file: &mut File) -> std::io::Result<String> {
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
    data = data.replace("\r\n", "\n").replace('\r', "\n");

    let (_, document) = Document::parse(&mut data, config).map_err(|err| match err {
        nom::Err::Error(e) | nom::Err::Failure(e) => {
            CustomError::NomError(format!("error: {:?}, input: {}", e.code, e.input))
        }
        nom::Err::Incomplete(_) => CustomError::NomError("Incomplete parsing".to_string()),
    })?;

    Ok(document)
}

pub fn parse_external_ent_file(
    file: &mut File,
    config: Config,
    entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
) -> Result<Vec<EntityValue>, CustomError> {
    let mut data = read_file(file)?;
    data = data.replace("\r\n", "\n").replace('\r', "\n");
    dbg!(&data);
    let (_, entity_values) = alt((
        map(
            |i| MarkupDeclaration::parse(i, entity_references.clone()),
            |int_subsets| {
                int_subsets
                    .into_iter()
                    .map(|markup_declaration| EntityValue::MarkupDecl(Box::new(markup_declaration)))
                    .collect::<Vec<_>>()
            },
        ),
        map_res(
            |i| Document::parse_prolog(i, entity_references.clone(), config.clone()),
            |(doc_option, _entity_refs)| {
                if let Some(doc) = doc_option {
                    dbg!("DOC HERE?");
                    dbg!(&doc);
                    Ok(vec![EntityValue::Document(doc)])
                } else {
                    Err(CustomError::NomError(
                        "Expected a document, but found None.".to_string(),
                    ))
                }
            },
        ),
        map(
            |i| Document::parse_content(i, entity_references.clone()),
            |doc| {
                dbg!("DOC");
                dbg!(&doc);
                vec![EntityValue::Document(doc)]
            },
        ),
    ))(data.as_str())
    .map_err(|err| match err {
        nom::Err::Error(e) | nom::Err::Failure(e) => {
            CustomError::NomError(format!("error: {:?}, input: {}", e.code, e.input))
        }
        nom::Err::Incomplete(_) => CustomError::NomError("Incomplete parsing".to_string()),
    })?;
    dbg!("PEEF");
    dbg!(&entity_values);
    Ok(entity_values)
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
