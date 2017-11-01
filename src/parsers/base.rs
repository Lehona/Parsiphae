use pom::parser::*;


pub fn string_parser() -> Parser<'static, u8, String> {
    let parser = sym(b'"')
        * none_of(b"\"").repeat(..).collect().map(|vec|String::from_utf8_lossy(&vec).to_string())
        - sym(b'"');

    parser
}

pub fn number_parser() -> Parser<'static, u8, i32> {
    let integer = one_of(b"0123456789").repeat(1..);
    let number = sym(b'-').opt() + integer;
    let val = number.collect().convert(String::from_utf8).convert(|str|str.parse::<i32>());
    val
}

pub fn float() -> Parser<'static, u8, f32> {
    let parser = number_parser() - sym(b'.') + number_parser();

    parser.collect().map(String::from_utf8).map(|s|s.unwrap().parse::<f32>().unwrap())
}



const IDENTIFIER_BEGIN: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
const IDENTIFIER_END: &'static [u8] =   b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_^@1234567890\xC4";

pub fn identifier() -> Parser<'static, u8, String> {
    let name = one_of(IDENTIFIER_BEGIN) + one_of(IDENTIFIER_END).repeat(0..);

    name.collect().map(|vec|String::from_utf8_lossy(&vec).to_string())
}