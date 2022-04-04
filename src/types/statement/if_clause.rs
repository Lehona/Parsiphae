use crate::types::{Expression, Statement};

#[derive(Debug, PartialEq, Clone)]
pub struct IfBranch {
    pub cond: Expression,
    pub body: Vec<Statement>,
    pub span: (usize, usize),
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub branches: Vec<IfBranch>,
    pub else_branch: Option<Vec<Statement>>,
    pub span: (usize, usize),
}
