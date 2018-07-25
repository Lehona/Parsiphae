use inner_errors::ParserError;
use parsers::{string_parser, Boolean};
use types::{Expression, Input};

named!(pub expression<Input, Expression, ParserError>, fix_error!(ParserError,
add_return_error!(ErrorKind::Custom(ParserError::IllegalExpression),
    alt!(
          map!(string_parser, Expression::String)
        | Boolean
    ))
));

named!(pub parenthesis<Input, Expression, ParserError>, fix_error!(ParserError, gws!(delimited!(
    char_e!('('),
    expression,
    char_e!(')')
))));
