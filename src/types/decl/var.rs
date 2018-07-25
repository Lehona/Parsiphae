use types::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub struct VarDeclaration {
    typ: Identifier,
    name: Identifier,
    array_size: Option<ArraySizeDeclaration>,
}

impl VarDeclaration {
    pub fn new(
        typ: Identifier,
        name: Identifier,
        array_size: Option<ArraySizeDeclaration>,
    ) -> Self {
        VarDeclaration {
            typ,
            name,
            array_size,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ArraySizeDeclaration {
    Identifier(Identifier),
    Size(i64),
}
