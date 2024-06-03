//!
#![doc = include_str!("docs/crate_description.md")]
//!
pub mod attribute;
pub mod config;
mod debug;
pub mod error;
pub mod io;
pub mod misc;
pub mod namespaces;
pub mod parse;
pub mod processing_instruction;
pub mod prolog;
pub mod reference;
pub mod tag;
pub mod transcode;

use crate::{
    config::{check_config, Config, ExternalEntityParseConfig},
    misc::{Misc, MiscState},
    parse::Parse,
    processing_instruction::ProcessingInstruction,
    prolog::{
        doctype::DocType,
        subset::{
            entity::{
                entity_declaration::EntityDecl, entity_definition::EntityDefinition,
                entity_value::EntityValue, EntitySource,
            },
            markup_declaration::MarkupDeclaration,
            Subset,
        },
        xmldecl::XmlDecl,
    },
    reference::Reference,
    tag::Tag,
};

use attribute::Attribute;

use error::{ConvertNomError, Error};
use io::parse_external_entity_file;
use namespaces::ParseNamespace;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    combinator::{cut, map, map_res, not, opt, value},
    multi::{many0, many1, many_till},
    sequence::{pair, preceded, tuple},
};

use prolog::{external_id::ExternalID, subset::entity::entity_declaration::EntityDeclaration};

use std::{cell::RefCell, collections::HashMap, fmt, fs::File, io::Write, rc::Rc};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type IResult<I, O> = nom::IResult<I, O, Error>;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Name {
    pub prefix: Option<String>,
    pub local_part: String,
}

impl Name {
    /// A more convenient way to create a new Name.
    ///
    /// ```rust
    /// use nom_xml::Name;
    /// // Create a new Name without a prefix
    /// let name = Name::new(None, "actual name");
    ///
    /// // Create a new Name with a prefix
    /// let prefixed_name = Name::new(Some("prefix"), "actual name");
    /// ```
    ///
    pub fn new(prefix: Option<&str>, local_part: &str) -> Self {
        Self {
            prefix: prefix.map(|p| p.to_string()),
            local_part: local_part.to_string(),
        }
    }
}

/// Main entry point for parsing XML documents
///
/// This enum encapsulates all of the top level types that comprise an XML document. The core variant is the `Element(Tag,Box<Document>,Tag)` type which allows recursive parsing of nested tags and their content.
#[derive(Clone, PartialEq, Eq)]
pub enum Document {
    Prolog {
        xml_decl: Option<XmlDecl>,
        misc: Option<Vec<Misc>>,
        doc_type: Option<DocType>,
    },
    Element(Tag, Box<Document>, Tag),
    Content(Option<String>), //TODO: Investigate if content can ever be None. I think Empty handles this case. If so, remove the Option
    Nested(Vec<Document>),
    Empty,
    EmptyTag(Tag),
    ProcessingInstruction(ProcessingInstruction),
    Comment(String),
    CDATA(String),
}
impl<'a> Parse<'a> for Document {
    type Args = Config;
    type Output = IResult<&'a str, Self>;

