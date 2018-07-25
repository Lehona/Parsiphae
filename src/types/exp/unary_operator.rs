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
            UnaryOperator::Negate => if val == 0 {
                1
            } else {
                0
            },
        }
    }
}
