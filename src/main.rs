use std::{error::Error, fs::File, path::Path};

use nomexml::{parse_file, Document};

fn main() -> Result<(), Box<dyn Error>> {
    let input = "<Root><Inner_Tag1>inner_tag1 content</Inner_Tag1><inner_tag2>2</inner_tag2><tst:inner_tag3>3</tst:inner_tag3><tst:Inner_Tag4><inner_inner_tag1>inner_inner_tag1 content</inner_inner_tag1><header>header contents</header><inner_inner_tag1>inner_inner_tag1 content2</inner_inner_tag1><inner_inner_tag2><inner_inner_inner_tag1>inner_inner_inner_tag1 content</inner_inner_inner_tag1></inner_inner_tag2></tst:Inner_Tag4></Root>";
    

    let (_, document) = Document::parse_xml_str(input)?;

    let res = document.get_tags("inner_inner_tag1");
    println!("{res:?}");

    Ok(())
}