    /// ```rust
    /// use nom_xml::{parse::Parse, config::Config, Document};
    ///
    /// let xml = "<root><child>Content</child></root>";
    /// let (_, doc) = Document::parse(xml, Config::default()).unwrap();
    /// println!("{doc:?}");
    /// ```
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        match check_config(&args) {
            Ok(_) => {
                let entity_references = Rc::new(RefCell::new(HashMap::new()));
                let (input, prolog_and_references) =
                    opt(|i| Self::parse_prolog(i, entity_references.clone(), args.clone()))(input)?;

                let (prolog, new_entity_references) = match prolog_and_references {
                    Some((prolog, entity_references)) => (prolog, entity_references),
                    None => (None, entity_references.clone()),
                };

                let mut documents = Vec::new();

                let mut current_input = input;
                while !current_input.is_empty() {
                    let (input, mut start_tag) = opt(|i| {
                        Tag::parse_start_tag(
                            i,
                            new_entity_references.clone(),
                            EntitySource::Internal,
                        )
                    })(current_input)?;

                    let source = Self::determine_source_from_references(&new_entity_references); //THIS IS THE ISSUE

                    let (input, content) = Self::parse_content(
                        input,
                        &new_entity_references,
                        source, //TODO Investigate how to handle both internal and external
                    )?;

                    let (input, end_tag) = opt(Tag::parse_end_tag)(input)?;

                    let mut empty_tag = if let Document::EmptyTag(empty_tag) = &content {
                        Some(empty_tag.clone())
                    } else {
                        None
                    };

                    if let Some(Document::Prolog {
                        doc_type:
                            Some(DocType {
                                subset: Some(ref subset),
                                ..
                            }),
                        ..
                    }) = prolog
                    {
                        for subset in subset {
                            if let Subset::MarkupDecl(MarkupDeclaration::AttList {
                                name,
                                att_defs: Some(att_defs),
                            }) = subset
                            {
                                if let Some(start_tag) = &mut start_tag {
                                    if start_tag.name == *name {
                                        start_tag.merge_default_attributes(&att_defs.clone());
                                    }
                                }
                                if let Some(empty_tag) = &mut empty_tag {
                                    if empty_tag.name == *name {
                                        empty_tag.merge_default_attributes(&att_defs.clone());
                                    }
                                }
                            }
                        }
                    }

                    let (input, doc) = Self::construct_document_element(
                        input, start_tag, content, end_tag, empty_tag,
                    )?;
                    if let Document::Empty = &doc {
                        break;
                    }

                    documents.push(doc);
                    current_input = input;
                }

                let (input, documents) = Self::construct_document(input, prolog, documents)?;
                Ok((input, documents))
            }
            Err(e) => Err(Error::from(e).into()),
        }
    }
}

impl Document {
    fn determine_source_from_references(
        refs: &Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
    ) -> EntitySource {
        let refs_borrow = refs.borrow(); // Immutable borrow for reading
                                         // Define a logic or condition based on the actual data
                                         // Example logic based on a placeholder condition
        if refs_borrow
            .keys()
            .any(|(_name, source)| *source == EntitySource::External)
        {
            EntitySource::External
        } else if refs_borrow
            .keys()
            .any(|(_, source)| *source == EntitySource::Internal)
        {
            EntitySource::Internal
        } else {
            EntitySource::None
        }
    }

    //[22 prolog ::= XMLDecl? Misc* (doctypedecl Misc*)?
    pub fn parse_prolog(
        input: &str,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        config: Config,
    ) -> IResult<
        &str,
        (
            Option<Document>,
            Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        ),
    > {
        let (input, xml_decl) = opt(|i| XmlDecl::parse(i, ()))(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, misc_before) =
            opt(|input| Misc::parse(input, MiscState::BeforeDoctype))(input)?;
        let (input, doc_type) =
            opt(|i| DocType::parse(i, (entity_references.clone(), config.clone())))(input)?;
        let (input, misc_after) = match &doc_type {
            Some(_) => opt(|input| Misc::parse(input, MiscState::AfterDoctype))(input)?,
            None => (input, None),
        };
        let updated_entity_references = match &doc_type {
            Some(dt) => Self::collect_entity_references(dt, entity_references.clone()),
            None => entity_references.clone(),
        };
        let miscs: Vec<Option<Misc>> = vec![misc_before, misc_after];
        let miscs: Vec<Misc> = miscs.into_iter().flatten().collect();
        let misc = if miscs.is_empty() { None } else { Some(miscs) };

        let prolog = match (&xml_decl, &misc, &doc_type) {
            (None, None, None) => None,
            _ => Some(Document::Prolog {
                xml_decl,
                misc,
                doc_type,
            }),
        };

        Ok((input, (prolog, updated_entity_references)))
    }

    // [14] CharData ::= [^<&]* - ([^<&]* ']]>' [^<&]*)
    fn parse_char_data(input: &str) -> IResult<&str, String> {
        map(
            tuple((
                take_till(|c: char| c == '<' || c == '&'),
                not(tag::<&str, &str, nom::error::Error<&str>>("]]>")),
            )),
            |(data, _)| data.to_string(),
        )(input)
        .map_err(|e| e.convert_nom_error())
    }

