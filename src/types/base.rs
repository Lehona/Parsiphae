use crate::types::PrintableByteVec;
use std::fmt::Debug;

use super::parsed::zPAR_TYPE;

#[derive(Clone, PartialEq)]
pub struct StringLiteral {
    pub data: PrintableByteVec,
    pub span: (usize, usize),
}

impl StringLiteral {
    pub fn new(data: &[u8], span: (usize, usize)) -> Self {
        StringLiteral {
            data: PrintableByteVec(data.to_vec()),
            span,
        }
    }
}

impl ::std::fmt::Debug for StringLiteral {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _ = self.data.fmt(f);
        write!(f, "{:?}", self.span)
    }
}
#[derive(Clone)]
pub struct Identifier {
    pub name: PrintableByteVec,
    pub span: (usize, usize),
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Identifier {}

impl Identifier {
    pub fn new(name: &[u8], span: (usize, usize)) -> Self {
        Identifier {
            name: PrintableByteVec(name.to_vec()),
            span,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.name.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.name.0.clone()
    }
}

// impl std::convert::TryFrom<crate::lexer::TokenKind> for Identifier {
//     type Error = &'static str;
//     fn try_from(token: crate::lexer::TokenKind) -> Result<Self, Self::Error> {
//         match token {
//             crate::lexer::TokenKind::Identifier(name) => Ok(Identifier {
//                 name: PrintableByteVec(name),
//             }),
//             _ => Err("Trying to convert non-identifier token to Identifier."),
//         }
//     }
// }

impl ::std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        self.name.fmt(f)
    }
}

impl ::std::fmt::Debug for Identifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let _ = self.name.fmt(f);
        write!(f, "{:?}", self.span)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct External {
    pub name: PrintableByteVec,
    pub parameters: Vec<zPAR_TYPE>,
    pub return_type: zPAR_TYPE,
    pub address: u32,
}
