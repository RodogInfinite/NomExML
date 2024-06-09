use std::fs::File;

use nom_xml::{
    attribute::{Attribute, AttributeValue},
    io::read_file,
    tag::Tag,
    update_fields, Document, Result, UpdateField,
};

#[derive(Debug, Default)]
struct Book {
    isbn: String,
    author: Author,
    title: String,
    genre: String,
    series_number: u8,
    description: String,
}

#[derive(Debug, Default, Clone)]
struct Author {
    pen_name: String,
    authors: Vec<AuthorName>,
}
#[derive(Debug, Default, Clone)]
struct AuthorName {
    first_name: String,
    last_name: String,
}
impl UpdateField for AuthorName {
    fn update_field(&mut self, tag: &Tag, doc: &Document) {
        let field_name = &tag.name.local_part;
        if let Document::Content(Some(value)) = &doc {
            match field_name.as_str() {
                "first_name" => {
                    self.first_name = value.to_string();
                }
                "last_name" => {
                    self.last_name = value.to_string();
                }
                e => {
                    eprintln!("Unknown field in AuthorName: {}", e);
                }
            }
        } else {
            eprintln!("Content is missing in AuthorName");
        }
    }
}

impl UpdateField for Author {
    fn update_field(&mut self, tag: &Tag, doc: &Document) {
        let field_name = &tag.name.local_part;
        let mut author_name = AuthorName::default();
        if let Document::Nested(_) = &doc {
            match field_name.as_str() {
                "authors" => {
                    doc.iter_with_depth(1).for_each(|record| {
                        if let Document::Element(_, inner_doc, _) = &record {
                            if let Document::Nested(ref elements) = **inner_doc {
                                for element in elements {
                                    if let Document::Element(tag, content, _) = element {
                                        author_name.update_field(tag, content);
                                    }
                                }
                                self.authors.push(author_name.clone());
                                author_name = AuthorName::default();
                            } else {
                                eprintln!("Content is missing");
                            }
                        }
                    });
                }
                e => {
                    eprintln!("Unknown field in Author: {}", e);
                }
            }
        } else if let Document::Content(Some(value)) = &doc {
            match field_name.as_str() {
                "pen_name" => {
                    self.pen_name = value.to_string();
                }
                e => {
                    eprintln!("Unknown field in Author11: {}", e);
                }
            }
        } else {
            eprintln!("Content is missing in Author");
        }
    }
}

impl UpdateField for Book {
    fn update_field(&mut self, tag: &Tag, doc: &Document) {
        let field_name = &tag.name.local_part;
        if let Some(attributes_vec) = &tag.attributes {
            if let Attribute::Instance {
                name,
                value: AttributeValue::Value(attr_val),
            } = attributes_vec.first().unwrap()
            {
                if name.local_part == "isbn" {
                    self.isbn = attr_val.to_string();
                }
            }
        }
        match &doc {
            Document::Nested(_) => {
                doc.iter_with_depth(1).for_each(|record| {
                    if let Document::Element(ref tag, ref inner_doc, _) = record {
                        if "authored_by" == tag.name.local_part {
                            update_fields(inner_doc, &mut self.author);
                        } else {
                            self.update_field(tag, inner_doc);
                        }
                    } else {
                        eprintln!("Unknown field: {record:#?}");
                    }
                });
            }

            Document::Content(Some(value)) => match field_name.as_str() {
                "title" => {
                    self.title = value.to_string();
                }
                "genre" => {
                    self.genre = value.to_string();
                }
                "series_number" => {
                    self.series_number = value.parse().unwrap_or_default();
                }
                "description" => {
                    self.description = value.to_string();
                }
                e => {
                    eprintln!("Unknown field2: {}", e);
                }
            },

            _ => {
                eprintln!("Content is missing");
            }
        }
    }
}

fn main() -> Result<()> {
    let mut file = File::open("examples/TheExpanseSeries.xml")?;
    let data = read_file(&mut file)?;
    let (_, doc) = Document::parse_element_by_tag_name(&data, "book", &None)?;
    let mut book = Book::default();
    doc.iter_with_depth(0)
        .filter_map(|record| {
            if let Document::Element(tag, inner_doc, _) = record {
                Some((tag, inner_doc))
            } else {
                None
            }
        })
        .for_each(|(tag, inner_doc)| book.update_field(tag, inner_doc));
    println!("{book:#?}");
    Ok(())
}
