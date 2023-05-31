use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use crate::parse::Parse;

use super::{external_id::ExternalID, internal_subset::InternalSubset};

#[derive(Clone, PartialEq)]
pub struct DocType<'a> {
    pub name: Cow<'a, str>,
    pub external_id: Option<ExternalID<'a>>,
    pub int_subset: Option<Vec<InternalSubset<'a>>>,
}

impl<'a> Parse<'a> for DocType<'a> {
    // [28] doctypedecl ::= '<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>'
    fn parse(input: &'a str) -> IResult<&'a str, DocType<'a>> {
        let (input, _) = tag("<!DOCTYPE")(input)?;
        let (input, _) = Self::parse_multispace1(input)?;
        let (input, name) = Self::parse_name(input)?;

        println!("\n\nx\ninput before parse externalid: {input:?}");
        let (input, external_id) =
            opt(preceded(Self::parse_multispace1, ExternalID::parse))(input)?;
        println!("input after parse externalid: {input:?}");
        println!("external_id: {:#?}", external_id);
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
                int_subset,
            },
        ))
    }
}
