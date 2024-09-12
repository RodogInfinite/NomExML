//! This example demonstrates how to extract `Document` data into a struct for easy access with the ExtractFields macro.

use std::fs::File;

use nom_xml::{
    attribute::{Attribute, AttributeValue},
    io::read_file,
    tag::Tag,
    Document, DocumentIteratorExt, UpdateFields,
};
use nom_xml_derive::ExtractFields;

#[derive(Debug, Default, Clone, ExtractFields)]
struct Books {
    catalog: Vec<Book>,
}

#[derive(Clone, Debug, Default, ExtractFields, PartialEq)]
struct Book {
    #[extract(from_attribute)]
    isbn: Option<String>,
    authored_by: AuthoredBy,
    title: Option<String>,
    genre: String,
    #[extract(from_tag = "type")]
    ty: String,
    series_number: u8,
    description: String,
}

#[derive(Debug, Default, Clone, ExtractFields, PartialEq)]
struct AuthoredBy {
    pen_name: String,
    authors: Vec<AuthorName>,
}

#[derive(Debug, Default, Clone, ExtractFields, PartialEq)]
struct AuthorName {
    first_name: String,
    last_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("examples/TheExpanseSeries.xml")?;
    let data = read_file(&mut file)?;
    let (_, doc) = Document::parse_element_by_tag_name(&data, "catalog", &None)?;
    let mut books = Books::default();

    doc.iter_with_depth(0)
        .filter_map(|element| {
            if let Document::Element(tag, inner_doc, _) = element {
                Some((tag, inner_doc))
            } else {
                None
            }
        })
        .try_for_each(|(tag, inner_doc)| books.update_field(tag, inner_doc))
        .map_err(|e| {
            println!("Error updating field: {}", e);
            e
        })?;

    println!("{books:#?}");
    Ok(())
}
