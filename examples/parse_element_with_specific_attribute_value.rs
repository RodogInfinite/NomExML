//! This example demonstrates how to parse a single element with a specific attribute value as a `Document`.
//! Note that this will only parse the first element that matches the all of the criteria.
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
//!         &Some(vec![Attribute::new("isbn", "978-0316332910")]),
//!         &Rc::new(RefCell::new(HashMap::new())), // This is currently needed to track external entities. It's not used in this example, but it's mandatory to be included
//!     )?;
//!     println!("{doc:?}");
//!     Ok(())
//! }
//! ```

use std::{cell::RefCell, collections::HashMap, fs::File, rc::Rc};

use nom_xml::{attribute::Attribute, io::read_file, Document, Result};

fn main() -> Result<()> {
    let mut file = File::open("examples/TheExpanseSeries.xml")?;
    let data = read_file(&mut file)?;
    let (_, doc) = Document::parse_element_by_tag_name(
        &data,
        "book",
        &Some(vec![Attribute::new("isbn", "978-0316332910")]),
        &Rc::new(RefCell::new(HashMap::new())), // This is currently needed to track external entities. It's not used in this example, but it's mandatory to be included
    )?;
    println!("{doc:?}");
    Ok(())
}
