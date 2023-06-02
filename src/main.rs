use std::{error::Error, fs::File};

use nomxml::{
    document::Document,
    parse::Parse,
    parse_file,
    prolog::{doctype::DocType, external_id::ExternalID, xmldecl::XmlDecl},
};

fn test_valid_sa_file<'a>(file_number: &str) -> Result<Document<'a>, Box<dyn Error>> {
    let mut file = File::open(format!("tests/xmltest/valid/sa/{file_number}.xml"))?;
    let document = parse_file(&mut file)?;
    Ok(document)
}

fn test_data_file<'a>(file_name: &str) -> Result<Document<'a>, Box<dyn Error>> {
    let mut file = File::open(format!("data/{file_name}.xml"))?;
    let document = parse_file(&mut file)?;
    Ok(document)
}

fn main() -> Result<(), Box<dyn Error>> {
    //let input = "<!DOCTYPE greeting SYSTEM \"hello.dtd\">";
    //let (_, doc) = DocType::parse(input)?;
    //let (_, doc) = Document::parse_xml_str(input)?;
    //let doc = test_data_file("test_xml_decl")?;
    //let doc = test_data_file("test_product_obs")?;
    let doc = test_data_file("test_identification_area")?;
    println!("\n\nDOC:\n{doc:?}");

    Ok(())
}

//let (_, doc) = Document::parse_prolog(input)?;
// TODO: make let doc = test_valid_sa_file("023")?; convert the content <doc>&e;</doc> based on the parsed entity
//let doc = test_valid_sa_file("023")?;
