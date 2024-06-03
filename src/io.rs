use crate::attribute::Attribute;
use crate::parse::Parse;
use crate::prolog::subset::entity::entity_value::EntityValue;
use crate::Name;

use crate::prolog::subset::entity::{self, EntitySource};
use crate::prolog::subset::markup_declaration::MarkupDeclaration;
use crate::prolog::textdecl::TextDecl;
use crate::reference::Reference;
use crate::tag::Tag;
use crate::Config;

use crate::{error::Error, Document, Result};
use encoding_rs::*;
use nom::branch::alt;
use nom::combinator::{map, map_res, opt};

use nom::multi::many1;

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::BufReader;

use std::rc::Rc;
use std::{fs::File, io::Read};

/// Read the file and decode the contents into a String
pub fn read_file(file: &mut File) -> std::io::Result<String> {
    let mut reader = BufReader::new(file);
    let mut bytes = vec![];

    reader.read_to_end(&mut bytes)?;

    let (encoding, bom_length) = match Encoding::for_bom(&bytes) {
        Some((enc, len)) => (enc, len),
        None => (UTF_8, 0),
    };
    let (decoded_str, _, _) = encoding.decode(&bytes[bom_length..]);

    let mut data = decoded_str.into_owned();

    data = data.replace("\r\n", "\n").replace('\r', "\n");

    Ok(data)
}

/// Parse the entire file into a Document
///
/// Note: Beware using for extremely large files as it will load the entire file into memory
pub fn parse_entire_file(file: &mut File, config: Config) -> Result<Document> {
    let data = read_file(file)?;

    let parse_result = Document::parse(&data, config);
    match parse_result {
        Ok((_, document)) => Ok(document),
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
            // Handle Nom parsing errors
            Err(Error::NomError(nom::error::Error::new(
                e.to_string(),
                nom::error::ErrorKind::Fail,
            ))
            .into())
        }
        Err(nom::Err::Incomplete(_)) => Err(Error::NomError(nom::error::Error::new(
            "parse_file: Incomplete parsing".to_string(),
            nom::error::ErrorKind::Fail,
        ))
        .into()),
    }
}

// Parse only the first matching element from a file
pub fn parse_element_from_file(
    file: &mut File,

    tag_name: &str,
    attributes: &Option<Vec<Attribute>>,
    entity_references: &Rc<RefCell<HashMap<(Name, entity::EntitySource), EntityValue>>>,
) -> Result<Document> {
    let data = read_file(file)?;

    let parse_result = Document::parse_element_by_tag_name(&data, tag_name, attributes);
    match parse_result {
        Ok((_, document)) => Ok(document),
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => Err(Error::NomError(
            nom::error::Error::new(e.to_string(), nom::error::ErrorKind::Fail),
        )
        .into()),
        Err(nom::Err::Incomplete(_)) => Err(Error::NomError(nom::error::Error::new(
            "parse_element_from_file: Incomplete parsing".to_string(),
            nom::error::ErrorKind::Fail,
        ))
        .into()),
    }
}

//
pub(crate) fn parse_external_entity_file(
    file: &mut File,
    config: &Config,
    external_entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
) -> Result<Vec<EntityValue>> {
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
                    Err(Error::NomError(nom::error::Error::new(
                        "parse_external_ent_file: Expected a Document, but found None. Prolog not parsed.".to_string(), nom::error::ErrorKind::Fail,
                    )))
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
                    Err(Error::NomError(nom::error::Error::new(
                        "parse_external_ent_file: Expected a MarkupDeclaration, but found None.".to_string(), nom::error::ErrorKind::Fail,
                    )))
                }
            },
        ),
        map(
            |i| Document::parse_content(i, &external_entity_references,EntitySource::External),
            |doc| {
                vec![EntityValue::Document(doc)]
            },
        ),
    ))(input)
    .map_err(|err| match err {
        nom::Err::Error(_e) | nom::Err::Failure(_e) => Box::new(Error::NomError(nom::error::Error::new(
            input.to_string(), nom::error::ErrorKind::Fail,
        ))),        nom::Err::Incomplete(_) => Box::new(Error::NomError(nom::error::Error::new(
            "parse_external_ent_file: Incomplete input.".to_string(), nom::error::ErrorKind::Fail,
        ))),
    })?;
    Ok(entity_values)
}

// pub fn parse_directory(
//     path: &Path,
//     config: Config,
// ) -> Result<Vec<Result<Document, Error>>, IoError> {
//     let entries = fs::read_dir(path)?;
//     let results = entries
//         .par_bridge()
//         .filter_map(|entry_result| entry_result.ok()) // Handle entry_result as std::io::Result
//         .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("xml"))
//         .map(|entry| {
//             let mut file = File::open(entry.path())?;
//             parse_file(&mut file, config.clone())
//         })
//         .collect::<Vec<_>>();
//     Ok(results)
// }
