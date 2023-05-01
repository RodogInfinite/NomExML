use std::error::Error;

use nomexml::{parse_file, Document};

fn main() -> Result<(), Box<dyn Error>> {
    let input = "<!DOCTYPE doc [
        <!ELEMENT doc (#PCDATA)>
        ]>
        <doc ></doc>
        ";

    let (_, res) = Document::parse_xml_str(input)?;
    println!("{res:?}");

    Ok(())
}