    // [20] CData ::= (Char* - (Char* ']]>' Char*))
    fn parse_cdata(input: &str) -> IResult<&str, String> {
        map(
            cut(|i| {
                let original_input = i;
                let (input, _) = many_till(Self::parse_char, tag("]]>"))(i)?;
                let parsed_length = original_input.len() - input.len() - 3; // subtract 3 for ']]>'
                let cdata_slice = &original_input[..parsed_length];
                Ok((input, cdata_slice.to_string()))
            }),
            |s| s,
        )(input)
    }

    // [18] CDSect ::= CDStart CData CDEnd
    // [19] CDStart ::= '<![CDATA['
    //[21] CDEnd ::= ']]>'
    fn parse_cdata_section(input: &str) -> IResult<&str, Document> {
        map(
            preceded(tag("<![CDATA["), Self::parse_cdata),
            Document::CDATA,
        )(input)
    }

    // [39] element	::= EmptyElemTag | STag content ETag
    fn parse_element(
        input: &str,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
    ) -> IResult<&str, Document> {
        let (input, doc) = alt((
            preceded(
                Self::parse_multispace0, // this is not adhering strictly to the spec, but handles the case where there is whitespace before the start tag for human readability
                map(
                    |i| {
                        Tag::parse_empty_element_tag(
                            i,
                            entity_references.clone(),
                            EntitySource::None,
                        )
                    },
                    Document::EmptyTag,
                ),
            ),
            map(
                tuple((
                    Self::parse_multispace0, // this is not adhering strictly to the spec, but handles the case where there is whitespace before the start tag for human readability
                    |i| Tag::parse_start_tag(i, entity_references.clone(), EntitySource::Internal),
                    |i| Self::parse_content(i, &entity_references, EntitySource::Internal),
                    Tag::parse_end_tag,
                    Self::parse_multispace0, // this is not adhering strictly to the spec, but handles the case where there is whitespace after the start tag for human readability
                )),
                |(_whitespace1, start_tag, content, end_tag, _whitespace2)| {
                    Document::Element(start_tag, Box::new(content), end_tag)
                },
            ),
        ))(input)?;

        Ok((input, doc))
    }

    fn collect_entity_references(
        doc_type: &DocType,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
    ) -> Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>> {
        if let Some(entities) = doc_type.extract_entities() {
            for boxed_entity in &entities {
                if let Subset::MarkupDecl(MarkupDeclaration::Entity(entity_decl)) = &**boxed_entity
                {
                    match entity_decl {
                        EntityDecl::General(decl) | EntityDecl::Parameter(decl) => {
                            match &decl.entity_def {
                                EntityDefinition::EntityValue(value) => {
                                    let mut references = entity_references.borrow_mut();
                                    references
                                        .entry((decl.name.clone(), EntitySource::Internal))
                                        .or_insert(value.clone());
                                }
                                EntityDefinition::External { .. } => {
                                    let mut references = entity_references.borrow_mut();

                                    references.entry((decl.name.clone(), EntitySource::External));
                                }
                            }
                        }
                    }
                }
            }
        }

        if entity_references.borrow().is_empty() {
            Rc::new(RefCell::new(HashMap::new()))
        } else {
            entity_references
        }
    }

    fn process_references(
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
    ) -> impl Fn(Vec<Reference>) -> Document {
        move |references| {
            let mut contents: Vec<String> = Vec::new();
            for reference in references.into_iter() {
                match reference.normalize_entity(entity_references.clone()) {
                    EntityValue::Document(doc) => return doc,
                    EntityValue::Value(val) => contents.push(val),
                    _ => {}
                }
            }
            let content = contents.concat();
            Document::Content(Some(content))
        }
    }

