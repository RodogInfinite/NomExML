use nom::{
    bytes::complete::tag,
    combinator::opt,
    sequence::{delimited, pair, preceded},
    IResult,
};

use crate::{namespaces::ParseNamespace, parse::Parse, QualifiedName};

use super::{external_id::ExternalID, internal_subset::InternalSubset};

#[derive(Clone, PartialEq)]
pub struct DocType<'a> {
    pub name: QualifiedName<'a>,
    pub external_id: Option<ExternalID<'a>>,
    pub int_subset: Option<Vec<InternalSubset<'a>>>,
}

impl<'a> Parse<'a> for DocType<'a> {
    // [28] doctypedecl ::= '<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>'
    fn parse(input: &'a str) -> IResult<&'a str, DocType<'a>> {
        let (input, _) = tag("<!DOCTYPE")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_name(input)?;

        let (input, external_id) =
            opt(preceded(Self::parse_multispace1, ExternalID::parse))(input)?;

        let (input, _) = Self::parse_multispace0(input)?;

        let (input, int_subset) = opt(delimited(
            pair(tag("["), Self::parse_multispace0),
            InternalSubset::parse_internal_subset,
            pair(Self::parse_multispace0, tag("]")),
        ))(input)?;

        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag(">")(input)?;
        let (input, _) = Self::parse_multispace0(input)?;

        Ok((
            input,
            Self {
                name: QualifiedName {
                    prefix: None,
                    local_part: name,
                },
                external_id,
                int_subset,
            },
        ))
    }
}
impl<'a> DocType<'a> {
    // [16] doctypedecl ::= '<!DOCTYPE' S QName (S ExternalID)? S? ('[' (markupdecl | PEReference | S)* ']' S?)? '>'
    fn parse_qualified_doctype(input: &'a str) -> IResult<&'a str, DocType<'a>> {
        let (input, _) = tag("<!DOCTYPE")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_qualified_name(input)?;

        let (input, external_id) =
            opt(preceded(Self::parse_multispace1, ExternalID::parse))(input)?;

        let (input, _) = Self::parse_multispace0(input)?;

        let (input, int_subset) = opt(delimited(
            pair(tag("["), Self::parse_multispace0),
            InternalSubset::parse_internal_subset,
            pair(Self::parse_multispace0, tag("]")),
        ))(input)?;

        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag(">")(input)?;
        let (input, _) = Self::parse_multispace0(input)?;

        Ok((
            input,
            Self {
                name,
                external_id,
                int_subset,
            },
        ))
    }
}

impl<'a> ParseNamespace<'a> for DocType<'a> {}
