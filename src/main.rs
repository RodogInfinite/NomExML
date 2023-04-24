use std::fs::File;

use nomexml::{Element, Namespace, Tag};

fn main() {
    let mut file = File::open("data/test.xml").unwrap();

    let mut buffer = String::new();
    let result = Element::parse_file(&mut file, &mut buffer);

    match result {
        Ok((_rest, element)) => {
            println!("Parsed XML: {:?}", element);
        }
        Err(e) => {
            eprintln!("Error parsing XML: {:?}", e);
        }
    }
}
