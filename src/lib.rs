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
    rc::Rc,
};

#[derive(Clone, PartialEq)]
pub enum Document<'a> {
    Prolog {
        xml_decl: Option<XmlDecl<'a>>,
        misc: Option<Vec<Misc<'a>>>,
        doc_type: Option<DocType<'a>>,
    },
    Element(Tag<'a>, Box<Document<'a>>, Tag<'a>),
    Content(Option<Cow<'a, str>>), //TODO: Investigate if content can ever be None. I think Empty handles this case. If so, remove the Option
    Nested(Vec<Document<'a>>),
    Empty,
    EmptyTag(Tag<'a>),
    ProcessingInstruction(ProcessingInstruction<'a>),
    Comment(Cow<'a, str>),
    CDATA(Cow<'a, str>),
}

impl<'a> Parse<'a> for Document<'a> {
    type Args = ();
    type Output = IResult<&'a str, Self>;
}

impl<'a> Document<'a> {
    //[22 prolog ::= XMLDecl? Misc* (doctypedecl Misc*)?
    pub fn parse_prolog(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<
        &'a str,
        (
            Option<Document<'a>>,
            Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
        ),
    > {
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

        let miscs: Vec<Option<Misc<'a>>> = vec![misc_before, misc_after];
        let miscs: Vec<Misc<'a>> = miscs.into_iter().flatten().collect();
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
        doc_type: &DocType<'a>,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>> {
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
    fn parse_char_data(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        map(
            tuple((take_till(|c: char| c == '<' || c == '&'), not(tag("]]>")))),
            |(data, _)| Cow::Borrowed(data),
        )(input)
    }

    // [20] CData ::= (Char* - (Char* ']]>' Char*))
    fn parse_cdata(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        map(
            cut(|i| {
                let original_input = i;
                let (input, _) = many_till(Self::parse_char, tag("]]>"))(i)?;
                let parsed_length = original_input.len() - input.len() - 3; // subtract 3 for ']]>'
                let cdata_slice = &original_input[..parsed_length];
                Ok((input, Cow::Borrowed(cdata_slice)))
            }),
            |cow| cow,
        )(input)
    }
    // [18] CDSect ::= CDStart CData CDEnd
    // [19] CDStart ::= '<![CDATA['
    //[21] CDEnd ::= ']]>'
    fn parse_cdata_section(input: &'a str) -> IResult<&'a str, Document<'a>> {
        map(
            preceded(tag("<![CDATA["), Self::parse_cdata),
            Document::CDATA,
        )(input)
    }

    // [39] element	::= EmptyElemTag | STag content ETag
    pub fn parse_element(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, Document<'a>> {
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
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> impl Fn(Vec<Reference<'a>>) -> Document<'a> + 'a {
        move |references| {
            let mut contents: Vec<Cow<'a, str>> = Vec::new();
            for reference in references.into_iter() {
                match reference.normalize_entity(entity_references.clone()) {
                    EntityValue::Document(doc) => return doc, // If we encounter a Document, return it immediately.
                    EntityValue::Value(val) => contents.push(val),
                    _ => {}
                }
            }
            // Join the contents into a single string
            let content = contents
                .into_iter()
                .map(Cow::into_owned)
                .collect::<String>();
            Document::Content(Some(Cow::Owned(content)))
        }
    }

    // TODO: add validation for elements using the ConditionalState in the ContentParticle from the prolog
    // [43] content ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
    fn parse_content(
        input: &'a str,
        entity_references: Rc<RefCell<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, Document<'a>> {
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
    pub fn parse_comment(input: &'a str) -> IResult<&'a str, Document<'a>> {
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
                    Ok(Document::Comment(Cow::Owned(comment_string)))
                }
            },
        )(input)
    }

    pub fn parse_xml_str(input: &'a str) -> IResult<&'a str, Document<'a>> {
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
                opt(|i| Tag::parse_start_tag(i, new_entity_references.clone()))(current_input)?;
            let (input, content) = Self::parse_content(input, new_entity_references.clone())?;
            let (input, end_tag) = opt(Tag::parse_end_tag)(input)?;

            let empty_tag = if let Document::EmptyTag(empty_tag) = &content {
                Some(empty_tag.clone())
            } else {
                None
            };
            let (input, doc) =
                Self::construct_document_element(input, start_tag, content, end_tag, empty_tag)?;
            if let Document::Empty = &doc {
                break;
            }

            documents.push(doc);
            current_input = input;
        }

        let (input, documents) = Self::construct_document(input, prolog, documents)?;

        Ok((input, documents))
    }

    fn construct_document_element(
        input: &'a str,
        start_tag: Option<Tag<'a>>,
        content: Document<'a>,
        end_tag: Option<Tag<'a>>,
        empty_tag: Option<Tag<'a>>,
    ) -> IResult<&'a str, Document<'a>> {
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
        input: &'a str,
        prolog: Option<Document<'a>>,
        documents: Vec<Document<'a>>,
    ) -> IResult<&'a str, Document<'a>> {
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
pub struct QualifiedName<'a> {
    pub prefix: Option<Cow<'a, str>>,
    pub local_part: Cow<'a, str>,
}
pub type Name<'a> = QualifiedName<'a>;

impl<'a> QualifiedName<'a> {
    pub fn new(prefix: Option<&'a str>, local_part: &'a str) -> Self {
        if let Some(prefix) = prefix {
            Self {
                prefix: Some(Cow::Borrowed(prefix)),
                local_part: Cow::Borrowed(local_part),
            }
        } else {
            Self {
                prefix: None,
                local_part: Cow::Borrowed(local_part),
            }
        }
    }
}

impl<'a> ParseNamespace<'a> for Document<'a> {}

impl<'a> Document<'a> {
    pub fn extract(&self, tag: &QualifiedName<'a>) -> Result<Vec<Document<'a>>, Box<dyn Error>> {
        let mut result = Vec::new();

        match self {
            Document::Element(start_tag, inner_doc, _end_tag) => {
                if &start_tag.name == tag {
                    result.push(self.clone());
                }
                result.extend(inner_doc.extract(tag)?);
            }
            Document::Nested(docs) => {
                for doc in docs {
                    result.extend(doc.extract(tag)?);
                }
            }
            _ => {} // Handle other Document variants if needed
        }

        Ok(result)
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

    pub fn get_duplicate_subtags(
        &self,
        outer_tag: &str,
        inner_tag: &str,
    ) -> Result<BTreeMap<(String, usize), BTreeMap<String, String>>, Box<dyn Error>> {
        let result = self
            .extract(&Name {
                prefix: None,
                local_part: Cow::Owned(outer_tag.into()),
            })?
            .iter()
            .map(|doc| {
                doc.extract(&Name {
                    prefix: None,
                    local_part: Cow::Owned(inner_tag.into()),
                })
                .as_indexed_map()
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<BTreeMap<_, _>>();

        Ok(result)
    }
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
    ) -> Result<BTreeMap<(String, usize), BTreeMap<String, String>>, Box<dyn Error>>;
}

impl<'a> AsOrderedMap for Document<'a> {
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

    fn as_indexed_map(
        &self,
    ) -> Result<BTreeMap<(String, usize), BTreeMap<String, String>>, Box<dyn Error>> {
        Err("Not applicable for non-nested Document. Try `as_map`".into())
    }
}

impl<'a> AsOrderedMap for Result<Vec<Document<'a>>, Box<dyn Error>> {
    fn as_map(&self) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
        Err("Not applicable for Vec<Document>. Try `as_indexed_map`".into())
    }

    fn as_indexed_map(
        &self,
    ) -> Result<BTreeMap<(String, usize), BTreeMap<String, String>>, Box<dyn Error>> {
        let mut map = BTreeMap::new();

        if let Ok(docs) = self {
            for (index, doc) in docs.iter().enumerate() {
                if let Document::Element(tag, _content, _) = doc {
                    let tag_name = tag.name.local_part.to_string();
                    let content_map = doc.as_map()?; // renamed to avoid shadowing
                    map.insert((tag_name, index), content_map);
                }
            }
            Ok(map)
        } else {
            Err("An error occurred while processing the documents.".into())
        }
    }
}
