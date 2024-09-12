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
    unsigned8: Option<Vec<u8>>,
    unsigned16: Option<Vec<u16>>,
    #[extract(from_tag = "unsigned32swap")]
    unsigned32: Option<Vec<u32>>,
    unsigned64: Option<Vec<u64>>,
    unsigned128: Option<Vec<u128>>,
    signed8: Option<Vec<i8>>,
    signed16: Option<Vec<i16>>,
    signed32: Option<Vec<i32>>,
    signed64: Option<Vec<i64>>,
    signed128: Option<Vec<i128>>,
    float32: Option<Vec<f32>>,
    float64: Option<Vec<f64>>,
    string: Option<Vec<String>>,
    #[extract(from_tag = "string2swap")]
    string2: Option<Vec<String>>,
}

fn run() -> Result<VecTests, Box<dyn std::error::Error>> {
    let mut file = File::open("tests/derived_tests/data/vec_test_missing.xml")?;
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
        unsigned8: Some(vec![10, 1]),
        unsigned16: Some(vec![20, 2]),
        unsigned32: None,
        unsigned64: Some(vec![4, 40]),
        unsigned128: Some(vec![50, 5]),
        signed8: Some(vec![-10, -1]),
        signed16: Some(vec![-2, -20]),
        signed32: Some(vec![-30, -3]),
        signed64: None,
        signed128: Some(vec![-50, -5]),
        float32: Some(vec![1.1, 11.1]),
        float64: Some(vec![22.2, 2.2]),
        string: Some(vec!["string".to_string(), "another string".to_string()]),
        string2: Some(vec!["extracted from string2swap".to_string()]),
    };

    assert_eq!(vec_numerical_tests, expected_vec_numerical_tests);

    Ok(())
}
