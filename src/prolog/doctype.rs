use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    IResult,
};

use crate::parse::Parse;

use super::{internal_subset::InternalSubset, ExternalID};

#[derive(Clone, PartialEq)]
pub struct DocType<'a> {
    pub name: Cow<'a, str>,
    pub external_id: Option<ExternalID>,
    pub int_subset: Option<Vec<InternalSubset<'a>>>,
}

impl<'a> Parse<'a> for DocType<'a> {
    // [28] doctypedecl	::= '<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>'
    fn parse(input: &'a str) -> IResult<&'a str, DocType<'a>> {
        let (input, _) = tag("<!DOCTYPE")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_name(input)?;

        let (input, _) = Self::parse_multispace0(input)?;
        let (input, external_id) = opt(alt((
            map(tag("SYSTEM"), |_| ExternalID::System),
            map(tag("PUBLIC"), |_| ExternalID::Public),
        )))(input)?;

        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag("[")(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, int_subset) = opt(InternalSubset::parse_internal_subset)(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag("]")(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        let (input, _) = tag(">")(input)?;
        let (input, _) = Self::parse_multispace0(input)?;
        Ok((
            input,
            Self {
                name,
                external_id,
                int_subset: int_subset,
            },
        ))
    }
}
