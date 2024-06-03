//! This example demonstrates how to extract `Document` data into a struct for easy access.
//!
//!
//! ```rust
//! use std::fs::File;
//!
//! use nom_xml::{
//!     attribute::{Attribute, AttributeValue},
//!     io::read_file,
//!     tag::Tag,
//!     Document, Result,
//! };
//!
//! #[derive(Debug, Default)]
//! struct Book {
//!     isbn: String,
//!     author: String,
//!     title: String,
//!     genre: String,
//!     description: String,
//! }
//!
//! impl Book {
//!     fn update_field(&mut self, tag: &Tag, doc: &Document) {
//!         let field_name = &tag.name.local_part;
//!         if let Some(attributes_vec) = &tag.attributes {
//!             if let Attribute::Instance {
//!                 name,
//!                 value: AttributeValue::Value(attr_val),
//!             } = attributes_vec.get(0).unwrap()
//!             {
//!                 if name.local_part == "isbn" {
//!                     self.isbn = attr_val.to_string();
//!                 }
//!             }
//!         }
//!         if let Document::Nested(_) = &doc {
//!             doc.iter_with_depth(1).for_each(|record| {
//!                 if let Document::Element(tag, inner_doc, _) = record {
//!                     self.update_field(tag, inner_doc);
//!                 } else {
//!                     eprintln!("Unknown field: {record:#?}");
//!                 }

//!             });
//!         } else if let Document::Content(Some(value)) = &doc {
//!             match field_name.as_str() {
//!                 "author" => {
//!                     self.author = value.to_string();
//!                 }
//!                 "title" => {
//!                     self.title = value.to_string();
//!                 }
//!                 "genre" => {
//!                     self.genre = value.to_string();
//!                 }
//!                 "description" => {
//!                     self.description = value.to_string();
//!                 }
//!                 e => {
//!                     eprintln!("Unknown field: {}", e);
//!                 }
//!             }
//!         } else {
//!             eprintln!("Content is missing");
//!         }
//!     }
//! }
//!
//! fn main() -> Result<()> {
//!     let mut file = File::open("examples/TheExpanseSeries.xml")?;
//!     let data = read_file(&mut file)?;
//!     let (_, doc) = Document::parse_element_by_tag_name(
//!         &data,
//!         "book",
//!         &None,
//!     )?;
//!     let mut book = Book::default();
//!     doc.iter_with_depth(0)
//!         .filter_map(|record| {
//!             if let Document::Element(tag, inner_doc, _) = record {
//!                 Some((tag, inner_doc))
//!             } else {
//!                 None
//!             }
//!         })
//!         .for_each(|(tag, inner_doc)| book.update_field(tag, &inner_doc));
//!     println!("{book:#?}");
//!     Ok(())
//! }
//!
//! ```
//!

use std::fs::File;

use nom_xml::{
    attribute::{Attribute, AttributeValue},
    io::read_file,
    tag::Tag,
    Document, Result,
};

#[derive(Debug, Default)]
struct Book {
    isbn: String,
    author: String,
    title: String,
    genre: String,
    description: String,
}
//TODO: make nom-xml-derive proc macro crate to generate this code if possible
impl Book {
    fn update_field(&mut self, tag: &Tag, doc: &Document) {
        let field_name = &tag.name.local_part;
        if let Some(attributes_vec) = &tag.attributes {
            if let Attribute::Instance {
                name,
                value: AttributeValue::Value(attr_val),
            } = attributes_vec.get(0).unwrap()
            {
                if name.local_part == "isbn" {
                    self.isbn = attr_val.to_string();
                }
            }
        }
        if let Document::Nested(_) = &doc {
            doc.iter_with_depth(1).for_each(|record| {
                if let Document::Element(tag, inner_doc, _) = record {
                    self.update_field(tag, inner_doc);
                } else {
                    eprintln!("Unknown field: {record:#?}");
                }
            });
        } else if let Document::Content(Some(value)) = &doc {
            match field_name.as_str() {
                "author" => {
                    self.author = value.to_string();
                }
                "title" => {
                    self.title = value.to_string();
                }
                "genre" => {
                    self.genre = value.to_string();
                }
                "description" => {
                    self.description = value.to_string();
                }
                e => {
                    eprintln!("Unknown field: {}", e);
                }
            }
        } else {
            eprintln!("Content is missing");
        }
    }
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
        .for_each(|(tag, inner_doc)| book.update_field(tag, &inner_doc));
    println!("{book:#?}");
    Ok(())
}
