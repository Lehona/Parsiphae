#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ParserError {
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
    FromNom,
}

impl ParserError {
    pub fn description(&self) -> &'static str {
        use self::ParserError::*;

        match *self {
            MissingSemi => "Missing semicolon",
            MissingIdentifier => "Missing identifier",
            VariableDeclaration => "Error during variable declaration",
            ClassDeclaration => "Error during class declaration",
            IfClause => "Error in if-block",
            ElseClause => "Error in else-block",
            IllegalStatement => "Error in statement",
            InvalidCall => "Error in function call",
            IllegalExpression => "Error in expression",
            Declaration => "Error in declaration",
            FromNom => "You should never see this",
        }
    }
}

impl ::std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl ::std::convert::From<u32> for ParserError {
    fn from(_f: u32) -> Self {
        ParserError::FromNom
    }
}
