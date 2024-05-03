use crate::parse::Parse;
use crate::prolog::subset::entity::entity_value::EntityValue;

use crate::prolog::subset::entity::EntitySource;
use crate::prolog::subset::markup_declaration::MarkupDeclaration;
use crate::prolog::textdecl::TextDecl;
use crate::reference::Reference;
use crate::{Config, Name};

use crate::{error::CustomError, Document};
use encoding_rs::*;
use nom::branch::alt;
use nom::combinator::{map, map_res, opt};

use nom::multi::many1;

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

    let parse_result = Document::parse(&data, config);
    match parse_result {
        Ok((_, document)) => {
            // Successfully parsed document, return it
            Ok(document)
        }
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
            // Handle Nom parsing errors
            let custom_error = CustomError::NomError(format!("{:?}", e.code), e.input.to_string());
            Err(custom_error)
        }
        Err(nom::Err::Incomplete(_)) => {
            // Handle incomplete parsing errors
            let custom_error =
                CustomError::NomError("parse_file: Incomplete parsing".to_string(), data);
            Err(custom_error)
        }
    }
}

pub fn parse_external_entity_file(
    file: &mut File,
    config: &Config,
    external_entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
) -> Result<Vec<EntityValue>, CustomError> {
    let mut data = read_file(file)?;
    data = data.replace("\r\n", "\n").replace('\r', "\n");
    let (input, _text_decl) = opt(|i| TextDecl::parse(i, ()))(data.as_str())?;
    //TODO: handle the text_decl such that if the encoding being used to parse the file is different, then the encoding is handled accordingly, i.e file being parsed again with the proper decoding
    let (_, entity_values) = alt((
        map_res(
            |i| Document::parse_prolog(i, external_entity_references.clone(), config.clone()),
            |(doc_option, _entity_refs)| {
                if let Some(doc) = doc_option {
                    Ok(vec![EntityValue::Document(doc)])
                } else {
                    Err(CustomError::NomError(
                        "parse_external_ent_file: Expected a Document, but found None. Prolog not parsed.".to_string(), input.to_string()
                    ))
                }
            },
        ),
        many1(
            map(
                |i| Reference::parse(i, EntitySource::External),
                EntityValue::Reference
            )
        ),
        map_res(
            |i| MarkupDeclaration::parse(i, (external_entity_references.clone(),EntitySource::External)),
            |opt_markup_decl| {
                        if opt_markup_decl.is_some() {
                            Ok(
                                opt_markup_decl
                                .into_iter()
                                .map(|markup_declaration| {
                                    EntityValue::MarkupDecl(Box::new(markup_declaration))
                                })
                                .collect::<Vec<_>>()
                        )
                } else {
                    Err(CustomError::NomError(
                        "parse_external_ent_file: Failed to parse MarkupDeclaration"
                            .to_string(), input.to_string()
                    ))
                }
            },
        ),
        map(
            |i| Document::parse_content(i, external_entity_references.clone(),EntitySource::External),
            |doc| {
                vec![EntityValue::Document(doc)]
            },
        ),
    ))(input)
    .map_err(|err| match err {
        nom::Err::Error(e) | nom::Err::Failure(e) => {
            CustomError::NomError(format!("{:?}", e.code), e.input.to_string())
        }
        nom::Err::Incomplete(_) => CustomError::NomError("parse_external_ent_file: Incomplete parsing".to_string(),input.to_string()),
    })?;
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
