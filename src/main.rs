use std::error::Error;

use nomexml::Document;

fn main() -> Result<(), Box<dyn Error>> {
    let input = "<!DOCTYPE doc [
        <!ELEMENT doc (#PCDATA)>
        ]>
        <doc>&amp;&lt;&gt;&quot;&apos;</doc>
        
        ";

    let (_, doc) = Document::parse_xml_str(input)?;
    println!("DOC:\n{doc:?}");

    Ok(())
}
