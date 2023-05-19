use std::{error::Error, fs::File};

use nomexml::{parse_file, read_file, Document};

fn main() -> Result<(), Box<dyn Error>> {
    let input = "<!DOCTYPE doc [
        <!ELEMENT doc (#PCDATA)>
        ]>
        <doc></doc >
        
        ";

    let (_,doc) = Document::parse_xml_str(input)?;
    println!("DOC:\n{doc:?}");
    Ok(())
}
