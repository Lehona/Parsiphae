use crate::inner_errors::ParserError;
use crate::types::Input;

pub fn flatten_vec<T>(vec: Vec<Vec<T>>) -> Vec<T> {
    vec.into_iter()
        .flat_map(|inner| inner.into_iter())
        .collect()
}

macro_rules! one_of_tag {
    ($first:expr $(, $op:expr)*) => {
        |input: Input| gws!(input, fix_error!(ParserError, alt!(
            tag!($first)
            $(
            | tag!($op)
            )*
        )))
    }
}

named!(line_comment<Input, Input>, recognize!(delimited!(
    tag!("//"),
    opt!(is_not!(b"\n")),
    alt!(tag!("\n")|eof!())
)));

named!(multi_line_comment<Input, Input>, recognize!(preceded!(
    tag!("/*"),
    many_till!(take!(1), tag!("*/"))
)));

named!(pub whitespace<Input, Input, ParserError>, fix_error!(ParserError, recognize!(alt!(
    line_comment
    |multi_line_comment
    |is_a!(b" \t\r\n\x0c")
))));

#[macro_export]
macro_rules! gws (
  ($i:expr, $($args:tt)*) => (
    {
      use nom::Convert;
      use nom::Err;
      use nom::lib::std::result::Result::*;
      use $crate::parsers::replacements::multispace0;
      use $crate::inner_errors::ParserError;

      match sep!($i, multispace0, $($args)*) {
        Err(e) => Err(Err::<_, ParserError>::convert(e)),
        Ok((i1,o))    => {
          match (multispace0)(i1) {
            Err(e) => Err(Err::convert(e)),
            Ok((i2,_))    => Ok((i2, o))
          }
        }
      }
    }
  )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::expression;
    use crate::types::{BinaryExpression, BinaryOperator, Expression};

    #[test]
    fn simple_whitespace() {
        let input = Input(b" \n \t\r");
        let expected = Input(b" \n \t\r");

        let actual = whitespace(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn no_whitespace() {
        let input = Input(b"");
        let expected: Vec<Input> = Vec::new();

        let actual = crate::parsers::replacements::multispace0(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn simple_line_comment() {
        let input = Input(b"//foo\r\n");
        let expected = Input(b"//foo\r\n");

        let actual = whitespace(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn trailing_line_comment() {
        let input = Input(b"//foo");
        let expected = Input(b"//foo");

        let actual = whitespace(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn trailing_empty_line_comment() {
        let input = Input(b"//");
        let expected = Input(b"//");

        let actual = whitespace(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn empty_line_comment() {
        let input = Input(b"//\n");
        let expected = Input(b"//\n");

        let actual = whitespace(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn empty_multi_line_comment() {
        let input = Input(b"/**/");
        let expected = Input(b"/**/");

        let actual = whitespace(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn simple_multi_line_comment() {
        let input = Input(b"/*foo*/");
        let expected = Input(b"/*foo*/");

        let actual = whitespace(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn embedded_whitespace() {
        let input = Input(b"4//\n+3");
        let expected = Expression::Binary(Box::new(BinaryExpression::new(
            BinaryOperator::Plus,
            Expression::Int(4),
            Expression::Int(3),
        )));

        let actual = expression(input).unwrap().1;

        assert_eq!(expected, actual);
    }
}
