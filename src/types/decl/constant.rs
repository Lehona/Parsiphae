use crate::types::{ArraySizeDeclaration, Expression, Identifier};

#[derive(Debug, Clone, PartialEq)]
pub struct ConstDeclaration {
    pub name: Identifier,
    pub typ: Identifier,
    pub initializer: Expression,
    pub span: (usize, usize),
}

impl ConstDeclaration {
    pub fn new(typ: Identifier, name: Identifier, initializer: Expression, span: (usize, usize)) -> Self {
        ConstDeclaration {
            typ,
            name,
            initializer,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstArrayDeclaration {
    pub name: Identifier,
    pub typ: Identifier,
    pub array_size: ArraySizeDeclaration,
    pub initializer: ConstArrayInitializer,
    pub span: (usize, usize),
}

impl ConstArrayDeclaration {
    pub fn new(
        typ: Identifier,
        name: Identifier,
        array_size: ArraySizeDeclaration,
        initializer: ConstArrayInitializer,
        span: (usize, usize),
    ) -> Self {
        ConstArrayDeclaration {
            typ,
            name,
            array_size,
            initializer,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstArrayInitializer {
    expressions: Vec<Expression>,
    pub span: (usize, usize),
}

impl ConstArrayInitializer {
    pub fn new(expressions: Vec<Expression>, span: (usize, usize)) -> Self {
        ConstArrayInitializer { expressions, span }
    }
}
