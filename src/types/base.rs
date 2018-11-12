use inner_errors::ParserError;
use nom::types::CompleteByteSlice;
use nom::IResult;
use std::fmt::Debug;
use types::PrintableByteVec;

pub type Input<'a> = CompleteByteSlice<'a>;
#[allow(non_snake_case)]
pub fn Input<'a>(input: &'a [u8]) -> Input<'a> {
    CompleteByteSlice(input)
}

pub type PResult<'a, O> = IResult<Input<'a>, O, ParserError>;

#[derive(Clone, PartialEq)]
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

impl ::std::fmt::Debug for StringLiteral {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.data.fmt(f)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct Identifier {
    name: PrintableByteVec,
}

impl Identifier {
    pub fn new(name: &[u8]) -> Self {
        Identifier {
            name: PrintableByteVec(name.to_vec()),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.name.0
    }
}

impl ::std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        self.name.fmt(f)
    }
}

impl ::std::fmt::Debug for Identifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        self.name.fmt(f)
    }
}
