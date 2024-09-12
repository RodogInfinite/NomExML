//! This example demonstrates how to extract `Document` data into a struct for easy access with the ExtractFields macro.

use nom_xml::{
    attribute::{Attribute, AttributeValue},
    io::read_file,
    tag::Tag,
    Document, UpdateFields, DocumentIteratorExt
};
use nom_xml_derive::ExtractFields;
use std::fs::File;
use std::prelude::rust_2021::*;

#[derive(Debug, Default, Clone, ExtractFields, PartialEq)]
struct Book {
    #[extract(from_attribute)]
    isbn: String,
    authored_by: Option<AuthoredBy>,
    title: String,
    genre: String,
    #[extract(from_tag = "type")]
    ty: String,
    series_number: u8,
    description: Option<String>,
}

#[derive(Debug, Default, Clone, ExtractFields, PartialEq)]
struct AuthoredBy {
    pen_name: Option<String>,
    authors: Option<Vec<AuthorName>>,
}

#[derive(Debug, Default, Clone, ExtractFields, PartialEq)]
struct AuthorName {
    first_name: String,
    last_name: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("examples/TheExpanseSeries.xml")?;
    let data = read_file(&mut file)?;
    let (_, doc) = Document::parse_element_by_tag_name(&data, "book", &None)?;
    let mut book = Book::default();
    // doc.iter_with_depth(0)
    //     .filter_map(|record| {
    //         if let Document::Element(tag, inner_doc, _) = record {
    //             Some((tag, inner_doc))
    //         } else {
    //             None
    //         }
    //     })
    //     .try_for_each(|(tag, inner_doc)| book.update_field(tag, inner_doc))?;
    // book.update_attribute_fields(&doc);
    book.update_fields(&doc)?;
    println!("{book:#?}");
    Ok(())
}
