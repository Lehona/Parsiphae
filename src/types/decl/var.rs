use crate::types::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub struct VarDeclaration {
    pub typ: Identifier,
    pub name: Identifier,
    pub array_size: Option<ArraySizeDeclaration>,
    pub span: (usize, usize),
}

impl VarDeclaration {
    pub fn new(
        typ: Identifier,
        name: Identifier,
        array_size: Option<ArraySizeDeclaration>,
        span: (usize, usize),
    ) -> Self {
        VarDeclaration {
            typ,
            name,
            array_size,
            span,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ArraySizeDeclaration {
    Identifier(Identifier),
    Size(i64),
}
