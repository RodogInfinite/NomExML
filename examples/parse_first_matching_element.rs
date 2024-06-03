//! This example demonstrates how to parse the first element that matches the given tag name.
//!
//!
//! ```rust
//! use std::fs::File;
//!
//! use nom_xml::{io::read_file, Document, Result};
//!
//! fn main() -> Result<()> {
//!     let mut file = File::open("examples/TheExpanseSeries.xml")?;
//!     let data = read_file(&mut file)?;
//!     let (_, doc) = Document::parse_element_by_tag_name(
//!         &data,
//!         "book",
//!         &None,
//!     )?;
//!     println!("{doc:?}");
//!     Ok(())
//! }
//! ```
//!

use std::fs::File;

use nom_xml::{io::read_file, Document, Result};

fn main() -> Result<()> {
    let mut file = File::open("examples/TheExpanseSeries.xml")?;
    let data = read_file(&mut file)?;
    let (_, doc) = Document::parse_element_by_tag_name(&data, "book", &None)?;
    println!("{doc:?}");
    Ok(())
}
