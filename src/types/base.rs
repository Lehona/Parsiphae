use crate::types::PrintableByteVec;
use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub struct StringLiteral {
    pub data: PrintableByteVec,
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

impl std::convert::TryFrom<crate::lexer::TokenKind> for Identifier {
    type Error = &'static str;
    fn try_from(token: crate::lexer::TokenKind) -> Result<Self, Self::Error> {
        match token {
            crate::lexer::TokenKind::Identifier(name) => Ok(Identifier {
                name: PrintableByteVec(name),
            }),
            _ => Err("Trying to convert non-identifier token to Identifier."),
        }
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
