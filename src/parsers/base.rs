use crate::inner_errors::ParserError;
use crate::types::*;

fn is_not_quote(input: u8) -> bool {
    input != b'\"'
}

fn convert_string_literal(input: Input) -> StringLiteral {
    StringLiteral::new(&input.0)
}

named!(pub string_parser<Input, StringLiteral, ParserError>, fix_error!(ParserError, map!(
    delimited!(
        tag!("\""),
        take_while!(is_not_quote),
        tag!("\"")
    ),
    convert_string_literal
)));

named!(pub number_parser<Input, i64, ParserError>, fix_error!(ParserError, flat_map!(
    recognize!(
        tuple!(
            opt!(char!('-')),
            is_a!("0123456789")
    )),
    parse_to!(i64)
)));

named!(pub float_parser<Input, f32, ParserError>, fix_error!(ParserError, flat_map!(
    recognize!(
        tuple!(
            opt!(tag!("-")),
            is_a!("0123456789"),
            char!('.'),
            is_a!("0123456789")
         )
    ),
    parse_to!(f32)
)));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::utility::*;
    use nom::ErrorKind;

    #[test]
    pub fn test_string_parser() {
        test_parser_done(
            string_parser,
            b"\"hello world\"",
            StringLiteral::new(b"hello world"),
            b"",
        );

        test_parser_done(string_parser, b"\"\"", StringLiteral::new(b""), b"");

        test_parser_done(
            string_parser,
            b"\"hello\"world",
            StringLiteral::new(b"hello"),
            b"world",
        );

        test_parser_error(string_parser, b"\"hello", incomplete_result());
        test_parser_error(string_parser, b"", incomplete_result());
    }

    #[test]
    pub fn test_number_parser() {
        test_parser_done(number_parser, b"-1", -1, b"");
        test_parser_done(number_parser, b"15", 15, b"");

        test_parser_error(
            number_parser,
            b"xxx",
            failure_result(b"xxx", ErrorKind::IsA),
        );
    }

    #[test]
    pub fn test_float_parser() {
        test_parser_done(float_parser, b"-1.0", -1.0, b"");
        test_parser_done(float_parser, b"15.775", 15.775, b"");

        test_parser_error(float_parser, b"xxx", failure_result(b"xxx", ErrorKind::IsA));
    }
}
