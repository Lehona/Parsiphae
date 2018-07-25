use inner_errors::ParserError;
use parsers::{expression, identifier_parser};
use types::{Input, VarAccess};

named!(pub var_access<Input, VarAccess, ParserError>, fix_error!(ParserError, gws!(do_parse!(
    first: identifier_parser >>
    second: opt!(preceded!(
        char_e!('.'),
        identifier_parser)) >>
    index: opt!(delimited!(
        char_e!('['),
        delimited!(multispace0, expression, multispace0),
        char_e!(']'))) >>

    (VarAccess::new(first, second, index))
))));

#[cfg(test)]
mod tests {
    use super::*;
    use types::{Expression, Identifier};

    #[test]
    fn simple() {
        let input = Input(b"foo");
        let expected = VarAccess::new(Identifier::new(b"foo"), None, None);

        let actual = var_access(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn instance() {
        let input = Input(b"foo.bar");
        let expected = VarAccess::new(Identifier::new(b"foo"), Some(Identifier::new(b"bar")), None);

        let actual = var_access(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn simple_array_int() {
        let input = Input(b"foo[3]");
        let expected = VarAccess::new(Identifier::new(b"foo"), None, Some(Expression::Int(3)));

        let actual = var_access(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn instance_array_int() {
        let input = Input(b"foo.bar[3]");
        let expected = VarAccess::new(
            Identifier::new(b"foo"),
            Some(Identifier::new(b"bar")),
            Some(Expression::Int(3)),
        );

        let actual = var_access(input).unwrap().1;

        assert_eq!(expected, actual);
    }
}
