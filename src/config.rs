//! Config is currently for setting up the parser to allow or prevent external entity parsing.
//! It's important for preventing some security risks, but it's has not been thoroughly tested for all XML attacks.
//! It prevents parsing of external entities by default, warns the user if it's enabled, and asks the user to confirm proceeding with external parsing in the terminal.
use crate::{warnln, Result};

use std::io::Write;

#[derive(Clone, Default, Debug)]
pub struct Config {
    pub external_parse_config: ExternalEntityParseConfig,

    #[cfg(feature = "experimental")]
    pub targeted_parsing: Option<TargetedParsingConfig>,
}

#[derive(Clone, Default, Debug)]
pub struct ExternalEntityParseConfig {
    pub allow_ext_parse: bool,
    pub ignore_ext_parse_warning: bool,
    pub base_directory: Option<String>,
}

#[cfg(feature = "experimental")]
#[derive(Clone, Debug)]
pub struct TargetedParsingConfig {
    pub tag_name: String,
    pub parse_multiple: bool, // True to parse multiple elements, false for a single element
}

pub(crate) fn check_config(config: &Config) -> Result<()> {
    match config {
        Config {
            external_parse_config:
                ExternalEntityParseConfig {
                    allow_ext_parse: true,
                    ignore_ext_parse_warning: false,
                    ..
                },
        } => {
            warnln!("The configuration `{:?}` allows external entity parsing which might expose the system to an XML External Entity (XXE) attack.\nThis crate makes no guarantees for security in this regard so make sure you trust your sources.\nVerification of all `.ent` files is strongly recommended.", config);

            loop {
                print!("Do you wish to proceed? [y/n]: ");
                std::io::stdout().flush().unwrap();

                let mut decision = String::new();
                std::io::stdin().read_line(&mut decision).unwrap();

                match decision.trim().to_lowercase().as_str() {
                    "y" | "Y" | "yes" => break,
                    "n" | "N" | "no" => {
                        return Err(nom::Err::Error(
                            "User decided to stop due to potential XXE attack",
                        )
                        .into());
                    }
                    _ => eprintln!("Invalid input. Please type 'y' or 'n'"),
                }
            }
        }
        Config {
            external_parse_config:
                ExternalEntityParseConfig {
                    allow_ext_parse: false,
                    ignore_ext_parse_warning: true,
                    ..
                },
        } => {
            warnln!("The configuration `{:?}` may allow for unexpected parsing if `allow_ext_parse` is changed to true in the future", config);
        }
        _ => (),
    }
    Ok(())
}
