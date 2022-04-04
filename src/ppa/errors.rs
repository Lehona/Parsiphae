use crate::types::Identifier;

pub type Result<O> = std::result::Result<O, TypecheckError>;

pub enum TypecheckErrorKind {
    UnknownIdentifier(Vec<u8>),
    MixingFloatAndString(usize, usize), // Secondary Span
    MixingFLoatAndInt(usize, usize),
    MixingIntAndString(usize, usize),
    VoidFunctionInExpression,
    UnknownReturnType(Identifier),
}

pub struct TypecheckError {
    pub kind: TypecheckErrorKind,
    pub span: (usize, usize),
}
