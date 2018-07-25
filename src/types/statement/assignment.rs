use types::{Expression, VarAccess};

#[derive(Clone, Debug, PartialEq)]
pub enum AssignmentOperator {
    PlusEq,
    MinusEq,
    MultiplyEq,
    DivideEq,
    Eq,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub var: VarAccess,
    pub op: AssignmentOperator,
    pub exp: Expression,
}

impl AssignmentOperator {
    pub fn from(op: &[u8]) -> Self {
        match op {
            b"+=" => AssignmentOperator::PlusEq,
            b"-=" => AssignmentOperator::MinusEq,
            b"*=" => AssignmentOperator::MultiplyEq,
            b"/=" => AssignmentOperator::DivideEq,
            b"=" => AssignmentOperator::Eq,
            _ => panic!("Illegal assignment operator"),
        }
    }
}
