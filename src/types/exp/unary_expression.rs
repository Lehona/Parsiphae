use crate::lexer;
use crate::types::{Expression, UnaryOperator};
use anyhow::Result;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    pub op: UnaryOperator,
    pub right: Expression,
}

impl UnaryExpression {
    pub fn new(op: u8, right: Expression) -> Self {
        UnaryExpression {
            op: UnaryOperator::from_ascii(op),
            right,
        }
    }

    pub fn new_token(op: lexer::TokenKind, right: Expression) -> Result<Self> {
        Ok(UnaryExpression {
            op: op.try_into()?,
            right,
        })
    }

    pub fn evaluate(&self) -> Result<i64, ()> {
        let right = self.right.evaluate_int()?;
        Ok(self.op.apply(right))
    }

    pub fn is_constant(&self) -> bool {
        self.right.is_constant()
    }
}
