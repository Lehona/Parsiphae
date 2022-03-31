use crate::inner_errors::ParserError;
use crate::types::{Expression, Input};
use nom::Context::Code;
use nom::{Err, ErrorKind, IResult};
use std::fmt::Debug;

pub fn incomplete_result<'a, O, E>() -> IResult<Input<'a>, O, E> {
    let err = Err::Error(Code(Input(b""), ErrorKind::Eof));

    Result::Err(err)
}

pub fn failure_result<'a, O, E>(
    input: &'a [u8],
    error_kind: ErrorKind<E>,
) -> IResult<Input<'a>, O, E> {
    let err = Err::Error(Code(Input(input), error_kind));

    Result::Err(err)
}

pub fn test_parser_done<'a, O: Debug + PartialEq>(
    parser: fn(Input<'a>) -> IResult<Input<'a>, O, ParserError>,
    input: &'a [u8],
    expected: O,
    leftover: &'a [u8],
) {
    let result = parser(Input(input));

    assert_eq!(result, Ok((Input(leftover), expected)));
}

pub fn test_parser_error<'a, O: Debug + PartialEq>(
    parser: fn(Input<'a>) -> IResult<Input<'a>, O, ParserError>,
    input: &'a [u8],
    expected: IResult<Input<'a>, O, ParserError>,
) where
    IResult<Input<'a>, O>: PartialEq,
{
    let result = parser(Input(input));

    assert_eq!(result, expected);
}

pub fn test_expression_value<'a>(
    parser: fn(Input<'a>) -> IResult<Input<'a>, Expression, ParserError>,
    input: &'a [u8],
    expected: i64,
) {
    let expression = parser(Input(input));

    assert!(expression.is_ok());

    let (leftover, expression) = expression.unwrap();
    assert_eq!(leftover, Input(b""));
    let result = expression.evaluate_int();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}
