pub mod attribute;
mod debug;
mod error;
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
    misc::{Misc, MiscState},
    parse::Parse,
    processing_instruction::ProcessingInstruction,
    prolog::{
        doctype::DocType,
        internal_subset::{
            entity_declaration::{EntityDecl, EntityDeclaration},
            entity_definition::EntityDefinition,
            InternalSubset,
        },
        xmldecl::XmlDecl,
    },
    reference::Reference,
    tag::Tag,
    transcode::Decode,
};
use attribute::Attribute;
use namespaces::ParseNamespace;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    combinator::{cut, map, map_res, not, opt, value},
    multi::{many0, many1, many_till},
    sequence::{pair, preceded, tuple},
    IResult,
};
use prolog::internal_subset::entity_value::EntityValue;
use std::{
    borrow::Cow,
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    error::Error,
    io::Write,
    rc::Rc,
};

#[derive(Clone, Default, Debug)]
pub struct ExternalEntityParseConfig {
    pub allow_ext_parse: bool,
    pub ignore_ext_parse_warning: bool,
}

#[derive(Clone, Default, Debug)]
pub struct Config {
    pub external_parse_config: ExternalEntityParseConfig,
}

#[derive(Clone, PartialEq)]
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

macro_rules! warnln {
    ($($arg:tt)*) => ({
        eprintln!("\x1B[33mWARNING:\x1B[0m {}", format!($($arg)*));
    });
}

fn check_config(config: &Config) -> Result<(), nom::Err<&'static str>> {
    match config {
        Config {
            external_parse_config:
                ExternalEntityParseConfig {
                    allow_ext_parse: true,
                    ignore_ext_parse_warning: false,
                },
        } => {
            warnln!("The configuration `{:?}` allows external entity parsing which might expose the system to an XML External Entity (XXE) attack.\nThis crate makes no guarantees for security in this regard so make sure you trust your sources.\nVerification of all `.ent` files is strongly recommended.", config);

            loop {
                print!("Do you wish to proceed? [y/n]: ");
                std::io::stdout().flush().unwrap();

                let mut decision = String::new();
                std::io::stdin().read_line(&mut decision).unwrap();

                match decision.trim().to_lowercase().as_str() {
                    "y" | "Y" | "yes" => break,
                    "n" | "N" | "no" => {
                        return Err(nom::Err::Error(
                            "User decided to stop due to potential XXE attack",
                        ));
                    }
                    _ => eprintln!("Invalid input. Please type 'y' or 'n'"),
                }
            }
        }
        Config {
            external_parse_config:
                ExternalEntityParseConfig {
                    allow_ext_parse: false,
                    ignore_ext_parse_warning: true,
                },
        } => {
            warnln!("The configuration `{:?}` may allow for unexpected parsing if `allow_ext_parse` is changed to true in the future", config);
        }
        _ => (),
    }
    Ok(())
}

