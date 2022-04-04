use crate::types::{
    Assignment, ConstArrayDeclaration, ConstDeclaration, Expression, IfStatement, VarDeclaration,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Exp(Expression),
    Ass(Assignment),
    If(Box<IfStatement>),
    VarDeclarations(Vec<VarDeclaration>),
    ConstDeclaration(ConstDeclaration),
    ConstArrayDeclaration(ConstArrayDeclaration),
    ReturnStatement(ReturnStatement),
}

impl Statement {
    pub fn get_span(&self) -> (usize, usize) {
        match self {
            Statement::Exp(e) => e.get_span(),
            Statement::Ass(a) => a.span,
            Statement::If(i) => i.span,
            Statement::VarDeclarations(v) => (0, 0), // TODO implement
            Statement::ConstDeclaration(c) => c.span,
            Statement::ConstArrayDeclaration(c) => c.span,
            Statement::ReturnStatement(r) => r.span,

        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub exp: Option<Expression>,
    pub span: (usize, usize),
}