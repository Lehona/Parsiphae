use crate::lexer;
use crate::parser::errors::{ParsingError, Result as PResult};
use crate::types::{Expression, UnaryOperator};
use anyhow::Result;
use std::convert::TryInto;
use std::net::UdpSocket;

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    pub op: UnaryOperator,
    pub right: Expression,
    pub span: (usize, usize),
}

impl UnaryExpression {
    pub fn new(op: u8, right: Expression, span: (usize, usize)) -> Self {
        UnaryExpression {
            op: UnaryOperator::from_ascii(op),
            right,
            span,
        }
    }

    pub fn from_token(
        op: lexer::TokenKind,
        right: Expression,
        span: (usize, usize),
    ) -> PResult<Self> {
        Ok(UnaryExpression {
            op: op.try_into().map_err(|_| ParsingError::internal_error())?,
            right,
            span,
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
