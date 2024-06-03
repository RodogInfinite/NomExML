//! This example demonstrates how to parse the first element that matches the given tag name.
//!
//!
//! ```rust
//! use std::{cell::RefCell, collections::HashMap, fs::File, rc::Rc};
//!
//! use nom_xml::{attribute::Attribute, io::read_file, Document, Result};
//!
//! fn main() -> Result<()> {
//!     let mut file = File::open("examples/TheExpanseSeries.xml")?;
//!     let data = read_file(&mut file)?;
//!     let (_, doc) = Document::parse_element_by_tag_name(
//!         &data,
//!         "book",
//!         &None,
//!         &Rc::new(RefCell::new(HashMap::new())),
//!     )?;
//!     println!("{doc:?}");
//!     Ok(())
//! }
//! ```
//!

use std::{cell::RefCell, collections::HashMap, fs::File, rc::Rc};

use nom_xml::{attribute::Attribute, io::read_file, Document, Result};

fn main() -> Result<()> {
    let mut file = File::open("examples/TheExpanseSeries.xml")?;
    let data = read_file(&mut file)?;
    let (_, doc) = Document::parse_element_by_tag_name(
        &data,
        "book",
        &None,
        &Rc::new(RefCell::new(HashMap::new())),
    )?;
    println!("{doc:?}");
    Ok(())
}
