use inner_errors::ParserError;
use nom::ErrorKind;
use parsers::replacements::*;
use parsers::{identifier_parser, number_parser};
use types::{ArraySizeDeclaration, Identifier, Input, VarDeclaration};

named!(pub var_decl<Input, Vec<VarDeclaration>, ParserError>, do_parse!(
    tag_no_case_e!("var") >> multispace1 >>
    decls: return_error!(ErrorKind::Custom(ParserError::VariableDeclaration), var_decl_real) >>
    (decls)
));

named!(var_decl_real<Input, Vec<VarDeclaration>, ParserError>, fix_error!(ParserError, do_parse!(
    typ: identifier_parser >>
    multispace1 >>
    identifier_decls: separated_nonempty_list!(
        gws!(char_e!(',')),
        var_identifier_decl
    ) >>
    (identifier_decls.into_iter().map(|(name, array)|VarDeclaration::new(typ.clone(), name, array)).collect())
)));

named!(var_identifier_decl<Input, (Identifier, Option<ArraySizeDeclaration>), ParserError>, fix_error!(ParserError, gws!(
    pair!(
        identifier_parser,
        opt!(array_size_decl)
    )
)));

named!(pub var_decl_list<Input, Vec<VarDeclaration>, ParserError>, fix_error!(ParserError,
    map!(gws!(separated_nonempty_list!(
        gws!(char_e!(',')),
        var_decl
    )),
    |decl: Vec<Vec<_>>| decl.into_iter().flat_map(|inner|inner.into_iter()).collect()
)));

named!(pub var_decl_list_0<Input, Vec<VarDeclaration>, ParserError>, fix_error!(ParserError,
    map!(gws!(separated_list!(
        gws!(char_e!(',')),
        var_decl
    )),
    |decl: Vec<Vec<_>>| decl.into_iter().flat_map(|inner|inner.into_iter()).collect()
)));

named!(pub array_size_decl<Input, ArraySizeDeclaration, ParserError>, fix_error!(ParserError, do_parse!(
    char_e!('[') >> multispace0 >>
    size: alt!(
          map!(identifier_parser, ArraySizeDeclaration::Identifier)
        | map!(number_parser, ArraySizeDeclaration::Size)
    ) >> multispace0 >>
    char_e!(']') >> multispace0 >>
    (size)
)));

#[cfg(test)]
mod tests {
    use super::*;
    use types::Identifier;

    #[test]
    fn multi_var_decl() {
        let input = Input(b"var int foo, var zCVob bar");
        let expected = vec![
            VarDeclaration::new(Identifier::new(b"int"), Identifier::new(b"foo"), None),
            VarDeclaration::new(Identifier::new(b"zCVob"), Identifier::new(b"bar"), None),
        ];

        let actual = var_decl_list(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_var_decl_one_array() {
        let input = Input(b"var int foo[3], var zCVob bar");
        let expected = vec![
            VarDeclaration::new(
                Identifier::new(b"int"),
                Identifier::new(b"foo"),
                Some(ArraySizeDeclaration::Size(3)),
            ),
            VarDeclaration::new(Identifier::new(b"zCVob"), Identifier::new(b"bar"), None),
        ];

        let actual = var_decl_list(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_var_decl_var_camel_case() {
        /* the parser might parse this as "var int foo[3], vAr;, which mustn't happen */
        let input = Input(b"var int foo[3], vAr zCVob bar");
        let expected = vec![
            VarDeclaration::new(
                Identifier::new(b"int"),
                Identifier::new(b"foo"),
                Some(ArraySizeDeclaration::Size(3)),
            ),
            VarDeclaration::new(Identifier::new(b"zCVob"), Identifier::new(b"bar"), None),
        ];

        let actual = var_decl_list(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_var_decl_both_array() {
        let input = Input(b"var int foo[3], var zCVob bar [MAX]");
        let expected = vec![
            VarDeclaration::new(
                Identifier::new(b"int"),
                Identifier::new(b"foo"),
                Some(ArraySizeDeclaration::Size(3)),
            ),
            VarDeclaration::new(
                Identifier::new(b"zCVob"),
                Identifier::new(b"bar"),
                Some(ArraySizeDeclaration::Identifier(Identifier::new(b"MAX"))),
            ),
        ];

        let actual = var_decl_list(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn int_foo_decl() {
        let input = Input(b"var int foo");
        let expected = vec![VarDeclaration::new(
            Identifier::new(b"int"),
            Identifier::new(b"foo"),
            None,
        )];

        let actual = var_decl(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn int_array_size() {
        let input = Input(b"[13]");
        let expected = ArraySizeDeclaration::Size(13);

        let actual = array_size_decl(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn identifier_array_size() {
        let input = Input(b"[MAX]");
        let expected = ArraySizeDeclaration::Identifier(Identifier::new(b"MAX"));

        let actual = array_size_decl(input).unwrap().1;

        assert_eq!(expected, actual);
    }

}
