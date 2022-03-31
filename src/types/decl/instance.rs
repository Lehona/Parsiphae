use crate::types::{Identifier, Statement, VarDeclaration};

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub name: Identifier,
    pub class: Identifier,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prototype {
    pub name: Identifier,
    pub class: Identifier,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: Identifier,
    pub members: Vec<VarDeclaration>,
}
