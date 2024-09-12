//! This example demonstrates how to extract `Document` data into a struct for easy access with the ExtractFields macro.

use std::fs::File;

use nom_xml::{
    attribute::{Attribute, AttributeValue},
    io::read_file,
    tag::Tag,
    Document, DocumentIteratorExt, UpdateFields,
};
use nom_xml_derive::ExtractFields;

#[derive(Clone, Debug, Default, ExtractFields, PartialEq)]
struct Books {
    catalog: Option<Vec<Book>>,
}

#[derive(Clone, Debug, Default, ExtractFields, PartialEq)]
struct Book {
    #[extract(from_attribute)]
    isbn: Option<String>,
    authored_by: Option<AuthoredBy>,
    title: Option<String>,
    genre: Option<String>,
    #[extract(from_tag = "type")]
    ty: Option<String>,
    series_number: Option<u8>,
    description: Option<String>,
}

#[derive(Clone, Debug, Default, ExtractFields, PartialEq)]
struct AuthoredBy {
    pen_name: Option<String>,
    authors: Option<Vec<AuthorName>>,
}

#[derive(Clone, Debug, Default, ExtractFields, PartialEq)]
struct AuthorName {
    first_name: Option<String>,
    last_name: Option<String>,
}

fn run() -> Result<Books, Box<dyn std::error::Error>> {
    let mut file = File::open(
        "tests/derived_tests/data/TheExpanseSeries_outer_optionals_vec_missing_data.xml",
    )?;
    let data = read_file(&mut file)?;
    let (_, doc) = Document::parse_element_by_tag_name(&data, "catalog", &None)?;
    let mut books = Books::default();

    doc.iter_with_depth(0)
        .filter_map(|element| {
            if let Document::Element(tag, inner_doc, _) = element {
                Some((tag, inner_doc))
            } else {
                None
            }
        })
        .try_for_each(|(tag, inner_doc)| books.update_field(tag, inner_doc))
        .map_err(|e| {
            println!("Error updating field: {}", e);
            e
        })?;

    Ok(books)
}

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let books: Books = run()?;

    let expected_books = Books {
        catalog: Some(vec![
            Book {
                isbn: Some("978-0316129084".to_string()),
                authored_by: Some(AuthoredBy {
                    pen_name: Some("James S.A. Corey".to_string()),
                    authors: Some(vec![
                        AuthorName {
                            first_name: Some("Daniel".to_string()),
                            last_name: Some("Abraham".to_string()),
                        },
                        AuthorName {
                            first_name: Some("Ty".to_string()),
                            last_name: Some("Franck".to_string()),
                        },
                    ]),
                }),
                title: Some("Leviathan Wakes".to_string()),
                genre: Some("Science Fiction".to_string()),
                ty: Some("Novel".to_string()),
                series_number: Some(1),
                description: Some("The first book in the Expanse series".to_string()),
            },
            Book {
                isbn: None,
                authored_by: None,
                title: None,
                genre: None,
                ty: None,
                series_number: None,
                description: None,
            },
            Book {
                isbn: Some("978-0316129077".to_string()),
                authored_by: Some(AuthoredBy {
                    pen_name: Some("James S.A. Corey".to_string()),
                    authors: None,
                }),
                title: Some("Abaddon's Gate".to_string()),
                genre: Some("Science Fiction".to_string()),
                ty: Some("Novel".to_string()),
                series_number: Some(3),
                description: Some("The third book in the Expanse series".to_string()),
            },
            Book {
                isbn: Some("978-0316334686".to_string()),
                authored_by: Some(AuthoredBy {
                    pen_name: None,
                    authors: Some(vec![
                        AuthorName {
                            first_name: Some("Daniel".to_string()),
                            last_name: Some("Abraham".to_string()),
                        },
                        AuthorName {
                            first_name: Some("Ty".to_string()),
                            last_name: Some("Franck".to_string()),
                        },
                    ]),
                }),
                title: None,
                genre: None,
                ty: None,
                series_number: None,
                description: None,
            },
            Book {
                isbn: Some("978-0316334716".to_string()),
                authored_by: Some(AuthoredBy {
                    pen_name: Some("James S.A. Corey".to_string()),
                    authors: Some(vec![
                        AuthorName {
                            first_name: Some("Daniel".to_string()),
                            last_name: Some("Abraham".to_string()),
                        },
                        AuthorName {
                            first_name: Some("Ty".to_string()),
                            last_name: Some("Franck".to_string()),
                        },
                    ]),
                }),
                title: Some("Nemesis Games".to_string()),
                genre: Some("Science Fiction".to_string()),
                ty: Some("Novel".to_string()),
                series_number: Some(5),
                description: Some("The fifth book in the Expanse series".to_string()),
            },
            Book {
                isbn: Some("978-0316334747".to_string()),
                authored_by: Some(AuthoredBy {
                    pen_name: Some("James S.A. Corey".to_string()),
                    authors: Some(vec![
                        AuthorName {
                            first_name: Some("Daniel".to_string()),
                            last_name: Some("Abraham".to_string()),
                        },
                        AuthorName {
                            first_name: Some("Ty".to_string()),
                            last_name: Some("Franck".to_string()),
                        },
                    ]),
                }),
                title: Some("Babylon's Ashes".to_string()),
                genre: Some("Science Fiction".to_string()),
                ty: Some("Novel".to_string()),
                series_number: Some(6),
                description: Some("The sixth book in the Expanse series".to_string()),
            },
            Book {
                isbn: Some("978-0316332828".to_string()),
                authored_by: Some(AuthoredBy {
                    pen_name: Some("James S.A. Corey".to_string()),
                    authors: Some(vec![
                        AuthorName {
                            first_name: Some("Daniel".to_string()),
                            last_name: Some("Abraham".to_string()),
                        },
                        AuthorName {
                            first_name: Some("Ty".to_string()),
                            last_name: Some("Franck".to_string()),
                        },
                    ]),
                }),
                title: Some("Persepolis Rising".to_string()),
                genre: Some("Science Fiction".to_string()),
                ty: Some("Novel".to_string()),
                series_number: Some(7),
                description: Some("The seventh book in the Expanse series".to_string()),
            },
            Book {
                isbn: Some("978-0316332873".to_string()),
                authored_by: Some(AuthoredBy {
                    pen_name: Some("James S.A. Corey".to_string()),
                    authors: Some(vec![
                        AuthorName {
                            first_name: Some("Daniel".to_string()),
                            last_name: Some("Abraham".to_string()),
                        },
                        AuthorName {
                            first_name: Some("Ty".to_string()),
                            last_name: Some("Franck".to_string()),
                        },
                    ]),
                }),
                title: Some("Tiamat's Wrath".to_string()),
                genre: Some("Science Fiction".to_string()),
                ty: Some("Novel".to_string()),
                series_number: Some(8),
                description: Some("The eighth book in the Expanse series".to_string()),
            },
            Book {
                isbn: Some("978-0316332910".to_string()),
                authored_by: Some(AuthoredBy {
                    pen_name: Some("James S.A. Corey".to_string()),
                    authors: Some(vec![
                        AuthorName {
                            first_name: Some("Daniel".to_string()),
                            last_name: Some("Abraham".to_string()),
                        },
                        AuthorName {
                            first_name: Some("Ty".to_string()),
                            last_name: Some("Franck".to_string()),
                        },
                    ]),
                }),
                title: Some("Leviathan Falls".to_string()),
                genre: Some("Science Fiction".to_string()),
                ty: Some("Novel".to_string()),
                series_number: Some(9),
                description: Some("The ninth book in the Expanse series".to_string()),
            },
        ]),
    };

    assert_eq!(books, expected_books);

    Ok(())
}
