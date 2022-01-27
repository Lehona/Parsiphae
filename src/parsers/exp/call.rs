use inner_errors::ParserError;
use nom::ErrorKind;
use parsers::{expression, identifier_parser, replacements::*};
use types::{Call, Expression, Input};

named!(pub call_parser<Input, Call, ParserError>, do_parse!(
    name: identifier_parser >> multispace0 >>
    char_e!('(') >> multispace0 >>
    params: return_error!(ErrorKind::Custom(ParserError::InvalidCall), call_parser_real) >>

    (Call { func: name, params })
));

named!(call_parser_real<Input, Vec<Expression>, ParserError>, fix_error!(ParserError, gws!(do_parse!(
    params: terminated!(
        separated_list!(
            gws!(char_e!(',')),
            expression),
        char_e!(')')) >>
    (params)
))));

#[cfg(test)]
mod tests {
    use super::*;
    use types::{Expression, Identifier, StringLiteral};

    #[test]
    fn no_param() {
        let input = Input(b"foo()");
        let expected = Call {
            func: Identifier::new(b"foo"),
            params: Vec::new(),
        };

        let actual = call_parser(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn single_int_param() {
        let input = Input(b"foo(3)");
        let expected = Call {
            func: Identifier::new(b"foo"),
            params: vec![Expression::Int(3)],
        };

        let actual = call_parser(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_mixed_params() {
        let input = Input(b"foo(3, \"hello\")");
        let expected = Call {
            func: Identifier::new(b"foo"),
            params: vec![
                Expression::Int(3),
                Expression::String(StringLiteral::new(b"hello")),
            ],
        };

        let actual = call_parser(input).unwrap().1;

        assert_eq!(expected, actual);
    }
}
