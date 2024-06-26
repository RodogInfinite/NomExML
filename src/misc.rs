// misc.rs
use crate::{
    parse::Parse, processing_instruction::ProcessingInstruction, Document, Error, IResult,
};
use nom::{branch::alt, combinator::map};

#[derive(Clone, PartialEq, Eq)]
pub enum MiscState {
    BeforeDoctype,
    AfterDoctype,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Misc {
    pub content: Box<Document>, // Document::Comment | Document::ProcessingInstruction>
    pub state: MiscState,
}

impl<'a> Parse<'a> for Misc {
    type Args = MiscState;
    type Output = IResult<&'a str, Self>;

    //[27] Misc ::= Comment | PI | S
    fn parse(input: &'a str, args: Self::Args) -> Self::Output {
        let mut input_remaining = input;
        let mut content_vec: Vec<Document> = vec![];
        loop {
            let parse_result = alt((
                Document::parse_comment,
                map(
                    |i| ProcessingInstruction::parse(i, ()),
                    Document::ProcessingInstruction,
                ),
                map(Self::parse_multispace1, |_| Document::Empty),
            ))(input_remaining);

            match parse_result {
                Ok((remaining, document)) => {
                    match document {
                        Document::Empty => {} // Don't add Document::Empty types to content_vec
                        _ => content_vec.push(document),
                    }
                    input_remaining = remaining;
                }

                Err(_) => {
                    if !content_vec.is_empty() {
                        break;
                    } else {
                        return Err(nom::Err::Error(Error::NomError(nom::error::Error::new(
                            input.to_string(),
                            nom::error::ErrorKind::Many0,
                        ))));
                    }
                }
            }
        }

        let content = Box::new(Document::Nested(content_vec));
        Ok((
            input_remaining,
            Misc {
                content,
                state: args,
            },
        ))
    }
}
