use types::{BinaryOperator, Expression};

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpression {
    pub op: BinaryOperator,
    pub left: Expression,
    pub right: Expression,
}

impl BinaryExpression {
    pub fn new(op: BinaryOperator, left: Expression, right: Expression) -> Self {
        BinaryExpression { op, left, right }
    }

    pub fn is_constant(&self) -> bool {
        self.left.is_constant() && self.right.is_constant()
    }

    pub fn evaluate(&self) -> Result<i64, ()> {
        let left = self.left.evaluate_int()?;
        let right = self.right.evaluate_int()?;
        Ok(self.op.apply(left, right))
    }
}
