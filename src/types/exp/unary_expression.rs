use types::{Expression, UnaryOperator};

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    op: UnaryOperator,
    right: Expression,
}

impl UnaryExpression {
    pub fn new(op: u8, right: Expression) -> Self {
        UnaryExpression {
            op: UnaryOperator::from_ascii(op),
            right,
        }
    }

    pub fn evaluate(&self) -> Result<i64, ()> {
        let right = self.right.evaluate_int()?;
        Ok(self.op.apply(right))
    }

    pub fn is_constant(&self) -> bool {
        self.right.is_constant()
    }
}
