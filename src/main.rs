use nom_xml::{io::parse_file, AsOrderedMap, Config, ExternalEntityParseConfig, Name};
use std::fs::File;

fn main() {
    // Open the file and handle potential errors
    let file_path = "./data/test/097.xml";
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    // Parse the file
    let x = parse_file(
        &mut file,
        Config {
            external_parse_config: ExternalEntityParseConfig {
                allow_ext_parse: true,
                ignore_ext_parse_warning: true,
                base_directory: Some("tests/xmltest/valid/sa".into()),
            },
        },
    );
}
