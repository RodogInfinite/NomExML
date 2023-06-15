use std::{borrow::Cow, error::Error, fs::File};

use nom_xml::{io::parse_file, AsOrderedMap, Document, Name};

fn test_data_file<'a>(file_name: &str) -> Result<Document<'a>, Box<dyn Error>> {
    let mut file = File::open(format!("data/{file_name}.xml"))?;
    let document = parse_file(&mut file)?;
    Ok(document)
}

fn extract_duplicate_subtags(
    doc: &Document,
    outer_tag: &str,
    inner_tag: &str,
) -> Result<(), Box<dyn Error>> {
    doc.extract(&Name {
        prefix: None,
        local_part: Cow::Owned(outer_tag.into()),
    })?
    .iter()
    .for_each(|doc| {
        let extracted = doc
            .extract(&Name {
                prefix: None,
                local_part: Cow::Owned(inner_tag.into()),
            })
            .as_indexed_map();
        println!("Extracted: {:?}", extracted);
    });
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let doc = test_data_file("test_all")?;
    extract_duplicate_subtags(&doc, "X", "Y")?;
    Ok(())
}
