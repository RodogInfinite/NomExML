use std::fs::File;

use nom_xml::{
    attribute::{Attribute, AttributeValue},
    io::read_file,
    tag::Tag,
    Document, DocumentIteratorExt, UpdateFields,
};
use nom_xml_derive::ExtractFields;

#[derive(Clone, Debug, Default, ExtractFields, PartialEq)]
struct VecTests {
    unsigned8: Vec<u8>,
    unsigned16: Vec<u16>,
    #[extract(from_tag = "unsigned32swap")]
    unsigned32: Vec<u32>,
    unsigned64: Vec<u64>,
    unsigned128: Vec<u128>,
    signed8: Vec<i8>,
    signed16: Vec<i16>,
    signed32: Vec<i32>,
    signed64: Vec<i64>,
    signed128: Vec<i128>,
    float32: Vec<f32>,
    float64: Vec<f64>,
    string: Vec<String>,
    #[extract(from_tag = "string2swap")]
    string2: Vec<String>,
}

fn run() -> Result<VecTests, Box<dyn std::error::Error>> {
    let mut file = File::open("tests/derived_tests/data/vec_test.xml")?;
    let data = read_file(&mut file)?;
    let (_, doc) = Document::parse_element_by_tag_name(&data, "vectest", &None)?;
    let mut vectest = VecTests::default();

    doc.iter_with_depth(0)
        .filter_map(|element| {
            if let Document::Element(tag, inner_doc, _) = element {
                Some((tag, inner_doc))
            } else {
                None
            }
        })
        .try_for_each(|(tag, inner_doc)| vectest.update_field(tag, inner_doc))
        .map_err(|e| {
            println!("Error updating field: {}", e);
            e
        })?;
    println!("{:#?}", vectest);
    Ok(vectest)
}

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let vec_numerical_tests: VecTests = run()?;

    let expected_vec_numerical_tests = VecTests {
        unsigned8: vec![10, 1],
        unsigned16: vec![20, 2],
        unsigned32: vec![30, 3],
        unsigned64: vec![4, 40],
        unsigned128: vec![50, 5],
        signed8: vec![-10, -1],
        signed16: vec![-2, -20],
        signed32: vec![-30, -3],
        signed64: vec![-4, -40],
        signed128: vec![-50, -5],
        float32: vec![1.1, 11.1],
        float64: vec![22.2, 2.2],
        string: vec!["string".to_string(), "another string".to_string()],
        string2: vec!["extracted from string2swap".to_string()],
    };

    assert_eq!(vec_numerical_tests, expected_vec_numerical_tests);

    Ok(())
}
