use crate::lexer::TokenKind;

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Flip,
    Negate,
}

impl UnaryOperator {
    pub fn from_ascii(op: u8) -> Self {
        match op {
            b'+' => UnaryOperator::Plus,
            b'-' => UnaryOperator::Minus,
            b'!' => UnaryOperator::Negate,
            b'~' => UnaryOperator::Flip,
            _ => panic!("Constructing UnaryOperator from illegal byte"),
        }
    }

    pub fn apply(&self, val: i64) -> i64 {
        match *self {
            UnaryOperator::Plus => val,
            UnaryOperator::Minus => -val,
            UnaryOperator::Flip => !val,
            UnaryOperator::Negate => {
                if val == 0 {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl std::convert::TryFrom<TokenKind> for UnaryOperator {
    type Error = anyhow::Error;
    fn try_from(item: TokenKind) -> Result<Self, Self::Error> {
        match item {
            TokenKind::Plus => Ok(UnaryOperator::Plus),
            TokenKind::Minus => Ok(UnaryOperator::Minus),
            TokenKind::BitNot => Ok(UnaryOperator::Flip),
            TokenKind::Not => Ok(UnaryOperator::Negate),
            _ => anyhow::bail!(
                "Trying to convert illegal token to UnaryOperator: {:?}",
                item
            ),
        }
    }
}
