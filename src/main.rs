use std::{error::Error, fs::File, path::Path};

use nomexml::{parse_file, Document};

fn main() -> Result<(), Box<dyn Error>> {
    let input = "<root><inner_tag1>inner_tag1 content</inner_tag1><inner_tag2>2</inner_tag2><tst:inner_tag3>3</tst:inner_tag3><tst:inner_tag4><inner_inner_tag1>inner_inner_tag1 content</inner_inner_tag1><header>header contents</header><inner_inner_tag1>inner_inner_tag1 content2</inner_inner_tag1><inner_inner_tag2><inner_inner_inner_tag1>inner_inner_inner_tag1 content</inner_inner_inner_tag1></inner_inner_tag2></tst:inner_tag4></root>";

    let (_, result) = Document::parse_tag_contents(input, "tst:inner_tag4").unwrap();
    println!("{result:?}");
    Ok(())
}
