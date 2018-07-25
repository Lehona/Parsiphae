use types::{Expression, Identifier};

#[derive(Clone, PartialEq, Debug)]
pub struct Call {
    pub func: Identifier,
    pub params: Vec<Expression>,
}