impl<'a> Parse<'a> for Document {
    type Args = Config;
    type Output = IResult<&'a str, Self>;
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        match check_config(&args) {
            Ok(_) => {
                let entity_references = Rc::new(RefCell::new(HashMap::new()));
                let (input, prolog_and_references) =
                    opt(|i| Self::parse_prolog(i, entity_references.clone()))(input)?;

                let (prolog, new_entity_references) = match prolog_and_references {
                    Some((p, r)) => (p, r),
                    None => (None, entity_references.clone()),
                };

                let mut documents = Vec::new();

                let mut current_input = input;
                while !current_input.is_empty() {
                    let (input, start_tag) =
                        opt(|i| Tag::parse_start_tag(i, new_entity_references.clone()))(
                            current_input,
                        )?;
                    let (input, content) =
                        Self::parse_content(input, new_entity_references.clone())?;
                    let (input, end_tag) = opt(Tag::parse_end_tag)(input)?;

                    let empty_tag = if let Document::EmptyTag(empty_tag) = &content {
                        Some(empty_tag.clone())
                    } else {
                        None
                    };
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
            Err(nom::Err::Error(err_msg)) => {
                Err(nom::Err::Error(nom::error::Error {
                    input: err_msg,
                    code: nom::error::ErrorKind::Fail, // Or any other ErrorKind that is suitable for your case
                }))
            }
            Err(nom::Err::Incomplete(needed)) => {
                // handle the Incomplete case, if needed, or return an appropriate error
                Err(nom::Err::Incomplete(needed))
            }
            Err(nom::Err::Failure(err_msg)) => {
                Err(nom::Err::Failure(nom::error::Error {
                    input: err_msg,
                    code: nom::error::ErrorKind::Fail, // or another appropriate ErrorKind
                }))
            }
        }
    }
}

impl Document {
    // fn iter(&self) -> Box<dyn Iterator<Item = &Document> + '_> {
    //     match self {
    //         Document::Nested(docs) => Box::new(docs.iter()),
    //         _ => Box::new(std::iter::empty::<&Document>()),
    //     }
    // }
    //[22 prolog ::= XMLDecl? Misc* (doctypedecl Misc*)?
    pub fn parse_prolog(
        input: &str,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> IResult<&str, (Option<Document>, Rc<RefCell<HashMap<Name, EntityValue>>>)> {
        let (input, xml_decl) = opt(|i| XmlDecl::parse(i, ()))(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, misc_before) =
            opt(|input| Misc::parse(input, MiscState::BeforeDoctype))(input)?;

        let (input, doc_type) = opt(|i| DocType::parse(i, entity_references.clone()))(input)?;

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

    fn collect_entity_references(
        doc_type: &DocType,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> Rc<RefCell<HashMap<Name, EntityValue>>> {
        if let InternalSubset::Entities(entities) = &doc_type.get_entities() {
            for boxed_entity in entities {
                if let InternalSubset::Entity(entity_decl) = &**boxed_entity {
                    match entity_decl {
                        EntityDecl::General(decl) | EntityDecl::Parameter(decl) => {
                            if let EntityDefinition::EntityValue(value) = &decl.entity_def {
                                // Check if the name already exists in the map
                                let mut references = entity_references.borrow_mut();
                                if !references.contains_key(&decl.name) {
                                    references.insert(decl.name.clone(), value.clone());
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

    // [14] CharData ::= [^<&]* - ([^<&]* ']]>' [^<&]*)
    fn parse_char_data(input: &str) -> IResult<&str, String> {
        map(
            tuple((
                take_till(|c: char| c == '<' || c == '&'),
                not(tag::<_, &str, _>("]]>")),
            )),
            |(data, _)| data.to_string(),
        )(input)
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
    pub fn parse_element(
        input: &str,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> IResult<&str, Document> {
        let (input, doc) = alt((
            preceded(
                Self::parse_multispace0, // this is not adhering strictly to the spec, but handles the case where there is whitespace before the start tag for human readability
                map(
                    |i| Tag::parse_empty_element_tag(i, entity_references.clone()),
                    |tag| Document::EmptyTag(tag.clone()),
                ),
            ),
            map(
                tuple((
                    Self::parse_multispace0, // this is not adhering strictly to the spec, but handles the case where there is whitespace before the start tag for human readability
                    |i| Tag::parse_start_tag(i, entity_references.clone()),
                    |i| Self::parse_content(i, entity_references.clone()),
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

    pub fn process_references(
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> impl Fn(Vec<Reference>) -> Document {
        move |references| {
            let mut contents: Vec<String> = Vec::new();
            for reference in references.into_iter() {
                match reference.normalize_entity(entity_references.clone()) {
                    EntityValue::Document(doc) => return doc, // If we encounter a Document, return it immediately.
                    EntityValue::Value(val) => contents.push(val),
                    _ => {}
                }
            }
            // Join the contents into a single string
            let content = contents.concat();
            Document::Content(Some(content))
        }
    }

    // TODO: add validation for elements using the ConditionalState in the ContentParticle from the prolog
    // [43] content ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
    fn parse_content(
        input: &str,
        entity_references: Rc<RefCell<HashMap<Name, EntityValue>>>,
    ) -> IResult<&str, Document> {
        let (input, ((_whitespace, maybe_chardata), elements)) = tuple((
            pair(
                Self::parse_multispace0, // this is not strictly adhering to the standard; however, it prevents the first Nested element from being Nested([Content(" ")])
                opt(Self::parse_char_data),
            ),
            many0(alt((
                pair(
                    map(
                        many1(|i| Reference::parse(i, entity_references.clone())), // TODO this is returning the bracket for &#60;doc>
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
                            _ => Document::Nested(content),
                        }
                    }
                }
            },
        ))
    }

    // [15] Comment ::= '<!--' ((Char - '-') | ('-' (Char - '-')))* '-->'
    pub fn parse_comment(input: &str) -> IResult<&str, Document> {
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
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Verify,
                    )));
                }

                let document = Document::Element(start, Box::new(content), end);

                Ok((input, document))
            }
            (Some(start), Some(end), _, Some(empty_tag)) => {
                if start.name != end.name {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Verify,
                    )));
                }

                let document =
                    Document::Element(start, Box::new(Document::EmptyTag(empty_tag)), end);

                Ok((input, document))
            }
            (Some(_), None, Document::Element(start, inner_content, end), None) => {
                if start.name != end.name {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Verify,
                    )));
                }

                let document = Document::Element(start, inner_content, end);

                Ok((input, document))
            }
            (None, None, Document::Element(start, inner_content, end), None) => {
                if start.name != end.name {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Verify,
                    )));
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
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            ))),
        }
    }

    fn construct_document(
        input: &str,
        prolog: Option<Document>,
        documents: Vec<Document>,
    ) -> IResult<&str, Document> {
        match documents.len() {
            0 => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            ))),
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
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct QualifiedName {
    pub prefix: Option<String>,
    pub local_part: String,
}
pub type Name = QualifiedName;

impl QualifiedName {
    pub fn new(prefix: Option<&str>, local_part: &str) -> Self {
        Self {
            prefix: prefix.map(|p| p.to_string()),
            local_part: local_part.to_string(),
        }
    }
}

impl<'a> ParseNamespace<'a> for Document {}

impl Document {
    pub fn extract(&self, tag: &QualifiedName) -> Result<Document, Box<dyn Error>> {
        let mut documents: Vec<Document> = Vec::new();

        match self {
            Document::Element(start_tag, inner_doc, _end_tag) => {
                if &start_tag.name == tag {
                    documents.push(self.clone());
                }

                match inner_doc.extract(tag) {
                    Ok(Document::Nested(inner_docs)) => documents.extend(inner_docs),
                    Ok(single_doc) => documents.push(single_doc),
                    Err(_) => {}
                }
            }
            Document::Nested(docs) => {
                for doc in docs {
                    match doc.extract(tag) {
                        Ok(Document::Nested(inner_docs)) => documents.extend(inner_docs),
                        Ok(single_doc) => documents.push(single_doc),
                        Err(_) => {}
                    }
                }
            }
            _ => {} // Handle other Document variants if needed
        }

        if documents.is_empty() {
            return Err(Box::new(DocumentError::NoMatchingDocuments));
        }

        match documents.as_slice() {
            [document] => Ok(document.clone()),
            _ => Ok(Document::Nested(documents)),
        }
    }

    pub fn get_content(&self) -> HashMap<String, String> {
        let mut results = HashMap::new();

        match self {
            Document::Element(tag, inner_doc, _) => {
                let tag_name = tag.name.local_part.to_string();
                match &**inner_doc {
                    Document::Content(Some(content)) => {
                        results.insert(tag_name, content.to_string());
                    }
                    Document::Nested(docs) => {
                        for doc in docs {
                            let mut inner_results = doc.get_content();
                            results.extend(inner_results.drain());
                        }
                    }
                    _ => {}
                }
            }
            Document::Nested(docs) => {
                for doc in docs {
                    let mut inner_results = doc.get_content();
                    results.extend(inner_results.drain());
                }
            }
            _ => {}
        }

        results
    }

    pub fn extract_enumerated_subtags(
        &self,
        outer_tag: &str,
        inner_tag: &str,
    ) -> Result<BTreeMap<(String, usize), BTreeMap<String, String>>, Box<dyn Error>> {
        let extracted = self.extract(&Name {
            prefix: None,
            local_part: outer_tag.to_string(),
        })?;

        if let Document::Element(_, inner_doc, _) = &extracted {
            if let Document::Nested(inner_docs) = &**inner_doc {
                let indexed_map = inner_docs.as_indexed_map(inner_tag)?;
                Ok(indexed_map)
            } else {
                Err(Box::new(DocumentError::ExpectedNestedDocument))
            }
        } else if let Document::Nested(inner_docs) = &extracted {
            let indexed_map = inner_docs.as_indexed_map(inner_tag)?;
            Ok(indexed_map)
        } else {
            Err(Box::new(DocumentError::ExpectedNestedDocument))
        }
    }

    pub fn extract_subtags_using_inner_key(
        &self,
        outer_tag: &str,
        inner_tag: &str,
        inner_tag_subtag_key: &str,
    ) -> Result<BTreeMap<String, BTreeMap<String, String>>, Box<dyn Error>> {
        let extracted = self.extract(&Name {
            prefix: None,
            local_part: outer_tag.to_string(),
        })?;

        if let Document::Element(_, inner_doc, _) = &extracted {
            if let Document::Nested(inner_docs) = &**inner_doc {
                let map_with_subtag_key =
                    inner_docs.as_map_with_subtag_value_key(inner_tag, inner_tag_subtag_key)?;
                Ok(map_with_subtag_key)
            } else {
                Err(Box::new(DocumentError::ExpectedNestedDocument))
            }
        } else if let Document::Nested(inner_docs) = &extracted {
            let map_with_subtag_key =
                inner_docs.as_map_with_subtag_value_key(inner_tag, inner_tag_subtag_key)?;
            Ok(map_with_subtag_key)
        } else {
            Err(Box::new(DocumentError::ExpectedNestedDocument))
        }
    }

    //TODO FIX THIS
    // pub fn get_attributes(&self) -> HashMap<String, String> {
    //     let mut results = HashMap::new();

    //     if let Document::Element(tag, inner_doc, _) = self {
    //         if let Some(attributes) = &tag.attributes {
    //             for attribute in attributes {
    //                 if let Attribute::Instance { name, value } = attribute {
    //                     let attr_name = name.local_part.to_string();

    //                     let attr_value = value.to_string();
    //                     results.insert(attr_name, attr_value);
    //                 }
    //             }
    //         }

    //         if let Document::Nested(docs) = &**inner_doc {
    //             for doc in docs {
    //                 let mut inner_results = doc.get_attributes();
    //                 results.extend(inner_results.drain());
    //             }
    //         }
    //     } else if let Document::Nested(docs) = self {
    //         for doc in docs {
    //             let mut inner_results = doc.get_attributes();
    //             results.extend(inner_results.drain());
    //         }
    //     }

    //     results
    // }
}

#[derive(Clone, Debug, PartialEq)]
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

pub trait AsOrderedMap {
    fn as_map(&self) -> Result<BTreeMap<String, String>, Box<dyn Error>>;
    fn as_indexed_map(
        &self,
        target_tag_name: &str,
    ) -> Result<BTreeMap<(String, usize), BTreeMap<String, String>>, Box<dyn Error>>;
    fn as_map_with_subtag_value_key(
        &self,
        target_tag_name: &str,
        subtag_key: &str,
    ) -> Result<BTreeMap<String, BTreeMap<String, String>>, Box<dyn Error>>;
    fn as_map_with_excluded_key(
        &self,
        excluded_key: &str,
    ) -> Result<(String, BTreeMap<String, String>), Box<dyn Error>>;
}

impl AsOrderedMap for Document {
    fn as_map(&self) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
        let mut map = BTreeMap::new();

        let content = self.get_content();
        for (key, value) in content {
            if map.contains_key(&key) {
                return Err(format!("Duplicate key: {}", key).into());
            }

            map.insert(key, value);
        }

        Ok(map)
    }
    fn as_map_with_excluded_key(
        &self,
        excluded_key: &str,
    ) -> Result<(String, BTreeMap<String, String>), Box<dyn Error>> {
        let mut map = BTreeMap::new();
        let mut excluded_value: Option<String> = None;

        let content = self.get_content();
        for (key, value) in content {
            if key == excluded_key {
                excluded_value = Some(value);
                continue;
            }

            map.insert(key, value);
        }

        if let Some(ev) = excluded_value {
            Ok((ev, map))
        } else {
            Err(format!("Key '{}' not found.", excluded_key).into())
        }
    }

    fn as_indexed_map(
        &self,
        _target_tag_name: &str,
    ) -> Result<BTreeMap<(String, usize), BTreeMap<String, String>>, Box<dyn Error>> {
        Err("Not applicable for non-nested Document. Try `as_map`".into())
    }
    fn as_map_with_subtag_value_key(
        &self,
        _target_tag_name: &str,
        _subtag_key: &str,
    ) -> Result<BTreeMap<String, BTreeMap<String, String>>, Box<dyn Error>> {
        Err("Not applicable for non-nested Document. Try `as_map`".into())
    }
}

impl AsOrderedMap for Result<Vec<Document>, Box<dyn Error>> {
    fn as_map(&self) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
        Err("Not applicable for Result<Vec<Document>, Box<dyn Error>>. Try `as_indexed_map`".into())
    }
    fn as_map_with_excluded_key(
        &self,
        _excluded_key: &str,
    ) -> Result<(String, BTreeMap<String, String>), Box<dyn Error>> {
        Err("Not applicable for Result<Vec<Document>, Box<dyn Error>>.".into())
    }

    fn as_indexed_map(
        &self,
        target_tag_name: &str,
    ) -> Result<BTreeMap<(String, usize), BTreeMap<String, String>>, Box<dyn Error>> {
        let mut map = BTreeMap::new();
        let mut tag_index = 0; // to keep the current index for the given tag_name

        if let Ok(docs) = self {
            for doc in docs.iter() {
                if let Document::Element(tag, _content, _) = doc {
                    let current_tag_name = tag.name.local_part.to_string();

                    if current_tag_name == target_tag_name {
                        let content_map = doc.as_map()?;

                        map.insert((current_tag_name.clone(), tag_index), content_map);

                        // Increment the index for this tag_name only
                        tag_index += 1;
                    }
                }
            }

            Ok(map)
        } else {
            Err("An error occurred while processing the documents.".into())
        }
    }

    fn as_map_with_subtag_value_key(
        &self,
        target_tag_name: &str,
        subtag_key: &str,
    ) -> Result<BTreeMap<String, BTreeMap<String, String>>, Box<dyn Error>> {
        let mut map = BTreeMap::new();

        if let Ok(docs) = self {
            for doc in docs.iter() {
                if let Document::Element(tag, _content, _) = doc {
                    let current_tag_name = tag.name.local_part.to_string();

                    if current_tag_name == target_tag_name {
                        let (key_value, content_map) = doc.as_map_with_excluded_key(subtag_key)?;

                        if map.insert(key_value.clone(), content_map).is_some() {
                            warnln!(
                                "`as_map_with_subtag_value(\"{target_tag_name}\",\"{subtag_key}\")` Duplicate key found: \"{subtag_key}\":\"{key_value}\"'. Overwriting previous value.\n",
                            );
                        }
                    }
                }
            }

            Ok(map)
        } else {
            Err("An error occurred while processing the documents.".into())
        }
    }
}

impl AsOrderedMap for Vec<Document> {
    fn as_map(&self) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
        Err("Not applicable for Vec<Document>. Try `as_indexed_map`".into())
    }
    fn as_map_with_excluded_key(
        &self,
        _excluded_key: &str,
    ) -> Result<(String, BTreeMap<String, String>), Box<dyn Error>> {
        Err("Not applicable for Vec<Document>.".into())
    }
    fn as_indexed_map(
        &self,
        target_tag_name: &str,
    ) -> Result<BTreeMap<(String, usize), BTreeMap<String, String>>, Box<dyn Error>> {
        let mut map = BTreeMap::new();
        let mut tag_index = 0; // track the current index for the given tag_name

        for doc in self.iter() {
            if let Document::Element(tag, _content, _) = doc {
                let current_tag_name = tag.name.local_part.to_string();

                // Only process elements with the provided tag name
                if current_tag_name == target_tag_name {
                    let content_map = doc.as_map()?;

                    map.insert((current_tag_name, tag_index), content_map);

                    // increment the index for this tag_name
                    tag_index += 1;
                }
            }
        }

        Ok(map)
    }
    fn as_map_with_subtag_value_key(
        &self,
        target_tag_name: &str,
        subtag_key: &str,
    ) -> Result<BTreeMap<String, BTreeMap<String, String>>, Box<dyn Error>> {
        let mut map = BTreeMap::new();

        for doc in self.iter() {
            if let Document::Element(tag, _content, _) = doc {
                let current_tag_name = tag.name.local_part.to_string();

                if current_tag_name == target_tag_name {
                    let (key_value, content_map) = doc.as_map_with_excluded_key(subtag_key)?;

                    // No need to extract the subtag's value for the key anymore
                    // because we get it directly from the updated function
                    if map.insert(key_value.clone(), content_map).is_some() {
                        warnln!(
                            "`as_map_with_subtag_value(\"{target_tag_name}\",\"{subtag_key}\")` Duplicate key found: \"{subtag_key}\":\"{key_value}\"'. Overwriting previous value.\n",
                            
                        );
                    }
                }
            }
        }

        Ok(map)
    }
}

use std::fmt;

#[derive(Debug)]
pub enum DocumentError {
    NoMatchingDocuments,
    ExpectedNestedDocument, // Other error variants can be added as needed
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DocumentError::NoMatchingDocuments => {
                write!(f, "No matching documents found during extraction")
            }
            DocumentError::ExpectedNestedDocument => {
                write!(f, "Expected a nested document, but found another variant")
            } // Handle other error variants here as needed
        }
    }
}

impl std::error::Error for DocumentError {}
