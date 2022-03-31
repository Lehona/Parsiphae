use crate::inner_errors::ParserError;
use crate::parsers::Value;
use crate::types::{Expression, Input, UnaryExpression};

named!(pub Unary<Input, Expression, ParserError>, fix_error!(ParserError, alt!(
    gws!(do_parse!(
        op: one_of_e!(b"!~-+") >>
        exp: Unary >>
        (Expression::Unary(Box::new(UnaryExpression::new(op as u8, exp))))
    ))

    | Value
)));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::utility::*;

    #[test]
    fn single() {
        let input = Input(b"!5");
        let expected = 0;

        let actual = Unary(input).unwrap().1.evaluate_int().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_unary_parser() {
        let exp = Expression::Unary(Box::new(UnaryExpression::new(b'-', Expression::Int(1))));
        test_parser_done(Unary, b"-1", exp, b"");

        let exp = Expression::Unary(Box::new(UnaryExpression::new(b'!', Expression::Int(7))));
        test_parser_done(Unary, b"!7", exp, b"");

        let exp = Expression::Unary(Box::new(UnaryExpression::new(b'~', Expression::Int(123))));
        test_parser_done(Unary, b"~ 123", exp, b"");

        let exp_inner =
            Expression::Unary(Box::new(UnaryExpression::new(b'~', Expression::Int(123))));
        let exp = Expression::Unary(Box::new(UnaryExpression::new(b'!', exp_inner)));
        test_parser_done(Unary, b"! ~ 123", exp, b"");

        test_expression_value(Unary, b"!1", 0);
        test_expression_value(Unary, b"!!!!!!!!15", 1);
        test_expression_value(Unary, b"~2147483647", -2147483648);
        test_expression_value(Unary, b"--34541", 34541);
        test_expression_value(Unary, b"-34541", -34541);
        test_expression_value(Unary, b"-0", 0);
        test_expression_value(Unary, b"+154", 154);
    }
}
