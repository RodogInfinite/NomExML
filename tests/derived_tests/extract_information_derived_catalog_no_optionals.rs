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
    catalog: Vec<Book>,
}

#[derive(Clone, Debug, Default, ExtractFields, PartialEq)]
struct Book {
    #[extract(from_attribute)]
    isbn: String,
    authored_by: AuthoredBy,
    title: String,
    genre: String,
    #[extract(from_tag = "type")]
    ty: String,
    series_number: u8,
    description: String,
}

#[derive(Clone, Debug, Default, ExtractFields, PartialEq)]
struct AuthoredBy {
    pen_name: String,
    authors: Vec<AuthorName>,
}

#[derive(Clone, Debug, Default, ExtractFields, PartialEq)]
struct AuthorName {
    first_name: String,
    last_name: String,
}

fn run() -> Result<Books, Box<dyn std::error::Error>> {
    let mut file = File::open("examples/TheExpanseSeries.xml")?;
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

    // println!("{books:#?}");
    // Ok(())
}

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let books: Books = run()?;

    let expected_books = Books {
        catalog: vec![
            Book {
                isbn: "978-0316129084".to_string(),
                authored_by: AuthoredBy {
                    pen_name: "James S.A. Corey".to_string(),
                    authors: vec![
                        AuthorName {
                            first_name: "Daniel".to_string(),
                            last_name: "Abraham".to_string(),
                        },
                        AuthorName {
                            first_name: "Ty".to_string(),
                            last_name: "Franck".to_string(),
                        },
                    ],
                },
                title: "Leviathan Wakes".to_string(),
                genre: "Science Fiction".to_string(),
                ty: "Novel".to_string(),
                series_number: 1,
                description: "The first book in the Expanse series".to_string(),
            },
            Book {
                isbn: "978-0316129060".to_string(),
                authored_by: AuthoredBy {
                    pen_name: "James S.A. Corey".to_string(),
                    authors: vec![
                        AuthorName {
                            first_name: "Daniel".to_string(),
                            last_name: "Abraham".to_string(),
                        },
                        AuthorName {
                            first_name: "Ty".to_string(),
                            last_name: "Franck".to_string(),
                        },
                    ],
                },
                title: "Caliban's War".to_string(),
                genre: "Science Fiction".to_string(),
                ty: "Novel".to_string(),
                series_number: 2,
                description: "The second book in the Expanse series".to_string(),
            },
            Book {
                isbn: "978-0316129077".to_string(),
                authored_by: AuthoredBy {
                    pen_name: "James S.A. Corey".to_string(),
                    authors: vec![
                        AuthorName {
                            first_name: "Daniel".to_string(),
                            last_name: "Abraham".to_string(),
                        },
                        AuthorName {
                            first_name: "Ty".to_string(),
                            last_name: "Franck".to_string(),
                        },
                    ],
                },
                title: "Abaddon's Gate".to_string(),
                genre: "Science Fiction".to_string(),
                ty: "Novel".to_string(),
                series_number: 3,
                description: "The third book in the Expanse series".to_string(),
            },
            Book {
                isbn: "978-0316334686".to_string(),
                authored_by: AuthoredBy {
                    pen_name: "James S.A. Corey".to_string(),
                    authors: vec![
                        AuthorName {
                            first_name: "Daniel".to_string(),
                            last_name: "Abraham".to_string(),
                        },
                        AuthorName {
                            first_name: "Ty".to_string(),
                            last_name: "Franck".to_string(),
                        },
                    ],
                },
                title: "Cibola Burn".to_string(),
                genre: "Science Fiction".to_string(),
                ty: "Novel".to_string(),
                series_number: 4,
                description: "The fourth book in the Expanse series".to_string(),
            },
            Book {
                isbn: "978-0316334716".to_string(),
                authored_by: AuthoredBy {
                    pen_name: "James S.A. Corey".to_string(),
                    authors: vec![
                        AuthorName {
                            first_name: "Daniel".to_string(),
                            last_name: "Abraham".to_string(),
                        },
                        AuthorName {
                            first_name: "Ty".to_string(),
                            last_name: "Franck".to_string(),
                        },
                    ],
                },
                title: "Nemesis Games".to_string(),
                genre: "Science Fiction".to_string(),
                ty: "Novel".to_string(),
                series_number: 5,
                description: "The fifth book in the Expanse series".to_string(),
            },
            Book {
                isbn: "978-0316334747".to_string(),
                authored_by: AuthoredBy {
                    pen_name: "James S.A. Corey".to_string(),
                    authors: vec![
                        AuthorName {
                            first_name: "Daniel".to_string(),
                            last_name: "Abraham".to_string(),
                        },
                        AuthorName {
                            first_name: "Ty".to_string(),
                            last_name: "Franck".to_string(),
                        },
                    ],
                },
                title: "Babylon's Ashes".to_string(),
                genre: "Science Fiction".to_string(),
                ty: "Novel".to_string(),
                series_number: 6,
                description: "The sixth book in the Expanse series".to_string(),
            },
            Book {
                isbn: "978-0316332828".to_string(),
                authored_by: AuthoredBy {
                    pen_name: "James S.A. Corey".to_string(),
                    authors: vec![
                        AuthorName {
                            first_name: "Daniel".to_string(),
                            last_name: "Abraham".to_string(),
                        },
                        AuthorName {
                            first_name: "Ty".to_string(),
                            last_name: "Franck".to_string(),
                        },
                    ],
                },
                title: "Persepolis Rising".to_string(),
                genre: "Science Fiction".to_string(),
                ty: "Novel".to_string(),
                series_number: 7,
                description: "The seventh book in the Expanse series".to_string(),
            },
            Book {
                isbn: "978-0316332873".to_string(),
                authored_by: AuthoredBy {
                    pen_name: "James S.A. Corey".to_string(),
                    authors: vec![
                        AuthorName {
                            first_name: "Daniel".to_string(),
                            last_name: "Abraham".to_string(),
                        },
                        AuthorName {
                            first_name: "Ty".to_string(),
                            last_name: "Franck".to_string(),
                        },
                    ],
                },
                title: "Tiamat's Wrath".to_string(),
                genre: "Science Fiction".to_string(),
                ty: "Novel".to_string(),
                series_number: 8,
                description: "The eighth book in the Expanse series".to_string(),
            },
            Book {
                isbn: "978-0316332910".to_string(),
                authored_by: AuthoredBy {
                    pen_name: "James S.A. Corey".to_string(),
                    authors: vec![
                        AuthorName {
                            first_name: "Daniel".to_string(),
                            last_name: "Abraham".to_string(),
                        },
                        AuthorName {
                            first_name: "Ty".to_string(),
                            last_name: "Franck".to_string(),
                        },
                    ],
                },
                title: "Leviathan Falls".to_string(),
                genre: "Science Fiction".to_string(),
                ty: "Novel".to_string(),
                series_number: 9,
                description: "The ninth book in the Expanse series".to_string(),
            },
        ],
    };

    assert_eq!(books, expected_books);
    Ok(())
}
