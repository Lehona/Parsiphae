use types::{
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
    ReturnStatement(Option<Expression>),
}
