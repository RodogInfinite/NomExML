//! This example demonstrates how to parse a single element with a specific attribute value as a `Document`.
//! Note that this will only parse the first element that matches the all of the criteria.

use std::fs::File;

use nom_xml::{attribute::Attribute, io::read_file, Document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("examples/TheExpanseSeries.xml")?;
    let data = read_file(&mut file)?;
    let (_, doc) = Document::parse_element_by_tag_name(
        &data,
        "book",
        &Some(vec![Attribute::new("isbn", "978-0316332910")]),
    )?;
    println!("{doc:?}");
    Ok(())
}
