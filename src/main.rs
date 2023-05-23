use std::error::Error;

use nomexml::Document;

fn main() -> Result<(), Box<dyn Error>> {
    let input = "<!DOCTYPE doc [
        
        <!ATTLIST doc a1 CDATA #IMPLIED>
        ]>
        <doc a1=\"v1\"></doc>
        
        ";

    let (_, doc) = Document::parse_xml_str(input)?;
    println!("DOC:\n{doc:?}");

    Ok(())
}
