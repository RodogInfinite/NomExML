//!This example demonstrates how to parse an entire XML file into a `Document` .
//!The `Document` is a representation of the XML document as a tree of Elements
//!Notice that it's a bit unweildy when displayed, but it's a good way to see the entire structure of the XML document.
//!See the `parse_first_matching_element.rs` example for a more detailed example of how to parse individual elements.
//!
//! Note: Beware that this consume the entire file into memory, so it's not recommended for large files.
//!
//! ```rust
//! use std::fs::File;
//!
//! use nom_xml::{io::parse_entire_file, Config, Result};
//!
//! fn main() -> Result<()> {
//!     let mut file = File::open("examples/TheExpanseSeries.xml")?;
//!     let doc = parse_entire_file(&mut file, Config::default())?;
//!
//!     println!("{doc:?}");
//!     Ok(())
//! }
//! ```

use std::fs::File;

use nom_xml::{io::parse_entire_file, Config, Result};

fn main() -> Result<()> {
    let mut file = File::open("examples/TheExpanseSeries.xml")?;
    let doc = parse_entire_file(&mut file, Config::default())?;

    println!("{doc:?}");
    Ok(())
}
