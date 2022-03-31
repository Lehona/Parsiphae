use crate::inner_errors::ParserError;
use crate::parsers::exp::expression::parenthesis;
use crate::parsers::{call_parser, float_parser, number_parser, var_access};
use crate::types::{Expression, Input};

named!(pub Value<Input, Expression, ParserError>, fix_error!(ParserError, alt!(
        map!(call_parser, |call| Expression::Call(Box::new(call)))
       | map!(var_access, |va| Expression::Identifier(Box::new(va)))
       | map!(float_parser, Expression::Float)
       | map!(number_parser, Expression::Int)
       | parenthesis
)));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::utility::*;
    use crate::types::Call;
    use crate::types::Identifier;
    use crate::types::VarAccess;

    #[test]
    fn test_value_parser() {
        test_parser_done(Value, b"-1", Expression::Int(-1), b"");
        test_parser_done(Value, b"15", Expression::Int(15), b"");
        test_parser_done(
            Value,
            b"locals()",
            Expression::Call(Box::new(Call {
                func: Identifier::new(b"locals"),
                params: Vec::new(),
            })),
            b"",
        );

        test_parser_done(Value, b"1.5", Expression::Float(1.5), b"");
        test_parser_done(Value, b"(0)", Expression::Int(0), b"");
        test_parser_done(Value, b"((((((4))))))", Expression::Int(4), b"");
    }

    #[test]
    fn identifier_value() {
        let expected =
            Expression::Identifier(Box::new(VarAccess::new(Identifier::new(b"a"), None, None)));

        test_parser_done(Value, b"a", expected, b"");
    }
}
