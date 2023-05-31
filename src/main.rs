use std::{error::Error, fs::File};

use nomxml::{document::Document, parse::Parse, parse_file, prolog::external_id::ExternalID};

fn test_valid_sa_file<'a>(file_number: &str) -> Result<Document<'a>, Box<dyn Error>> {
    let mut file = File::open(format!("tests/xmltest/valid/sa/{file_number}.xml"))?;
    let document = parse_file(&mut file)?;
    Ok(document)
}

fn main() -> Result<(), Box<dyn Error>> {
    let doc = test_valid_sa_file("023")?;
    println!("\n\nDOC:\n{doc:?}");

    Ok(())
}
