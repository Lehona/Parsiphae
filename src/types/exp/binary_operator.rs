use crate::lexer::TokenKind;
use crate::types::Expression;

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Mod,
    LSL,
    LSR,
    GT,
    LT,
    GE,
    LE,
    Eq,
    NotEq,
    And,
    BitAnd,
    Or,
    BitOr,
}

impl std::convert::TryFrom<TokenKind> for BinaryOperator {
    type Error = anyhow::Error;
    fn try_from(item: TokenKind) -> anyhow::Result<Self> {
        match item {
            TokenKind::Plus => Ok(BinaryOperator::Plus),
            TokenKind::Minus => Ok(BinaryOperator::Minus),
            TokenKind::Multiply => Ok(BinaryOperator::Multiply),
            TokenKind::Divide => Ok(BinaryOperator::Divide),
            TokenKind::ShiftLeft => Ok(BinaryOperator::LSL),
            TokenKind::ShiftRight => Ok(BinaryOperator::LSR),
            TokenKind::Greater => Ok(BinaryOperator::GT),
            TokenKind::Lower => Ok(BinaryOperator::LT),
            TokenKind::GreaterEquals => Ok(BinaryOperator::GE),
            TokenKind::LowerEquals => Ok(BinaryOperator::LE),
            TokenKind::Equals => Ok(BinaryOperator::Eq),
            TokenKind::NotEquals => Ok(BinaryOperator::NotEq),
            TokenKind::And => Ok(BinaryOperator::And),
            TokenKind::BitAnd => Ok(BinaryOperator::BitAnd),
            TokenKind::Or => Ok(BinaryOperator::Or),
            TokenKind::BitOr => Ok(BinaryOperator::BitOr),
            _ => anyhow::bail!(
                "Trying to convert illegal token to BinaryOperator: {:?}",
                item
            ),
        }
    }
}

impl BinaryOperator {
    pub fn sign(&self) -> &str {
        match *self {
            BinaryOperator::Plus => "+",
            BinaryOperator::Minus => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Mod => "%",
            BinaryOperator::LSL => "<<",
            BinaryOperator::LSR => ">>",
            BinaryOperator::GT => ">",
            BinaryOperator::LT => "<",
            BinaryOperator::GE => ">=",
            BinaryOperator::LE => "<=",
            BinaryOperator::Eq => "==",
            BinaryOperator::NotEq => "!=",
            BinaryOperator::And => "&&",
            BinaryOperator::BitAnd => "&",
            BinaryOperator::Or => "||",
            BinaryOperator::BitOr => "|",
        }
    }

    pub fn from(v: &[u8]) -> BinaryOperator {
        match v {
            br"+" => BinaryOperator::Plus,
            br"-" => BinaryOperator::Minus,
            br"*" => BinaryOperator::Multiply,
            br"/" => BinaryOperator::Divide,
            br"%" => BinaryOperator::Mod,
            br"<<" => BinaryOperator::LSL,
            br">>" => BinaryOperator::LSR,
            br">" => BinaryOperator::GT,
            br"<" => BinaryOperator::LT,
            br">=" => BinaryOperator::GE,
            br"<=" => BinaryOperator::LE,
            br"==" => BinaryOperator::Eq,
            br"!=" => BinaryOperator::NotEq,
            br"&&" => BinaryOperator::And,
            br"&" => BinaryOperator::BitAnd,
            br"||" => BinaryOperator::Or,
            br"|" => BinaryOperator::BitOr,
            _ => panic!("Illegal binary operator"),
        }
    }

    pub fn get_order(&self) -> usize {
        match *self {
            BinaryOperator::LSL => 4,
            BinaryOperator::LSR => 4,
            BinaryOperator::BitAnd => 4,
            BinaryOperator::BitOr => 4,
            BinaryOperator::Multiply => 3,
            BinaryOperator::Divide => 3,
            BinaryOperator::Mod => 3,
            BinaryOperator::Plus => 2,
            BinaryOperator::Minus => 2,
            BinaryOperator::GT => 1,
            BinaryOperator::LT => 1,
            BinaryOperator::GE => 1,
            BinaryOperator::LE => 1,
            BinaryOperator::Eq => 1,
            BinaryOperator::NotEq => 1,
            BinaryOperator::And => 0,
            BinaryOperator::Or => 0,
        }
    }

    pub fn apply(&self, left: i64, right: i64) -> i64 {
        match *self {
            BinaryOperator::LSL => left << right,
            BinaryOperator::LSR => left >> right,
            BinaryOperator::BitAnd => left & right,
            BinaryOperator::BitOr => left | right,
            BinaryOperator::Multiply => left * right,
            BinaryOperator::Divide => left / right,
            BinaryOperator::Mod => left % right,
            BinaryOperator::Plus => left + right,
            BinaryOperator::Minus => left - right,
            BinaryOperator::GT => (left > right) as i64,
            BinaryOperator::LT => (left < right) as i64,
            BinaryOperator::GE => (left >= right) as i64,
            BinaryOperator::LE => (left <= right) as i64,
            BinaryOperator::Eq => (left == right) as i64,
            BinaryOperator::NotEq => (left != right) as i64,
            BinaryOperator::And => (left != 0 && right != 0) as i64,
            BinaryOperator::Or => (left != 0 || right != 0) as i64,
        }
    }

    pub fn needs_parentheses(&self, child: &Expression) -> bool {
        match *child {
            Expression::Float(_) => false,
            Expression::Int(_) => false,
            Expression::Identifier(_) => false,
            Expression::Call(_) => false,
            Expression::Unary(_) => false,
            Expression::String(_) => false,
            Expression::Binary(ref bin) => self.get_order() > bin.as_ref().op.get_order(),
        }
    }
}
