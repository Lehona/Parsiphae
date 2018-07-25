use inner_errors::ParserError;
use nom::types::CompleteByteSlice;
use nom::IResult;
use types::PrintableByteVec;

pub type Input<'a> = CompleteByteSlice<'a>;
#[allow(non_snake_case)]
pub fn Input<'a>(input: &'a [u8]) -> Input<'a> {
    CompleteByteSlice(input)
}

pub type PResult<'a, O> = IResult<Input<'a>, O, ParserError>;

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    data: PrintableByteVec,
}

impl StringLiteral {
    pub fn new(data: &[u8]) -> Self {
        StringLiteral {
            data: PrintableByteVec(data.to_vec()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    name: PrintableByteVec,
}

impl Identifier {
    pub fn new(name: &[u8]) -> Self {
        Identifier {
            name: PrintableByteVec(name.to_vec()),
        }
    }
}

impl ::std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", ::std::str::from_utf8(&self.name).unwrap())
    }
}
