//! This example demonstrates how to parse all elements that match the tag name.
//! Note: Beware that this consume the entire file into memory, so it's not recommended for large files.
//!
//! ```rust
//! use std::fs::File;
//!
//! use nom_xml::{ io::read_file, Document, Result};
//!
//! fn main() -> Result<()> {
//!     let mut file = File::open("examples/TheExpanseSeries.xml")?;
//!     let data = read_file(&mut file)?;
//!
//!     let (_, doc) = Document::parse_elements_by_tag_name(
//!         &data,
//!         "book",
//!         &None,
//!     )?;
//!     println!("{doc:?}");
//!     Ok(())
//! }
//! ```

use std::fs::File;

use nom_xml::{io::read_file, Document, Result};

fn main() -> Result<()> {
    let mut file = File::open("examples/TheExpanseSeries.xml")?;
    let data = read_file(&mut file)?;

    let (_, doc) = Document::parse_elements_by_tag_name(&data, "book", &None)?;
    println!("{doc:?}");
    Ok(())
}
