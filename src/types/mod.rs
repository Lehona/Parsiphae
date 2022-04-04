mod base;
mod decl;
mod exp;
mod printable;
mod statement;
mod symbol;

pub use self::printable::{PrintableByteSlice, PrintableByteVec};

pub use self::base::{Identifier, StringLiteral};
pub use self::exp::{
    BinaryExpression, BinaryOperator, Call, Expression, FloatNode, IntNode, UnaryExpression,
    UnaryOperator, VarAccess,
};

pub use self::decl::{
    ArraySizeDeclaration, Class, ConstArrayDeclaration, ConstArrayInitializer, ConstDeclaration,
    Declaration, Function, Instance, Prototype, VarDeclaration,
};

pub use self::statement::{Assignment, AssignmentOperator, IfBranch, IfStatement, Statement, ReturnStatement};

pub use self::symbol::parsed;
pub use self::symbol::SymbolCollection;

#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    pub declarations: Vec<Declaration>,
}
