use inner_errors::ParserError;
use parsers::Unary;
use types::PResult;
use types::{BinaryExpression, BinaryOperator, Expression, Input};

pub fn Boolean(input: Input) -> PResult<Expression> {
    left_associative_binary(one_of_tag!("||", "&&"), Cmp)(input)
}

pub fn Cmp(input: Input) -> PResult<Expression> {
    left_associative_binary(one_of_tag!(">=", "<=", "!=", "==", ">", "<"), Add)(input)
}

pub fn Add(input: Input) -> PResult<Expression> {
    left_associative_binary(one_of_tag!("+", "-"), Mul)(input)
}

pub fn Mul(input: Input) -> PResult<Expression> {
    left_associative_binary(one_of_tag!("*", "/", "%"), Bit)(input)
}

pub fn Bit(input: Input) -> PResult<Expression> {
    left_associative_binary(one_of_tag!("|", "&", ">>", "<<"), Unary)(input)
}

pub fn left_associative_binary<'a>(
    ops: fn(Input<'a>) -> PResult<Input<'a>>,
    next_level: fn(Input<'a>) -> PResult<Expression>,
) -> impl Fn(Input<'a>) -> PResult<Expression> {
    let parser = move |input: Input<'a>| {
        fix_error!(
            input,
            ParserError,
            do_parse!(
                first: next_level
                    >> folded:
                        fold_many0!(
                            gws!(tuple!(ops, next_level)),
                            first,
                            |acc, (op, exp): (Input, Expression)| {
                                Expression::Binary(Box::new(BinaryExpression::new(
                                    BinaryOperator::from(&op.0),
                                    acc,
                                    exp,
                                )))
                            }
                        ) >> (folded)
            )
        )
    };

    parser
}

#[cfg(test)]
mod tests {
    use super::*;
    use tests::utility::*;
    use types::UnaryExpression;

    #[test]
    fn two_ops() {
        let input = Input(b"2 | 4");
        let expected = 6;

        let actual = Boolean(input).unwrap().1.evaluate_int().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn left_assoc() {
        let input = Input(b"5-2-1");
        let expected = 2;

        let actual = Boolean(input).unwrap().1.evaluate_int().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn suite() {
        let pairs = vec![
            (Input(b"4+8/2*9"), 40),
            (Input(b"7+4/4*2"), 9),
            (Input(b"3+11*4+12/3"), 51),
            (Input(b"12-12+8*9-8"), 64),
            (Input(b"13+15-12*5/4-13"), 0),
            (Input(b"6/3-3+12+10*3"), 41),
            (Input(b"3*5&1"), 3),
            (Input(b"1||2"), 1),
            (Input(b"1||0"), 1),
            (Input(b"1&&1"), 1),
            (Input(b"3<<1+7"), 13),
            (Input(b"1||0&&1||0"), 1),
            (Input(b"1&&1||1&&0"), 0),
            (Input(b"7*-3"), -21),
            (Input(b"7*-3+5"), -16),
            (Input(b"7*-(3+5)"), -56),
        ];

        for (equation, expected) in pairs {
            let actual = Boolean(equation).unwrap().1.evaluate_int().unwrap();
            assert_eq!(expected, actual);
        }
    }

    fn make_binary_exp_int(op: &[u8], left: i64, right: i64) -> Expression {
        Expression::Binary(Box::new(BinaryExpression::new(
            BinaryOperator::from(op),
            Expression::Int(left),
            Expression::Int(right),
        )))
    }

    fn make_binary_exp(op: &[u8], left: Expression, right: Expression) -> Expression {
        Expression::Binary(Box::new(BinaryExpression::new(
            BinaryOperator::from(op),
            left,
            right,
        )))
    }

    #[test]
    fn simple_tree() {
        let exp = make_binary_exp_int(b"|", 1, 2);
        test_parser_done(Bit, b"1|2", exp, b"");
    }

    #[test]
    fn simple_tree_double_char_op() {
        let exp = make_binary_exp_int(b">>", 1, 2);
        test_parser_done(Bit, b"1>>2", exp, b"");
    }

    #[test]
    fn three_node_tree() {
        let inner = make_binary_exp_int(b"|", 1, 1);
        let outer = make_binary_exp(b"&", inner, Expression::Int(2));
        test_parser_done(Bit, b"1|1&2", outer, b"");
    }

    #[test]
    fn tree_with_unary() {
        let inner = Expression::Unary(Box::new(UnaryExpression::new(b'!', Expression::Int(2))));
        let exp = make_binary_exp(b">>", Expression::Int(1), inner);
        test_parser_done(Bit, b"1>>!2", exp, b"");
    }
}
