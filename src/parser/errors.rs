use crate::lexer::{Token, TokenKind};

pub type Result<O> = std::result::Result<O, ParsingError>;

#[derive(Debug)]
pub enum ParsePossibility {
    IfClause,
    Statement,
    Assignment,
    Expression,
    Declaration,
    VariableAccess,
    Integer,
    Decimal,
    Call,
    StringLiteral,
}
#[derive(Debug)]
pub enum ParsingErrorKind {
    MissingSemi,
    MissingIdentifier,
    VariableDeclaration,
    ClassDeclaration,
    IfClause,
    ElseClause,
    IllegalStatement,
    InvalidCall,
    IllegalExpression,
    Declaration,
    ExpectedToken(TokenKind),
    ExpectedOneOfToken(Vec<TokenKind>),
    ExpectedOneOf(Vec<ParsePossibility>),
    ReachedEOF,
    Failure,
    InternalParserFailure,
}

#[derive(Debug)]
pub struct ParsingError {
    pub kind: ParsingErrorKind,
    pub span: (usize, usize),
}

impl ParsingError {
    pub fn from_token(kind: ParsingErrorKind, token: &Token) -> Self {
        ParsingError {
            kind,
            span: token.span,
        }
    }

    pub fn internal_error() -> Self {
        ParsingError {
            kind: ParsingErrorKind::InternalParserFailure,
            span: (0, 0),
        }
    }
}
