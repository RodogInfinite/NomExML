use std::error::Error;

use nomexml::document::Document;

fn main() -> Result<(), Box<dyn Error>> {
    let input = "<!DOCTYPE doc [
        <!ELEMENT doc (#PCDATA)>
        ]>
        <doc><?pi some data ?><?x?></doc>
        ";

    let (_, doc) = Document::parse_xml_str(input)?;
    println!("DOC:\n{doc:?}");

    Ok(())
}
