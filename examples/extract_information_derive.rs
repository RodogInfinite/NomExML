//! This example demonstrates how to extract `Document` data into a struct for easy access with the UpdateFields macro.

use std::fs::File;

use nom_xml::{
    attribute::{Attribute, AttributeValue},
    io::read_file,
    tag::Tag,
    Document, Result, UpdateField,
};
use nom_xml_derive::UpdateFields;

#[derive(Debug, Default, Clone, UpdateFields)]
struct Book {
    #[update(from_attribute)]
    isbn: String,
    author: Author,
    title: String,
    genre: String,
    series_number: u8,
    description: String,
}

#[derive(Debug, Default, Clone, UpdateFields)]
struct Author {
    pen_name: String,
    //authors: Vec<AuthorName>,
}
#[derive(Debug, Default, Clone, UpdateFields)]
struct AuthorName {
    first_name: String,
    last_name: String,
}
fn main() -> Result<()> {
    let mut file = File::open("examples/TheExpanseSeries.xml")?;
    let data = read_file(&mut file)?;
    let (_, doc) = Document::parse_element_by_tag_name(&data, "book", &None)?;
    let mut book = Book::default();
    doc.iter_with_depth(0)
        .filter_map(|record| {
            if let Document::Element(tag, inner_doc, _) = record {
                Some((tag, inner_doc))
            } else {
                None
            }
        })
        .for_each(|(tag, inner_doc)| book.update_field(tag, inner_doc));
    println!("{book:#?}");
    Ok(())
}
