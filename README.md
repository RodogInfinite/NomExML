# NomExML

[![Crates.io](https://img.shields.io/crates/v/nom-xml)](https://crates.io/crates/nom-xml)  [![docs.rs](https://img.shields.io/docsrs/nom-xml)](https://docs.rs/nom-xml/latest/nom_xml/) ![Crates.io](https://img.shields.io/crates/l/nom-xml) ![Crates.io (latest)](https://img.shields.io/crates/dv/nom_xml)

---

`nom-xml` is a crate for parsing XML documents using the [`nom`](https://github.com/rust-bakery/nom) parser combinator crate.

This crate was initially created to be able to parse the following XML pattern which was troublesome in other Rust XML parsers explored at the time due to the limitations of the Serde crate:

```xml
<root>
    <header>
        <header_field1>Value1</header_field1>
        <header_field2>Value2</header_field2>
    </header>
    <body>
        <body_field1>BodyValue1</body_field1>
        <body_field2>BodyValue2</body_field2>
    </body>
    <header>
        <header_field1>Value1</header_field1>
        <header_field2>Value2</header_field2>
    </header>
    <body>
        <body_field1>BodyValue1</body_field1>
        <body_field2>BodyValue2</body_field2>
    </body>
</root>
```

It eventually evolved into implementing the [XML 1.0 Specification - Fifth Edition](https://www.w3.org/TR/2008/REC-xml-20081126/) as closely as possible.
Nom was chosen specifically for its combinator parsing style which allowed for the implementation of the XML specification rules from their lowest level up to parsing the full document step-by-step.
There is still a decent way to go to get to full compliance but the ultimate goal is to be able to parse any XML document, validate on schema, and write compliant XML documents.
Unless complicated external entities are involved, this crate should already be able to parse most XML documents.

# Key Data Structure:

# `Document`

This enum encapsulates all of the top level types that comprises an XML document. The core variant is the `Element(Tag,Box<Document>,Tag)` type which allows recursive parsing of nested tags and their content.

```rust
pub enum Document {
    Prolog {
        xml_decl: Option<XmlDecl>,
        misc: Option<Vec<Misc>>,
        doc_type: Option<DocType>,
    },
    Element(Tag, Box<Document>, Tag),
    Content(Option<String>),
    Nested(Vec<Document>),
    Empty,
    EmptyTag(Tag),
    ProcessingInstruction(ProcessingInstruction),
    Comment(String),
    CDATA(String),
}

```



# Key Methods:

## Document::parse
The main way to parse an ***entire*** XML `&str`.

### Example:
```rust
use nom_xml::{parse::Parse, config::Config, Document};

fn main() {
    let xml = "<root><child>Content</child></root>";
    let (_, doc) = Document::parse(xml, &Config::default()).unwrap();
    println!("{doc:?}");
}
```

### Output:
```rust
Element(
    Tag {
        name:
            Name {
                prefix: None,
                local_part: "root",
            },
        attributes: None,
        state: Start,
    },
    Nested([
        Element(
            Tag {
                name:
                    Name {
                        prefix: None,
                        local_part: "child",
                    },
                attributes: None,
                state: Start,
            },
            Content("Content"),
            Tag {
                name:
                    Name {
                        prefix: None,
                        local_part: "child",
                    },
                attributes: None,
                state: End,
            },
        ),
    ]),
    Tag {
        name:
            Name {
                prefix: None,
                local_part: "root",
            },
        attributes: None,
        state: End,
    },
)
```

---

## Document::iter_with_depth

A method for iterating to a specific depth of an XML tree. See the ['extract_information`](https://github.com/RodogInfinite/NomExML/blob/main/examples/extract_information.rs) example for more details


# Introducing `nom-xml-derive`
As of `nom-xml` version 0.3.0, `nom-xml-derive` is available for use. The `nom-xml-derive` derive macro crate was created to reduce the boilerplate necessary for users to extract data into structs. See the differences between manual implementations and derived counterpart implementations in the [examples](https://github.com/RodogInfinite/NomExML/blob/main/examples).


---
## TODO
---
| Task | Priority Level | Status |
|:----------------------|:----------:|:-----:|
| Implement full test suite | High | :construction: |
| Implement all [production rules](src/docs/parser_implementation_tracking.md)  | High | :construction: |
| Add ability to parse content and children from specific tags | High | ✅ |
| Add better error handling | High | :thought_balloon: |
| Fix debug output | Medium | :thought_balloon: |
| Add streaming for parsing large XML documents | Low | ❌ |
| Implement Display | Low | :thought_balloon: |
| Investigate [console](https://crates.io/crates/console) for use in Debug and Display output formatting (page interface?) | Low | :thought_balloon: |

