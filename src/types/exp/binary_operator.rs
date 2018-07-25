use types::Expression;

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
