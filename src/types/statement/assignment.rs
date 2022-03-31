use crate::types::{Expression, VarAccess};

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

use crate::lexer::TokenKind;
impl std::convert::TryFrom<TokenKind> for AssignmentOperator {
    type Error = anyhow::Error;
    fn try_from(item: TokenKind) -> Result<Self, Self::Error> {
        match item {
            TokenKind::Assign => Ok(AssignmentOperator::Eq),
            TokenKind::PlusAssign => Ok(AssignmentOperator::PlusEq),
            TokenKind::MinusAssign => Ok(AssignmentOperator::MinusEq),
            TokenKind::DivideAssign => Ok(AssignmentOperator::DivideEq),
            TokenKind::MultiplyAssign => Ok(AssignmentOperator::MultiplyEq),
            _ => anyhow::bail!(
                "Trying to convert illegal token to AssignmentOperator: {:?}",
                item
            ),
        }
    }
}
