use inner_errors::ParserError;
use parsers::replacements::*;
use parsers::{array_size_decl, expression, identifier_parser};
use types::{ConstArrayDeclaration, ConstArrayInitializer, ConstDeclaration, Input};

named!(pub const_decl<Input, ConstDeclaration, ParserError>, fix_error!(ParserError, do_parse!(
    tag_no_case_e!("const") >>
    multispace1 >>
    typ: identifier_parser >>
    multispace1 >>
    name: identifier_parser >>
    multispace0 >>
    char_e!('=') >>
    multispace0 >>
    initializer: expression >> multispace0 >>
    (ConstDeclaration::new(typ, name, initializer))
)));

named!(pub const_array_decl<Input, ConstArrayDeclaration, ParserError>, fix_error!(ParserError, do_parse!(
    tag_no_case_e!("const") >>
    multispace1 >>
    typ: identifier_parser >>
    multispace1 >>
    name: identifier_parser >>
    multispace0 >>
    array_size: array_size_decl >>
    multispace0 >>
    char_e!('=') >>
    multispace0 >>
    initializer: const_array_init >>
    (ConstArrayDeclaration::new(typ, name, array_size, initializer))
)));

named!(pub const_array_init<Input, ConstArrayInitializer, ParserError>, fix_error!(ParserError, map!(gws!(delimited!(
       char_e!('{'),
       gws!(separated_nonempty_list!(gws!(char_e!(',')), expression)),
       char_e!('}')
    )),
    ConstArrayInitializer::new
)));

#[cfg(test)]
mod const_tests {
    use super::*;
    use tests::utility::*;
    use types::{Expression, Identifier, UnaryExpression};

    #[test]
    pub fn decl_int_foo() {
        let init = Expression::Int(5);
        let decl = ConstDeclaration::new(Identifier::new(b"int"), Identifier::new(b"foo"), init);
        test_parser_done(const_decl, b"const int foo = 5", decl, b"");
    }

    #[test]
    pub fn decl_uppercase_const_int_foo() {
        let init = Expression::Int(14);
        let decl = ConstDeclaration::new(Identifier::new(b"int"), Identifier::new(b"foo"), init);
        test_parser_done(const_decl, b"CONST int foo= 14", decl, b"");
    }

    #[test]
    pub fn decl_zcvob_foo_unary() {
        let init = Expression::Unary(Box::new(UnaryExpression::new(b'!', Expression::Int(5))));
        let decl = ConstDeclaration::new(Identifier::new(b"zCVob"), Identifier::new(b"foo"), init);
        test_parser_done(const_decl, b"CONST zCVob foo = !5", decl, b"");
    }
}

#[cfg(test)]
mod const_array_tests {
    use super::*;
    use tests::utility::*;
    use types::*;

    #[test]
    pub fn decl_int_index() {
        let init = vec![
            Expression::Int(5),
            Expression::Int(6),
            Expression::Unary(Box::new(UnaryExpression::new(b'+', Expression::Int(12)))),
        ];
        let array_size = ArraySizeDeclaration::Size(3);
        let decl = ConstArrayDeclaration::new(
            Identifier::new(b"int"),
            Identifier::new(b"foo"),
            array_size,
            ConstArrayInitializer::new(init),
        );
        test_parser_done(
            const_array_decl,
            b"const int foo [ 3 ] = {5,6,+12}",
            decl,
            b"",
        );
    }

    #[test]
    pub fn decl_identifier_index() {
        let init = vec![
            Expression::Int(5),
            Expression::Int(6),
            Expression::Unary(Box::new(UnaryExpression::new(b'+', Expression::Int(12)))),
        ];
        let array_size = ArraySizeDeclaration::Identifier(Identifier::new(b"MAX_SIZE"));
        let decl = ConstArrayDeclaration::new(
            Identifier::new(b"int"),
            Identifier::new(b"foo"),
            array_size,
            ConstArrayInitializer::new(init),
        );
        test_parser_done(
            const_array_decl,
            b"const int foo [ MAX_SIZE ] = {5, 6, +12}",
            decl,
            b"",
        );
    }

    #[test]
    pub fn decl_string_initializer() {
        let init = vec![
            Expression::String(StringLiteral::new(b"hello")),
            Expression::Float(6.0),
            Expression::Unary(Box::new(UnaryExpression::new(b'+', Expression::Int(12)))),
        ];
        let array_size = ArraySizeDeclaration::Identifier(Identifier::new(b"MAX_SIZE"));
        let decl = ConstArrayDeclaration::new(
            Identifier::new(b"int"),
            Identifier::new(b"foo"),
            array_size,
            ConstArrayInitializer::new(init),
        );
        test_parser_done(
            const_array_decl,
            b"const int foo[ MAX_SIZE ] = {\"hello\", 6.0, +12}",
            decl,
            b"",
        );
    }
}