    // TODO: add validation for elements using the ConditionalState in the ContentParticle from the prolog
    // [43] content ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
    fn parse_content<'a>(
        input: &'a str,
        entity_references: &Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        entity_source: EntitySource,
    ) -> IResult<&'a str, Document> {
        let (input, ((_whitespace, maybe_chardata), elements)) = tuple((
            pair(
                Self::parse_multispace0, // this is not strictly adhering to the standard; however, it prevents the first Nested element from being Nested([Content(" ")])
                opt(Self::parse_char_data),
            ),
            many0(alt((
                pair(
                    map(
                        many1(|i| Reference::parse(i, entity_source.clone())),
                        Self::process_references(entity_references.clone()),
                    ),
                    pair(
                        Self::parse_multispace0, // this is not strictly adhering to the standard; however, it prevents the first Nested element from being Nested([Content(" ")])
                        opt(Self::parse_char_data),
                    ),
                ),
                pair(
                    |i| Self::parse_element(i, entity_references.clone()),
                    pair(
                        Self::parse_multispace0, // this is not strictly adhering to the standard; however, it prevents the first Nested element from being Nested([Content(" ")])
                        opt(Self::parse_char_data),
                    ),
                ),
                pair(
                    Self::parse_cdata_section,
                    pair(
                        Self::parse_multispace0, // this is not strictly adhering to the standard; however, it prevents the first Nested element from being Nested([Content(" ")])
                        opt(Self::parse_char_data),
                    ),
                ),
                pair(
                    map(
                        |i| ProcessingInstruction::parse(i, ()),
                        Document::ProcessingInstruction,
                    ),
                    pair(
                        Self::parse_multispace0, // this is not strictly adhering to the standard; however, it prevents the first Nested element from being Nested([Content(" ")])
                        opt(Self::parse_char_data),
                    ),
                ),
                pair(
                    Self::parse_comment,
                    pair(
                        Self::parse_multispace0, // this is not strictly adhering to the standard; however, it prevents the first Nested element from being Nested([Content(" ")])
                        opt(Self::parse_char_data),
                    ),
                ),
            ))),
        ))(input)?;

        // Check if maybe_chardata contains a comma
        let mut content = elements
            .into_iter()
            .flat_map(|(doc, maybe_chardata)| {
                let mut vec = Vec::new();

                vec.push(doc);

                if let (_, Some(chardata)) = maybe_chardata {
                    if !chardata.is_empty() {
                        vec.push(Document::Content(Some(chardata)));
                    }
                }
                vec
            })
            .collect::<Vec<_>>();

        Ok((
            input,
            match maybe_chardata {
                Some(chardata) if !chardata.is_empty() => {
                    let mut vec = Vec::new();

                    vec.push(Document::Content(Some(chardata)));

                    vec.append(&mut content);

                    match vec.as_slice() {
                        [doc] => doc.clone(),

                        _ => Document::Nested(vec),
                    }
                }
                _ => {
                    if content.is_empty() {
                        Document::Empty
                    } else {
                        match &content[..] {
                            [doc @ Document::Content(_)] => doc.clone(),
                            [doc @ Document::ProcessingInstruction(_)] => doc.clone(),
                            [doc @ Document::CDATA(_)] => doc.clone(),
                            [doc @ Document::Comment(_)] => doc.clone(),
                            [doc @ Document::EmptyTag(_)] => doc.clone(),
                            [doc @ Document::Empty] => doc.clone(),
                            [doc @ Document::Nested(_)] => doc.clone(),
                            _ => Document::Nested(content),
                        }
                    }
                }
            },
        ))
    }

    // [15] Comment ::= '<!--' ((Char - '-') | ('-' (Char - '-')))* '-->'
    fn parse_comment(input: &str) -> IResult<&str, Document> {
        map_res(
            pair(tag("<!--"), many_till(Self::parse_char, tag("-->"))),
            |(_open_comment, (comment_content, _close_comment))| {
                let comment_string: String = comment_content.into_iter().collect();
                if comment_string.contains("--") {
                    Err(nom::Err::Failure(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Verify,
                    )))
                } else {
                    Ok(Document::Comment(comment_string))
                }
            },
        )(input)
    }

    fn construct_document_element(
        input: &str,
        start_tag: Option<Tag>,
        content: Document,
        end_tag: Option<Tag>,
        empty_tag: Option<Tag>,
    ) -> IResult<&str, Document> {
        match (start_tag, end_tag, content, empty_tag) {
            (Some(start), Some(end), content, None) => {
                if start.name != end.name {
                    return Err(nom::Err::Error(Error::NomError(nom::error::Error::new(
                        input.to_string(),
                        nom::error::ErrorKind::Verify,
                    ))));
                }

                let document = Document::Element(start, Box::new(content), end);

                Ok((input, document))
            }
            (Some(start), Some(end), _, Some(empty_tag)) => {
                if start.name != end.name {
                    return Err(nom::Err::Error(Error::NomError(nom::error::Error::new(
                        input.to_string(),
                        nom::error::ErrorKind::Verify,
                    ))));
                }

                let document =
                    Document::Element(start, Box::new(Document::EmptyTag(empty_tag)), end);

                Ok((input, document))
            }
            (Some(_), None, Document::Element(start, inner_content, end), None) => {
                if start.name != end.name {
                    return Err(nom::Err::Error(Error::NomError(nom::error::Error::new(
                        input.to_string(),
                        nom::error::ErrorKind::Verify,
                    ))));
                }

                let document = Document::Element(start, inner_content, end);

                Ok((input, document))
            }
            (None, None, Document::Element(start, inner_content, end), None) => {
                if start.name != end.name {
                    return Err(nom::Err::Error(Error::NomError(nom::error::Error::new(
                        input.to_string(),
                        nom::error::ErrorKind::Verify,
                    ))));
                }

                let document = Document::Element(start, inner_content, end);

                Ok((input, document))
            }
            (None, None, _, Some(empty)) => {
                let document = Document::EmptyTag(empty);

                Ok((input, document))
            }
            (None, None, Document::Empty, None) => Ok((input, Document::Empty)),
            (None, None, Document::ProcessingInstruction(processing_instruction), None) => {
                let document = Document::ProcessingInstruction(processing_instruction);

                Ok((input, document))
            }
            (None, None, Document::Comment(comment), None) => {
                let document = Document::Comment(comment);

                Ok((input, document))
            }
            _ => Err(nom::Err::Error(Error::NomError(nom::error::Error::new(
                input.to_string(),
                nom::error::ErrorKind::Verify,
            )))),
        }
    }

    fn construct_document(
        input: &str,
        prolog: Option<Document>,
        documents: Vec<Document>,
    ) -> IResult<&str, Document> {
        match documents.len() {
            0 => Err(nom::Err::Error(Error::NomError(nom::error::Error::new(
                input.to_string(),
                nom::error::ErrorKind::Verify,
            )))),
            1 => match prolog {
                Some(prolog) => Ok((
                    input,
                    Document::Nested(vec![prolog, documents.into_iter().next().unwrap()]),
                )),
                None => Ok((input, documents.into_iter().next().unwrap())),
            },
            _ => match prolog {
                Some(prolog) => {
                    let mut vec = vec![prolog];
                    vec.extend(documents);
                    Ok((input, Document::Nested(vec)))
                }
                None => Ok((input, Document::Nested(documents))),
            },
        }
    }

    fn process_external_entity_file(
        file_path: String,
        name: &Name,
        config: Config,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
    ) -> Result<()> {
        match File::open(file_path) {
            Ok(mut file) => {
                match parse_external_entity_file(&mut file, &config, entity_references.clone())
                    .as_deref()
                {
                    Ok([entity]) => {
                        entity_references
                            .borrow_mut()
                            .insert((name.clone(), EntitySource::External), entity.clone());
                        Ok(())
                    }
                    _ => Err(nom::Err::Error(Error::NomError(nom::error::Error::new(
                        "Failed to match [entity] from `parse_external_entity_file`".to_string(),
                        nom::error::ErrorKind::Fail,
                    )))
                    .into()),
                }
            }
            Err(e) => Err(Error::from(e).into()),
        }
    }

    fn get_external_entity_from_declaration(
        entity_declaration: EntityDecl,
        entity_references: Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
        config: Config,
    ) -> Result<()> {
        if let Config {
            external_parse_config:
                ExternalEntityParseConfig {
                    allow_ext_parse: true,
                    base_directory,
                    ..
                },
        } = &config
        {
            if let EntityDecl::Parameter(EntityDeclaration {
                name,
                entity_def:
                    EntityDefinition::External {
                        id: ExternalID::System(ent_file),
                        ..
                    },
            })
            | EntityDecl::General(EntityDeclaration {
                name,
                entity_def:
                    EntityDefinition::External {
                        id: ExternalID::System(ent_file),
                        ..
                    },
            }) = &entity_declaration
            {
                let file_path = match base_directory {
                    Some(base) => format!("{}/{}", base, ent_file),
                    None => ent_file.clone(),
                };
                Self::process_external_entity_file(file_path, name, config, entity_references)
            } else if let EntityDecl::General(EntityDeclaration {
                name,
                entity_def:
                    EntityDefinition::External {
                        id:
                            ExternalID::Public {
                                system_identifier, ..
                            },
                        ..
                    },
            }) = entity_declaration
            {
                if let ExternalID::System(system_identifier) = *system_identifier {
                    let file_path = match base_directory {
                        Some(base) => format!("{}/{}", base, system_identifier),
                        None => system_identifier.clone(),
                    };
                    Document::process_external_entity_file(
                        file_path,
                        &name,
                        config,
                        entity_references,
                    )
                } else {
                    Err(nom::Err::Error(nom::error::Error::new(
                        "Failed to match *system_identifier",
                        nom::error::ErrorKind::Fail,
                    ))
                    .into())
                }
            } else {
                Err(nom::Err::Error(nom::error::Error::new(
                    "Failed to match ExternalID::Public",
                    nom::error::ErrorKind::Fail,
                ))
                .into())
            }
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                "Failed to match &entity_declaration",
                nom::error::ErrorKind::Fail,
            ))
            .into())
        }
    }

    /// The main interface for exracting content from the Document tree
    /// See the  [`extract_information`](../extract_information/index.html) example for more information
    pub fn iter_with_depth(&self, max_level: usize) -> DocumentIterator {
        DocumentIterator::new(self, Some(max_level))
    }

    /// The main interface for parsing the first element that matches criteria
    ///
    /// See the [`parse_first_matching_element`](../parse_first_matching_element/index.html) example for more information
    ///
    /// Run with `cargo run --example parse_first_matching_element`
    ///
    /// Also see the [`parse_element_with_specific_attribute_value`](../parse_element_with_specific_attribute_value/index.html) example
    ///
    /// Run with `cargo run --example parse_element_with_specific_attribute_value`
    ///
    // [39] element	::= EmptyElemTag | STag content ETag
    pub fn parse_element_by_tag_name<'a>(
        input: &'a str,
        tag_name: &'a str,
        attributes: &Option<Vec<Attribute>>,
    ) -> IResult<&'a str, Document> {
        let (input, _) = take_until(format!("<{}", tag_name).as_str())(input)?;
        let entity_references = &Rc::new(RefCell::new(HashMap::new()));
        let (input, doc) = alt((
            preceded(
                Self::parse_multispace0, // this is not adhering strictly to the spec, but handles the case where there is whitespace before the start tag for human readability
                map(
                    |i| {
                        Tag::parse_empty_element_tag_by_name(
                            i,
                            tag_name,
                            attributes,
                            entity_references,
                            EntitySource::None,
                        )
                    },
                    Document::EmptyTag,
                ),
            ),
            map(
                tuple((
                    Self::parse_multispace0, // this is not adhering strictly to the spec, but handles the case where there is whitespace before the start tag for human readability
                    |i| {
                        Tag::parse_start_tag_by_name(
                            i,
                            tag_name,
                            attributes,
                            entity_references,
                            EntitySource::Internal,
                        )
                    },
                    |i| Self::parse_content(i, entity_references, EntitySource::Internal),
                    |i| Tag::parse_end_tag_by_name(i, tag_name),
                    Self::parse_multispace0, // this is not adhering strictly to the spec, but handles the case where there is whitespace after the start tag for human readability
                )),
                |(_whitespace1, start_tag, content, end_tag, _whitespace2)| {
                    Document::Element(start_tag, Box::new(content), end_tag)
                },
            ),
        ))(input)?;
        Ok((input, doc))
    }

    /// The main interface for parsing many elements with the same tag name
    ///
    /// See the [`parse_all_of_specific_tag`](../parse_all_of_specific_tag/index.html) example for more information
    ///
    /// Run with `cargo run --example parse_all_of_specific_tag`
    ///
    // [43] content ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
    pub fn parse_elements_by_tag_name<'a>(
        input: &'a str,
        tag_name: &'a str,
        attributes: &Option<Vec<Attribute>>,
    ) -> IResult<&'a str, Vec<Document>> {
        warnln!("parse_elements_by_tag_name will parse all elements with the tag name `{tag_name}` no matter the nesting level", );
        warnln!("parse_element_by_tag_name currently only parses start tags without attributes, in this case`<{tag_name}>`");

        many1(|i| Self::parse_element_by_tag_name(i, tag_name, attributes))(input)
    }

    #[cfg(feature = "experimental")]
    pub fn parse_element_from_pattern<'a>(
        input: &'a str,
        tag_name: &'a str,
        pattern: &'a Pattern,
        strict: bool,
        entity_references: &Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
    ) -> IResult<&'a str, Document> {
        let (_, _pattern_doc) = Self::parse_element(pattern.xml, entity_references.clone())?;

        let pattern = pattern
            .parse(entity_references)
            .map_err(|e| nom::Err::Error(Error::from(e)))?;
        let (input, doc) =
            peek(|input| Self::parse_element_by_tag_name(input, tag_name, entity_references))(
                input,
            )?;
        match (&doc, &pattern.doc) {
            (
                Document::Element(_, inner_element, _),
                Document::Element(_, pattern_inner_element, _),
            ) => {
                if let (Document::Nested(inner_docs), Document::Nested(pattern_inner_docs)) =
                    (&**inner_element, &**pattern_inner_element)
                {
                    let mut doc_matches = vec![false; pattern_inner_docs.len()];

                    for (counter, pattern_doc) in pattern_inner_docs.iter().enumerate() {
                        for inner in inner_docs.iter() {
                            if strict {
                                if Self::compare_documents(
                                    inner,
                                    pattern.clone(),
                                    ComparisonMethod::Strict,
                                ) {}
                            } else if Self::compare_documents(
                                inner,
                                Pattern::new("", pattern_doc.clone()),
                                ComparisonMethod::Partial,
                            ) {
                                doc_matches[counter] = true;
                            }
                        }
                    }

                    if doc_matches.iter().all(|&vals| vals) {
                        let (input, doc) =
                            Self::parse_element_by_tag_name(input, tag_name, entity_references)?;
                        Ok((input, doc))
                    } else {
                        Err(nom::Err::Error(Error::NomError(nom::error::Error::new(
                            input.to_string(),
                            nom::error::ErrorKind::Verify,
                        ))))
                    }
                } else {
                    Err(nom::Err::Error(Error::NomError(nom::error::Error::new(
                        input.to_string(),
                        nom::error::ErrorKind::Verify,
                    ))))
                }
            }
            _ => Err(nom::Err::Error(Error::NomError(nom::error::Error::new(
                input.to_string(),
                nom::error::ErrorKind::Verify,
            )))),
        }
    }
    #[cfg(feature = "experimental")]
    fn compare_documents(doc1: &Document, pattern: Pattern, method: ComparisonMethod) -> bool {
        doc1.equals(pattern, method)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentIterator<'a> {
    stack: Vec<(&'a Document, usize)>,
    max_depth: Option<usize>,
}

impl<'a> DocumentIterator<'a> {
    fn new(doc: &'a Document, max_depth: Option<usize>) -> Self {
        let stack = vec![(doc, 0)];
        DocumentIterator { stack, max_depth }
    }
}
impl<'a> Iterator for DocumentIterator<'a> {
    type Item = &'a Document;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((doc, level)) = self.stack.pop() {
            if self.max_depth.map_or(true, |max| level < max) {
                match doc {
                    Document::Nested(docs) => {
                        for d in docs.iter().rev() {
                            self.stack.push((d, level + 1));
                        }

                        continue;
                    }
                    Document::Element(_, inner_doc, _) => {
                        // Add the inner document of an element
                        self.stack.push((inner_doc, level + 1));

                        continue;
                    }
                    _ => {}
                }
            }

            return Some(doc);
        }

        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConditionalState {
    None,
    Optional,
    ZeroOrMore,
    OneOrMore,
}
impl<'a> Parse<'a> for ConditionalState {
    type Args = ();
    type Output = IResult<&'a str, Self>;
    fn parse(input: &'a str, _args: Self::Args) -> Self::Output {
        alt((
            value(ConditionalState::Optional, tag("?")),
            value(ConditionalState::ZeroOrMore, tag("*")),
            value(ConditionalState::OneOrMore, tag("+")),
        ))(input)
    }
}

// TODO: migrate this to error.rs possibly combine with CustomError
#[derive(Debug)]
pub enum DocumentError {
    NoMatchingDocuments,
    ExpectedNestedDocument,
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DocumentError::NoMatchingDocuments => {
                write!(f, "No matching documents found during extraction")
            }
            DocumentError::ExpectedNestedDocument => {
                write!(f, "Expected a nested document, but found another variant")
            }
        }
    }
}

