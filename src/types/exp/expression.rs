use types::base::StringLiteral;
use types::{BinaryExpression, Call, UnaryExpression, VarAccess};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Int(i64),
    Float(f32),
    Identifier(Box<VarAccess>),
    Binary(Box<BinaryExpression>),
    Unary(Box<UnaryExpression>),
    Call(Box<Call>),
    String(StringLiteral),
}

impl Expression {
    pub fn is_float(&self) -> bool {
        match *self {
            Expression::Float(_) => true,
            _ => false,
        }
    }

    pub fn evaluate_int(&self) -> Result<i64, ()> {
        match *self {
            Expression::Int(i) => Ok(i),
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
