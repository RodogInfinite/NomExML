use std::{error::Error, fs::File};

use nom_xml::{io::parse_file, Document};

fn test_data_file<'a>(file_name: &str) -> Result<Document<'a>, Box<dyn Error>> {
    let mut file = File::open(format!("data/{file_name}.xml"))?;
    let document = parse_file(&mut file)?;
    Ok(document)
}

//let doc = test_data_file("test_xml_decl")?;
//let doc = test_data_file("test_product_obs")?;
//let doc = test_data_file("test_identification_area")?;
// let doc = test_data_file("observation_area/discipline_area")?;

fn main() -> Result<(), Box<dyn Error>> {
    // let input = "<root>
    // <inner_tag1>inner_tag1 content</inner_tag1>
    // <inner_tag2>2</inner_tag2>
    // <xmlns:inner_tag3>3</xmlns:inner_tag3>
    // <xmlns:inner_tag4>
    // <inner_inner_tag1>inner_inner_tag1 content</inner_inner_tag1>
    // <header>header contents</header>
    // <inner_inner_tag1>inner_inner_tag1 content2</inner_inner_tag1><inner_inner_tag2><inner_inner_inner_tag1>inner_inner_inner_tag1 content</inner_inner_inner_tag1></inner_inner_tag2></xmlns:inner_tag4></root>";

    // let (_, doc) = Document::parse_xml_str(input)?;

    let doc = test_data_file("test_all")?;
    println!("\n\nDOC:\n{doc:?}");

    Ok(())
}
