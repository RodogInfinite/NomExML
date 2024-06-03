//! This example demonstrates how to parse all elements that match the tag name.
//! Note: Beware that this consume the entire file into memory, so it's not recommended for large files.
//!
//! ```rust
//! use std::{cell::RefCell, collections::HashMap, fs::File, rc::Rc};
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
//!         &Rc::new(RefCell::new(HashMap::new())),
//!     )?;
//!     println!("{doc:?}");
//!     Ok(())
//! }
//! ```

use std::{cell::RefCell, collections::HashMap, fs::File, rc::Rc};

use nom_xml::{io::read_file, Document, Result};

fn main() -> Result<()> {
    let mut file = File::open("examples/TheExpanseSeries.xml")?;
    let data = read_file(&mut file)?;

    let (_, doc) = Document::parse_elements_by_tag_name(
        &data,
        "book",
        &None,
        &Rc::new(RefCell::new(HashMap::new())),
    )?;
    println!("{doc:?}");
    Ok(())
}
