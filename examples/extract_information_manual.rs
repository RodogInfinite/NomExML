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
    authored_by: AuthoredBy,
    title: String,
    genre: String,
    series_number: u8,
    description: String,
}
impl UpdateField for Book {
    fn update_field(&mut self, tag: &Tag, doc: &Document) -> Result<()> {
        let field_name = &tag.name.local_part;

        if let Some(attributes_vec) = &tag.attributes {
            for attr in attributes_vec.iter() {
                if let Attribute::Instance {
                    name,
                    value: AttributeValue::Value(attr_val),
                } = attr
                {
                    if name.local_part == "isbn" {
                        self.isbn = attr_val.to_string();
                    }
                }
            }
        }

        match &doc {
            Document::Nested(_) => {
                for element in doc.iter_with_depth(1) {
                    if let Document::Element(ref tag, ref inner_doc, _) = element {
                        if "authored_by" == tag.name.local_part {
                            update_fields(inner_doc, &mut self.authored_by)?;
                        } else {
                            self.update_field(tag, inner_doc)?;
                        }
                    } else {
                        return Err(format!("Unknown field: {element:#?}").into());
                    }
                }
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
                    return Err(format!("Unknown field2: {}", e).into());
                }
            },

            _ => {
                return Err("Content is missing".into());
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
struct AuthoredBy {
    pen_name: String,
    authors: Vec<AuthorName>,
}

impl UpdateField for AuthoredBy {
    fn update_field(&mut self, tag: &Tag, doc: &Document) -> Result<()> {
        match (tag.name.local_part.as_str(), doc) {
            ("pen_name", Document::Content(Some(value))) => {
                self.pen_name = value.to_string();
                Ok(())
            }
            ("authors", Document::Nested(elements)) => {
                elements.iter().try_for_each(
                    |element| -> std::result::Result<(), Box<dyn std::error::Error>> {
                        if let Document::Element(_, inner_doc, _) = element {
                            let mut author_name = AuthorName::default();
                            if let Document::Nested(inner_elements) = inner_doc.as_ref() {
                                inner_elements.iter().try_for_each(
                                    |inner_element| -> Result<()> {
                                        if let Document::Element(tag, content, _) = inner_element {
                                            author_name.update_field(tag, content)?;
                                        }
                                        Ok(())
                                    },
                                )?;
                                self.authors.push(author_name);
                            } else {
                                return Err("Content is missing in Author authors".into());
                            }
                        }
                        Ok(())
                    },
                )?;
                Ok(())
            }
            _ => Err(format!("Unknown field in Author: {}", tag.name.local_part).into()),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct AuthorName {
    first_name: String,
    last_name: String,
}
impl UpdateField for AuthorName {
    fn update_field(&mut self, tag: &Tag, doc: &Document) -> Result<()> {
        let field_name = &tag.name.local_part;

        if let Document::Content(Some(value)) = &doc {
            match field_name.as_str() {
                "first_name" => {
                    self.first_name = value.to_string();
                    Ok(())
                }
                "last_name" => {
                    self.last_name = value.to_string();
                    Ok(())
                }
                e => Err(format!("Unknown field in AuthorName: {}", e).into()),
            }
        } else {
            Err("Content is missing in AuthorName".into())
        }
    }
}
fn main() -> Result<()> {
    let mut file = File::open("examples/TheExpanseSeries.xml")?;
    let data = read_file(&mut file)?;
    let (_, doc) = Document::parse_element_by_tag_name(&data, "book", &None)?;
    let mut book = Book::default();

    doc.iter_with_depth(0)
        .filter_map(|element| {
            if let Document::Element(tag, inner_doc, _) = element {
                Some((tag, inner_doc))
            } else {
                None
            }
        })
        .try_for_each(|(tag, inner_doc)| book.update_field(tag, inner_doc))
        .map_err(|e| {
            println!("Error updating field: {}", e);
            e
        })?;

    println!("{book:#?}");
    Ok(())
}
