use std::{error::Error, fs::File};

use nomexml::{declaration::Declaration, parse_file, read_file, Document};

fn main() -> Result<(), Box<dyn Error>> {
    let input = "<!ATTLIST doc a1 CDATA #IMPLIED>";
    let (_, attlist) = Declaration::parse_attlist(input)?;
    println!("ATTLIST:\n{attlist:?}");

    let input = "<!DOCTYPE doc [
        
        <!ATTLIST doc a1 CDATA #IMPLIED>
        ]>
        <doc a1=\"v1\"></doc>
        
        ";

    let (_, doc) = Document::parse_xml_str(input)?;
    println!("DOC:\n{doc:?}");

    Ok(())
}