impl std::error::Error for DocumentError {}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pattern<'a> {
    pub xml: &'a str,
    pub doc: Document,
}
impl<'a> Pattern<'a> {
    pub fn new(xml: &'a str, doc: Document) -> Self {
        Self { xml, doc }
    }
    pub fn parse(
        &self,
        entity_references: &Rc<RefCell<HashMap<(Name, EntitySource), EntityValue>>>,
    ) -> Result<Pattern> {
        let (_, doc) = Document::parse_element(self.xml, entity_references.clone())?;

        Ok(Self { xml: self.xml, doc })
    }
}
pub(crate) trait PartialEqCustom {
    fn partial_eq(&self, pattern: Pattern) -> bool;
}

impl PartialEqCustom for Document {
    fn partial_eq(&self, pattern: Pattern) -> bool {
        match (self, &pattern.doc) {
            (
                Document::Prolog {
                    xml_decl: a_xml_decl,
                    misc: a_misc,
                    doc_type: a_doc_type,
                },
                Document::Prolog {
                    xml_decl: pattern_xml_decl,
                    misc: pattern_misc,
                    doc_type: pattern_doc_type,
                },
            ) => {
                a_xml_decl == pattern_xml_decl
                    && a_misc == pattern_misc
                    && a_doc_type == pattern_doc_type
            }

            (
                Document::Element(a_start_tag, a_docs, a_end_tag),
                Document::Element(pattern_start_tag, pattern_docs, pattern_end_tag),
            ) if a_start_tag == pattern_start_tag && a_end_tag == pattern_end_tag => {
                match (&**a_docs, &**pattern_docs) {
                    (Document::Nested(a_docs), Document::Nested(pattern_docs)) => a_docs
                        .iter()
                        .zip(pattern_docs.iter())
                        .all(|(pattern_doc, a_doc)| {
                            a_doc.partial_eq(Pattern::new("", pattern_doc.clone()))
                        }),
                    (Document::Content(_), Document::Content(_)) => true,
                    _ => panic!("Mismatched types"),
                }
            }

            (Document::Content(a_content), Document::Content(pattern_content)) => {
                a_content == pattern_content
            }

            (Document::Nested(a_docs), Document::Nested(pattern_docs)) => a_docs == pattern_docs,

            (Document::Empty, Document::Empty) => true,
            (Document::EmptyTag(a_tag), Document::EmptyTag(pattern_tag)) => a_tag == pattern_tag,
            (
                Document::ProcessingInstruction(a_pi),
                Document::ProcessingInstruction(pattern_pi),
            ) => a_pi == pattern_pi,
            (Document::Comment(a_comment), Document::Comment(pattern_comment)) => {
                a_comment == pattern_comment
            }
            (Document::CDATA(a_cdata), Document::CDATA(pattern_cdata)) => a_cdata == pattern_cdata,

            _ => false,
        }
    }
}
impl<'a> ParseNamespace<'a> for Document {}
pub(crate) trait StrictEq {
    fn strict_eq(&self, pattern: Pattern) -> bool;
}
impl StrictEq for Document {
    fn strict_eq(&self, pattern: Pattern) -> bool {
        self == &pattern.doc
    }
}
pub trait DynamicEquality {
    fn equals(&self, pattern: Pattern, method: ComparisonMethod) -> bool;
}

pub enum ComparisonMethod {
    Partial,
    Strict,
}

impl DynamicEquality for Document {
    fn equals(&self, pattern: Pattern, method: ComparisonMethod) -> bool {
        match method {
            ComparisonMethod::Partial => self.partial_eq(pattern),

            ComparisonMethod::Strict => self.strict_eq(pattern),
        }
    }
}
