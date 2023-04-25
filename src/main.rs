use std::{error::Error, fs::File, path::Path};

use nomexml::{Element};

fn main() -> Result<(), Box<dyn Error>> {
    // let mut file = File::open("data/xmltest/valid/sa/001.xml")?;
    // //let mut dir = Path::new("data/xmltest/valid/sa");

    // //let elements = parse_directory(dir)?;
    //  let element = parse_file(&mut file)?;

    //  println!("Parsed XML: {:?}", element);

    let input = "<root><inner_tag1>inner_tag1 content</inner_tag1><inner_tag2>2</inner_tag2><tst:inner_tag3>3</tst:inner_tag3><tst:inner_tag4><inner_inner_tag1>inner_inner_tag1 content</inner_inner_tag1><header>header contents</header><inner_inner_tag1>inner_inner_tag1 content2</inner_inner_tag1><inner_inner_tag2><inner_inner_inner_tag1>inner_inner_inner_tag1 content</inner_inner_inner_tag1></inner_inner_tag2></tst:inner_tag4></root>";
    let (tail, result) = Element::parse_xml_str(input).unwrap();
    println!("result:\n{result:?}");
    println!("tail: {tail:?}");

    Ok(())
}
