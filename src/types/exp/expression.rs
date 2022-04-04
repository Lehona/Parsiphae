use crate::types::base::StringLiteral;
use crate::types::{BinaryExpression, Call, UnaryExpression, VarAccess};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Int(IntNode),
    Float(FloatNode),
    Identifier(Box<VarAccess>),
    Binary(Box<BinaryExpression>),
    Unary(Box<UnaryExpression>),
    Call(Box<Call>),
    String(StringLiteral),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntNode {
    pub value: i64,
    pub span: (usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatNode {
    pub value: f64,
    pub span: (usize, usize),
}

impl Expression {
    pub fn is_float(&self) -> bool {
        match *self {
            Expression::Float(_) => true,
            _ => false,
        }
    }

    pub fn get_span(&self) -> (usize, usize) {
        match self {
            Expression::Int(i) => i.span,
            Expression::Float(f) => f.span,
            Expression::Identifier(i) => i.span,
            Expression::Binary(b) => b.span,
            Expression::Unary(u) => u.span,
            Expression::Call(c) => c.span,
            Expression::String(s) => s.span,
        }
    }

    pub fn evaluate_int(&self) -> Result<i64, ()> {
        match *self {
            Expression::Int(i) => Ok(i.value),
            Expression::Binary(ref b) => (*b).evaluate(),
            Expression::Unary(ref b) => (*b).evaluate(),
            _ => Err(()),
        }
    }
    /*
        pub fn is_constant_int(&self) -> bool {
            match *self {
                Expression::Value(_) => true,
                Expression::Float(_) => false,
                Expression::Identifier(_) => true,
                Expression::Binary(_) => true,
                Expression::Unary(_) => true,
                Expression::Call(_) => false,
                Expression::String(_) => false
            }
        }
    */

    pub fn is_constant(&self) -> bool {
        match *self {
            Expression::Int(_) => true,
            Expression::Float(_) => true,
            Expression::Identifier(ref var) => var.is_constant(),
            Expression::Binary(ref bin) => bin.is_constant(),
            Expression::Unary(ref un) => un.is_constant(),
            Expression::Call(_) => false,
            Expression::String(_) => true,
        }
    }

    /* pub fn fold(&self, table: &zSymbol_Table) -> ConstantFoldedValue {
        match *self {
            Expression::Value(i) => ConstantFoldedValue::Int(i),
            Expression::Binary(ref bin) => bin.fold(table),
            Expression::Unary(ref un) => un.fold(table),
            Expression::String(ref s) => ConstantFoldedValue::String(s.to_string()),
            Expression::Identifier(ref var) => var.fold(table),
            _ => panic!("Trying to fold non-const value"),
        }
    }*/
}
