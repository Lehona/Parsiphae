use parsers::var_decl;
use tests::utility::*;
use types::{ArraySizeDeclaration, Identifier, VarDeclaration};

#[test]
pub fn test_var_decl() {
    let decl = vec![VarDeclaration::new(
        Identifier::new(b"int"),
        Identifier::new(b"foo"),
        None,
    )];
    test_parser_done(var_decl, b"var int foo", decl, b"");

    let decl = vec![VarDeclaration::new(
        Identifier::new(b"zCVob"),
        Identifier::new(b"foo"),
        None,
    )];
    test_parser_done(var_decl, b"var zCVob foo", decl, b"");

    let decl = vec![VarDeclaration::new(
        Identifier::new(b"int"),
        Identifier::new(b"foo"),
        None,
    )];
    test_parser_done(var_decl, b"Var int foo", decl, b"");

    let decl = vec![VarDeclaration::new(
        Identifier::new(b"int"),
        Identifier::new(b"foo"),
        None,
    )];
    test_parser_done(var_decl, b"VAR int foo", decl, b"");

    let decl = vec![VarDeclaration::new(
        Identifier::new(b"int"),
        Identifier::new(b"foo"),
        None,
    )];
    test_parser_done(var_decl, b"VaR int foo", decl, b"");

    let decl = vec![VarDeclaration::new(
        Identifier::new(b"int"),
        Identifier::new(b"foo"),
        Some(ArraySizeDeclaration::Size(3)),
    )];
    test_parser_done(var_decl, b"var int foo[3]", decl, b"");

    let decl = vec![VarDeclaration::new(
        Identifier::new(b"int"),
        Identifier::new(b"foo"),
        Some(ArraySizeDeclaration::Identifier(Identifier::new(
            b"MAX_SIZE",
        ))),
    )];
    test_parser_done(var_decl, b"var int foo [ MAX_SIZE]", decl, b"");
}
