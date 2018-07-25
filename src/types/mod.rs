mod base;
mod decl;
mod exp;
mod printable;
mod statement;

pub use self::printable::{PrintableByteSlice, PrintableByteVec};

pub use self::base::{Identifier, Input, PResult, StringLiteral};
pub use self::exp::{
    BinaryExpression, BinaryOperator, Call, Expression, UnaryExpression, UnaryOperator, VarAccess,
};

pub use self::decl::{
    ArraySizeDeclaration, Class, ConstArrayDeclaration, ConstArrayInitializer, ConstDeclaration,
    Declaration, Function, Instance, Prototype, VarDeclaration,
};

pub use self::statement::{Assignment, AssignmentOperator, IfBranch, IfStatement, Statement};

#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    pub declarations: Vec<Declaration>,
}
