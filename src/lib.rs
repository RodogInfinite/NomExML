pub mod attribute;
mod debug;
pub mod decode;
mod error;
pub mod io;
pub mod misc;
pub mod namespaces;
pub mod parse;
pub mod processing_instruction;
pub mod prolog;
pub mod reference;
pub mod tag;

use crate::misc::MiscState;
use crate::processing_instruction::ProcessingInstruction;
use crate::prolog::doctype::DocType;
use crate::prolog::xmldecl::XmlDecl;
use crate::reference::Reference;
use crate::tag::Tag;
use crate::{misc::Misc, parse::Parse};
use attribute::Attribute;

use namespaces::ParseNamespace;
use nom::combinator::peek;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    combinator::{map, not, opt, verify},
    multi::{many0, many_till},
    sequence::{pair, tuple},
    IResult,
};
use prolog::internal_subset::{EntityDeclaration, EntityDefinition, EntityValue, InternalSubset};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub enum Document<'a> {
    Prolog {
        xml_decl: Option<XmlDecl<'a>>,
        misc: Option<Vec<Misc<'a>>>,
        doc_type: Option<DocType<'a>>,
    },
    Element(Tag<'a>, Box<Document<'a>>, Tag<'a>),
    Content(Option<Cow<'a, str>>),
    Nested(Vec<Document<'a>>),
    Empty,
    ProcessingInstruction(ProcessingInstruction<'a>),
    Comment(Cow<'a, str>),
    CDATA(Cow<'a, str>),
}

impl<'a> Parse<'a> for Document<'a> {}

impl<'a> Document<'a> {
    //[22 prolog ::= XMLDecl? Misc* (doctypedecl Misc*)?
    pub fn parse_prolog(
        input: &'a str,
    ) -> IResult<
        &'a str,
        (
            Option<Document<'a>>,
            Option<Rc<HashMap<Name<'a>, EntityValue<'a>>>>,
        ),
    > {
        let (input, xml_decl) = opt(XmlDecl::parse)(input)?;
        let (input, misc_before) =
            opt(|input| Misc::parse(input, MiscState::BeforeDoctype))(input)?;

        let (input, doc_type) = opt(DocType::parse)(input)?;
        let entity_references = match &doc_type {
            Some(dt) => Self::collect_entity_references(dt),
            None => None,
        };

        let (input, misc_after) = match &doc_type {
            Some(_) => opt(|input| Misc::parse(input, MiscState::AfterDoctype))(input)?,
            None => (input, None),
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

        Ok((input, (prolog, entity_references)))
    }

    fn collect_entity_references(
        doc_type: &DocType<'a>,
    ) -> Option<Rc<HashMap<Name<'a>, EntityValue<'a>>>> {
        let mut entity_references = HashMap::new();

        if let Some(int_subset) = &doc_type.int_subset {
            for internal_subset in int_subset {
                if let InternalSubset::Entity(EntityDeclaration::General(decl)) = internal_subset {
                    if let EntityDefinition::EntityValue(value) = &decl.entity_def {
                        entity_references.insert(decl.name.clone(), value.clone());
                    }
                }
            }
        }

        if entity_references.is_empty() {
            None
        } else {
            Some(Rc::new(entity_references))
        }
    }

    // [14] CharData ::= [^<&]* - ([^<&]* ']]>' [^<&]*)
    fn parse_char_data(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, data) = take_till(|c: char| c == '<' || c == '&')(input)?;
        let (input, _) = not(tag("]]>"))(input)?;
        Ok((input, Cow::Borrowed(data)))
    }

    // [18] CDSect ::= CDStart CData CDEnd
    // [19] CDStart ::= '<![CDATA['
    // [20] CData ::= (Char* - (Char* ']]>' Char*))
    fn parse_cdata(input: &'a str) -> IResult<&'a str, Cow<'a, str>> {
        let (input, (data, _)) = many_till(Self::parse_char, tag("]]>"))(input)?;
        let data: String = data.into_iter().collect();

        Ok((input, Cow::Owned(data)))
    }
    //[21] CDEnd ::= ']]>'
    fn parse_cdata_section(input: &'a str) -> IResult<&'a str, Document<'a>> {
        let (input, _) = tag("<![CDATA[")(input)?;
        let (input, cdata_content) = Self::parse_cdata(input)?;
        let cdata_string: String = cdata_content.to_string();
        Ok((input, Document::CDATA(Cow::Owned(cdata_string))))
    }

    // [39] element	::= EmptyElemTag | STag content ETag
    pub fn parse_element(
        input: &'a str,
        entity_references: Option<Rc<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, Document<'a>> {
        let (input, doc) = alt((
            map(Tag::parse_empty_element_tag, |tag| {
                Document::Element(tag.clone(), Box::new(Document::Empty), tag.clone())
            }),
            map(
                tuple((
                    Self::parse_multispace0, // this is not adhering strictly to the spec, but handles the case where there is whitespace before the start tag for human readability
                    Tag::parse_start_tag,
                    |i| Self::parse_content(i, entity_references.clone()),
                    Tag::parse_end_tag,
                    Self::parse_multispace0, // this is not adhering strictly to the spec, but handles the case where there is whitespace after the start tag for human readability
                )),
                |(_, start_tag, content, end_tag, _)| {
                    Document::Element(start_tag, Box::new(content), end_tag)
                },
            ),
        ))(input)?;
        Ok((input, doc))
    }

    // [43] content ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
    fn parse_content(
        input: &'a str,
        entity_references: Option<Rc<HashMap<Name<'a>, EntityValue<'a>>>>,
    ) -> IResult<&'a str, Document<'a>> {
        let (input, ((_, maybe_chardata), elements)) = tuple((
            pair(
                Self::parse_multispace0, // this is not strictly adhering to the standard; however, it prevents the first Nested element from being Nested([Content(" ")])
                opt(Self::parse_char_data),
            ),
            many0(pair(
                alt((
                    |i| Self::parse_element(i, entity_references.clone()),
                    map(many1(Reference::parse), |references| {
                        let content: String = references
                            .into_iter()
                            .map(|reference| match reference {
                                Reference::EntityRef(name) => {
                                    if let Some(refs) = &entity_references {
                                        if let Some(EntityValue::Value(value)) = refs.get(&name) {
                                            value.clone()
                                        } else {
                                            name.local_part
                                        }
                                    } else {
                                        name.local_part
                                    }
                                }
                                Reference::CharRef { value, .. } => value,
                            })
                            .collect();
                        println!("CONTENT: {content:}");
                        Document::Content(Some(Cow::Owned(content)))
                    }),
                    Self::parse_cdata_section,
                    map(
                        ProcessingInstruction::parse,
                        Document::ProcessingInstruction,
                    ),
                    Self::parse_comment,
                )),
                pair(
                    Self::parse_multispace0, // this is not strictly adhering to the standard; however, it prevents the first Nested element from being Nested([Content(" ")])
                    opt(Self::parse_char_data),
                ),
            )),
        ))(input)?;
        let mut content = elements
            .into_iter()
            .flat_map(|(doc, maybe_chardata)| {
                let mut vec = Vec::new();
                println!("DOC: {:?}", doc);
                vec.push(doc);
                if let (_, Some(chardata)) = maybe_chardata {
                    if !chardata.is_empty() {
                        println!("CHARDATA: {:?}", chardata);
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
                            _ => Document::Nested(content),
                        }
                    }
                }
            },
        ))
    }

    // [15] Comment ::= '<!--' ((Char - '-') | ('-' (Char - '-')))* '-->'
    pub fn parse_comment(input: &'a str) -> IResult<&'a str, Document<'a>> {
        let (input, _) = tag("<!--")(input)?;

        let (input, (comment_content, _)) = many_till(Self::parse_char, tag("-->"))(input)?;
        let comment_string: String = comment_content.into_iter().collect();
        if comment_string.contains("--") {
            return Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }
        Ok((input, Document::Comment(Cow::Owned(comment_string))))
    }

    pub fn parse_xml_str(input: &'a str) -> IResult<&'a str, Document<'a>> {
        let (input, prolog_and_references) = opt(Self::parse_prolog)(input)?;

        let (prolog, entity_references) = match prolog_and_references {
            Some((p, r)) => (p, r),
            None => (None, None),
        };

        let (input, start_tag) = Tag::parse_start_tag(input)?;
        let (input, content) = Self::parse_content(input, entity_references.clone())?;
        let (input, end_tag) = Tag::parse_end_tag(input)?;

        Self::construct_document(input, prolog, start_tag, content, end_tag)
    }

    fn construct_document(
        input: &'a str,
        prolog: Option<Document<'a>>,
        start_tag: Tag<'a>,
        content: Document<'a>,
        end_tag: Tag<'a>,
    ) -> IResult<&'a str, Document<'a>> {
        match (&start_tag, &end_tag) {
            (
                Tag {
                    name: start_name, ..
                },
                Tag { name: end_name, .. },
            ) if start_name == end_name => {
                let document = match prolog {
                    Some(prolog) => Document::Nested(vec![
                        prolog,
                        Document::Element(start_tag.clone(), Box::new(content), end_tag.clone()),
                    ]),
                    None => {
                        Document::Element(start_tag.clone(), Box::new(content), end_tag.clone())
                    }
                };

                Ok((input, document))
            }
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            ))),
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

    pub fn get_attributes(&self) -> HashMap<String, String> {
        let mut results = HashMap::new();

        if let Document::Element(tag, inner_doc, _) = self {
            if let Some(attributes) = &tag.attributes {
                for attribute in attributes {
                    if let Attribute::Instance { name, value } = attribute {
                        let attr_name = name.local_part.to_string();
                        let attr_value = value.to_string();
                        results.insert(attr_name, attr_value);
                    }
                }
            }

            if let Document::Nested(docs) = &**inner_doc {
                for doc in docs {
                    let mut inner_results = doc.get_attributes();
                    results.extend(inner_results.drain());
                }
            }
        } else if let Document::Nested(docs) = self {
            for doc in docs {
                let mut inner_results = doc.get_attributes();
                results.extend(inner_results.drain());
            }
        }

        results
    }

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

        match self {
            Ok(docs) => {
                for (index, doc) in docs.iter().enumerate() {
                    match doc {
                        Document::Element(tag, content, _) => {
                            let tag_name = tag.name.local_part.to_string();
                            let mut content = doc.as_map()?;
                            map.insert((tag_name, index), content);
                        }
                        _ => {}
                    }
                }
                Ok(map)
            }
            Err(e) => Err(e.to_string().into()),
        }
    }
}
