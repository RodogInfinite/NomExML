use crate::attribute::Attribute;
use crate::parse::Parse;
use crate::prolog::subset::entity::entity_value::EntityValue;
use crate::prolog::subset::Subset;
use crate::Name;

use crate::config::Config;
use crate::prolog::subset::entity::EntitySource;
use crate::prolog::textdecl::TextDecl;
use crate::reference::Reference;
use crate::{error::Error, Document};
use encoding_rs::*;
use nom::branch::alt;
use nom::combinator::{map, opt};

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
pub fn parse_entire_file(
    file: &mut File,
    config: &Config,
) -> Result<Document, Box<dyn std::error::Error>> {
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

pub(crate) fn parse_external_entity_file(
    file: &mut File,
    config: &Config,
    external_entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
) -> Result<(Vec<EntityValue>, Option<Vec<Subset>>), Box<dyn std::error::Error>> {
    let mut data = read_file(file)?;
    data = data.replace("\r\n", "\n").replace('\r', "\n");
    let (input, _text_decl) = opt(|i| TextDecl::parse(i, ()))(data.as_str())?;
    //TODO: handle the text_decl such that if the encoding being used to parse the file is different, then the encoding is handled accordingly, i.e file being parsed again with the proper decoding
    let args = (
        external_entity_references.clone(),
        config,
        EntitySource::External,
    );
    let (input, subsets) = match Subset::parse(input, args) {
        Ok((input, subsets)) => {
            if subsets.is_empty() {
                (input, None)
            } else {
                (input, Some(subsets))
            }
        }
        _ => (input, None),
    };

    let (_, entity_values) = alt((
        many1(map(
            |i| Reference::parse(i, EntitySource::External),
            EntityValue::Reference,
        )),
        map(
            |i| Document::parse_content(i, &external_entity_references, EntitySource::External),
            |doc| vec![EntityValue::Document(doc)],
        ),
    ))(input)
    .map_err(|err| match err {
        nom::Err::Error(_e) | nom::Err::Failure(_e) => Box::new(Error::NomError(
            nom::error::Error::new(input.to_string(), nom::error::ErrorKind::Fail),
        )),
        nom::Err::Incomplete(_) => Box::new(Error::NomError(nom::error::Error::new(
            "parse_external_ent_file: Incomplete input.".to_string(),
            nom::error::ErrorKind::Fail,
        ))),
    })?;
    Ok((entity_values, subsets))
}
