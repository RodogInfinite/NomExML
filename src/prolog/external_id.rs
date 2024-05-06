use std::{cell::RefCell, collections::HashMap, fs::File, rc::Rc};

use crate::{
    error::CustomError, io::parse_external_entity_file, parse::Parse, prolog::subset::Subset,
    Config, Document, ExternalEntityParseConfig, Name,
};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::map,
    sequence::{delimited, tuple},
    Err, IResult,
};

use super::{
    id::ID,
    subset::entity::{entity_value::EntityValue, EntitySource},
};

#[derive(Clone, Debug, PartialEq)]
pub enum ExternalID {
    System(String),
    Public {
        pubid: String,
        system_identifier: Box<ExternalID>, // Box<ExternalID::System>
    },
}

impl<'a> Parse<'a> for ExternalID {
    type Args = ();
    type Output = IResult<&'a str, Self>;
    //[75] ExternalID ::= 'SYSTEM' S SystemLiteral | 'PUBLIC' S PubidLiteral S SystemLiteral
    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
        alt((Self::parse_system, Self::parse_public))(input)
    }
}

impl ExternalID {
    fn parse_system(input: &str) -> IResult<&str, ExternalID> {
        map(
            tuple((
                tag("SYSTEM"),
                Self::parse_multispace1,
                Self::parse_system_literal,
            )),
            |(_system, _whitespace, system_literal)| ExternalID::System(system_literal),
        )(input)
    }

    fn parse_public(input: &str) -> IResult<&str, ExternalID> {
        map(
            tuple((
                tag("PUBLIC"),
                Self::parse_multispace1,
                ID::parse_public_id_literal,
                Self::parse_multispace1,
                Self::parse_system_literal,
            )),
            |(_public, _whitespace1, pubid_literal, _whitespace2, system_literal)| {
                ExternalID::Public {
                    pubid: pubid_literal,
                    system_identifier: Box::new(ExternalID::System(system_literal)),
                }
            },
        )(input)
    }

    // [11] SystemLiteral ::= ('"' [^"]* '"') | ("'" [^']* "'")
    fn parse_system_literal(input: &str) -> IResult<&str, String> {
        map(
            alt((
                delimited(tag("\""), is_not("\""), tag("\"")),
                delimited(tag("'"), is_not("'"), tag("'")),
            )),
            |s: &str| s.to_string(),
        )(input)
    }

    pub fn get_external_entity_from_id(
        &self,
        input: &str,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        config: Config,
    ) -> Result<(), CustomError> {
        // Check if external parsing is allowed and if a base directory is provided
        if let Config {
            external_parse_config:
                ExternalEntityParseConfig {
                    allow_ext_parse: true,
                    base_directory,
                    ..
                },
        } = &config
        {
            // Process only if ExternalID is of type System
            if let ExternalID::System(system_identifier) = self {
                // Construct the file path based on base_directory
                let file_path = base_directory.as_ref().map_or_else(
                    || system_identifier.clone(),
                    |base| format!("{}/{}", base, system_identifier),
                );
                dbg!(&file_path);
                // Attempt to open the file at the constructed path
                match File::open(file_path) {
                    Ok(mut file) => {
                        // Parse the external entity file
                        match parse_external_entity_file(
                            &mut file,
                            &config,
                            entity_references.clone(),
                        )
                        .as_deref()
                        {
                            Ok(entities) => {
                                dbg!(&entities);
                                dbg!(&input);
                                let (input, (subset, _whitespace1, _close_tag, _whitespace2)) =
                                    tuple((
                                        |i| {
                                            Subset::parse(
                                                i,
                                                (
                                                    entity_references.clone(),
                                                    config.clone(),
                                                    EntitySource::External,
                                                ),
                                            )
                                        },
                                        Self::parse_multispace0,
                                        tag(">"),
                                        Self::parse_multispace0,
                                    ))(input)?;
                                dbg!(&subset);
                                Ok(())
                            }
                            _ => Err(nom::Err::Error(nom::error::Error::new(
                                "Failed to match [entity] from `parse_external_entity_file`",
                                nom::error::ErrorKind::Fail,
                            ))
                            .into()),
                        }
                    }
                    Err(e) => Err(CustomError::from(e)),
                }
            } else {
                Err(nom::Err::Error(nom::error::Error::new(
                    "Only ExternalID::System is supported for direct parsing",
                    nom::error::ErrorKind::Fail,
                ))
                .into())
            }
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                "External parsing is disabled in the configuration",
                nom::error::ErrorKind::Fail,
            ))
            .into())
        }
    }
}
