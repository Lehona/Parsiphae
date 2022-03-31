use crate::types::{Identifier, Statement, VarDeclaration};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: Identifier,
    pub typ: Identifier,
    pub params: Vec<VarDeclaration>,
    pub body: Vec<Statement>,
}
