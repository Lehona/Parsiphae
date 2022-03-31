use crate::types::{ArraySizeDeclaration, Expression, Identifier};

#[derive(Debug, Clone, PartialEq)]
pub struct ConstDeclaration {
    pub name: Identifier,
    pub typ: Identifier,
    initializer: Expression,
}

impl ConstDeclaration {
    pub fn new(typ: Identifier, name: Identifier, initializer: Expression) -> Self {
        ConstDeclaration {
            typ,
            name,
            initializer,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstArrayDeclaration {
    pub name: Identifier,
    pub typ: Identifier,
    pub array_size: ArraySizeDeclaration,
    initializer: ConstArrayInitializer,
}

impl ConstArrayDeclaration {
    pub fn new(
        typ: Identifier,
        name: Identifier,
        array_size: ArraySizeDeclaration,
        initializer: ConstArrayInitializer,
    ) -> Self {
        ConstArrayDeclaration {
            typ,
            name,
            array_size,
            initializer,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstArrayInitializer {
    expressions: Vec<Expression>,
}

impl ConstArrayInitializer {
    pub fn new(expressions: Vec<Expression>) -> Self {
        ConstArrayInitializer { expressions }
    }
}
