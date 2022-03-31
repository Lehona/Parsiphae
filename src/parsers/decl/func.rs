use crate::inner_errors::ParserError;
use crate::parsers::{identifier_parser, replacements::*, statement_block, var_decl_list_0};
use crate::types::{Function, Input};

named!(pub func<Input, Function, ParserError>, fix_error!(ParserError, do_parse!(
    tag_no_case_e!("func") >>   multispace1 >>
    typ: identifier_parser >>   multispace1 >>
    name: identifier_parser >>  multispace0 >>
    char_e!('(') >>             multispace0 >>
    params: var_decl_list_0 >>  multispace0 >>
    char_e!(')') >>             multispace0 >>
    body: statement_block >>    multispace0 >>
    (Function {typ, name, params, body})
)));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Expression, Identifier, Statement, VarDeclaration};

    #[test]
    fn empty_void() {
        let input = Input(b"func void foo() {}");
        let expected = Function {
            typ: Identifier::new(b"void"),
            name: Identifier::new(b"foo"),
            params: Vec::new(),
            body: Vec::new(),
        };

        let actual = func(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn params_empty_body() {
        let input = Input(b"func void foo(var int bar) {}");
        let expected = Function {
            typ: Identifier::new(b"void"),
            name: Identifier::new(b"foo"),
            params: vec![VarDeclaration::new(
                Identifier::new(b"int"),
                Identifier::new(b"bar"),
                None,
            )],
            body: Vec::new(),
        };

        let actual = func(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn body() {
        let input = Input(b"func void foo() {3;}");
        let expected = Function {
            typ: Identifier::new(b"void"),
            name: Identifier::new(b"foo"),
            params: Vec::new(),
            body: vec![Statement::Exp(Expression::Int(3))],
        };

        let actual = func(input).unwrap().1;

        assert_eq!(expected, actual);
    }
}
