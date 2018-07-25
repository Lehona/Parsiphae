use inner_errors::ParserError;
use types::{Identifier, Input};

fn convert_identifier(input: Input) -> Identifier {
    Identifier::new(&input.0)
}

const IDENTIFIER_BEGIN: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_1234567890";
const IDENTIFIER_END: &'static [u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_^@1234567890\xC4\xE4\xD6\xF6\xFC\xDC\xDF";

named!(pub identifier_list<Input, Vec<Identifier>, ParserError>,
    separated_nonempty_list!(
        gws!(char_e!(',')),
        identifier_parser
    )
);

named!(pub identifier_parser<Input, Identifier, ParserError>, fix_error!(ParserError, map!(
    verify!(
        recognize!(
            tuple!(
                one_of!(IDENTIFIER_BEGIN),
                opt!(is_a!(IDENTIFIER_END))
            )
        ),
        |id: Input| !is_valid(id)
    ),
    convert_identifier
)));

fn is_valid(input: Input) -> bool {
    if is_keyword(input) {
        return true;
    }

    if input.0.iter().all(u8::is_ascii_digit) {
        return true;
    }

    return false;
}

fn is_keyword(input: Input) -> bool {
    lazy_static! {
        static ref keywords: &'static [&'static [u8]] = &[b"if", b"var"];
    }
    for keyword in keywords.iter() {
        if input.eq_ignore_ascii_case(keyword) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::ErrorKind;
    use tests::utility::*;

    #[test]
    pub fn test_identifier_parser() {
        test_parser_done(identifier_parser, b"foo", Identifier::new(b"foo"), b"");
        test_parser_done(
            identifier_parser,
            b"MEM_InitAll",
            Identifier::new(b"MEM_InitAll"),
            b"",
        );

        test_parser_done(identifier_parser, b"123foo", Identifier::new(b"123foo"), b"");


        test_parser_error(
            identifier_parser,
            b"123",
            failure_result(b"123", ErrorKind::Verify),
        );
    }
}
