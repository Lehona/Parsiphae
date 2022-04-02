use crate::lexer::TokenKind;

pub type Result<O> = std::result::Result<O, ParsingError>;

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl ParsePossibility {
    pub fn reason(&self) -> &'static str {
        match self {
            ParsePossibility::IfClause => "If-Clause",
            ParsePossibility::Statement => "Statement",
            ParsePossibility::Assignment => "Assignment",
            ParsePossibility::Expression => "Expression",
            ParsePossibility::Declaration => "Declaration",
            ParsePossibility::VariableAccess => "Variable Access",
            ParsePossibility::Integer => "Integer",
            ParsePossibility::Decimal => "Decimal",
            ParsePossibility::Call => "Call",
            ParsePossibility::StringLiteral => "Stringliteral",
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum ParsingErrorKind {
    Expected(ParsePossibility),
    UnexpectedToken(TokenKind, TokenKind),  // Found, Expected
    ExpectedToken(TokenKind),
    ExpectedOneOfToken(Vec<TokenKind>),
    ExpectedOneOf(Vec<ParsePossibility>),
    ReachedEOF,
    InternalParserFailure,
    MissingFunctionName,
    MissingFunctionType,
    MissingInstanceName,
    MissingInstanceType,
    StatementWithoutSemicolon,
}

impl ParsingErrorKind {
    pub fn reason(&self) -> String {
        match &self {
            ParsingErrorKind::Expected(pp) => pp.reason().to_string(),
            ParsingErrorKind::ExpectedOneOf(vec) => vec.iter().map(|pp| pp.reason()).collect::<Vec<_>>().join(", "),
            ParsingErrorKind::ExpectedOneOfToken(vec) => vec.iter().map(|token| format!("{:?}", token)).collect::<Vec<_>>().join(", "),
            ParsingErrorKind::ExpectedToken(token) => format!("{:?}", token),
            ParsingErrorKind::ReachedEOF => "Reached EOF".to_string(),
            ParsingErrorKind::InternalParserFailure => "Internal Parser Failure".to_string(),
            ParsingErrorKind::MissingFunctionName => "This function is missing a name.".to_string(),
            ParsingErrorKind::UnexpectedToken(found, expected) => format!("Unexpected Token {:?}, expected {:?}", found, expected),
            _ => "missing reason".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParsingError {
    pub kind: ParsingErrorKind,
    pub token_start: usize,
    pub token_end: usize,
    pub recoverable: bool,
}

impl ParsingError {
    pub fn from_token(kind: ParsingErrorKind, token_id: usize, recoverable: bool) -> Self {
        ParsingError {
            kind,
            token_start: 0,
            token_end: token_id,
            recoverable,
        }
    }

    pub fn internal_error() -> Self {
        ParsingError {
            kind: ParsingErrorKind::InternalParserFailure,
            token_start: 0,
            token_end: 0,
            recoverable: false,
        }
    }
}
