//! This example demonstrates how to parse the first element that matches the given tag name.

use std::fs::File;

use nom_xml::{io::read_file, Document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("examples/TheExpanseSeries.xml")?;
    let data = read_file(&mut file)?;
    let (_, doc) = Document::parse_element_by_tag_name(&data, "book", &None)?;
    println!("{doc:?}");
    Ok(())
}
