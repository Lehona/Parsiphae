use inner_errors::ParserError;
use parsers::util::whitespace;
use types::{Input, PResult};

named!(pub multispace0<Input, Vec<Input>, ParserError>, fix_error!(ParserError, many0!(whitespace)));
named!(pub multispace1<Input, Vec<Input>, ParserError>, fix_error!(ParserError, many1!(whitespace)));

macro_rules! tag_no_case_e {
  ($i:expr) => ( $crate::parsers::replacements::tag_no_case( $i ) );
  ($i:expr, $($args:expr),* ) => ( $crate::parsers::replacements::tag_no_case( $i, $($args),* ) );
}

pub fn tag_no_case<'a>(input: Input<'a>, tag: &str) -> PResult<'a, Input<'a>> {
    let result = fix_error!(input, ParserError, tag_no_case!(tag));

    result
}

macro_rules! tag_e {
  ($i:expr) => ( $crate::parsers::replacements::tag( $i ) );
  ($i:expr, $($args:expr),* ) => ( $crate::parsers::replacements::tag( $i, $($args),* ) );
}

pub fn tag<'a>(input: Input<'a>, tag: &str) -> PResult<'a, Input<'a>> {
    let result = fix_error!(input, ParserError, tag!(tag));

    result
}

macro_rules! one_of_e {
  ($i:expr) => ( $crate::parsers::replacements::one_of( $i ) );
  ($i:expr, $($args:expr),* ) => ( $crate::parsers::replacements::one_of( $i, $($args),* ) );
}

pub fn one_of<'a>(input: Input<'a>, ofs: &[u8]) -> PResult<'a, char> {
    let result = fix_error!(input, ParserError, one_of!(ofs));

    result
}

macro_rules! char_e {
  ($i:expr) => ( $crate::parsers::replacements::char_e( $i ) );
  ($i:expr, $($args:expr),* ) => ( $crate::parsers::replacements::char_e( $i, $($args),* ) );
}

pub fn char_e<'a>(input: Input<'a>, tag: char) -> PResult<'a, char> {
    let result = fix_error!(input, ParserError, char!(tag));

    result
}
