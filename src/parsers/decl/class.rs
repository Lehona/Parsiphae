use inner_errors::ParserError;
use parsers::util::flatten_vec;
use parsers::{identifier_parser, replacements::*, var_decl_list};
use types::{Class, Input};

named!(pub class<Input, Class, ParserError>, do_parse!(
    tag_no_case_e!("class") >>
    multispace0 >>
    cl: return_error!(ErrorKind::Custom(ParserError::ClassDeclaration), class_real) >>
    (cl)
));

named!(class_real<Input, Class, ParserError>, fix_error!(ParserError, gws!(do_parse!(

    name: identifier_parser >>
    char_e!('{') >>
    members: many0!(terminated!(var_decl_list,
        return_error!(ErrorKind::Custom(ParserError::MissingSemi), char_e!(';')))) >>
    char_e!('}') >>
    (Class {
        name,
        members: flatten_vec(members)
    })
))));

#[cfg(test)]
mod tests {
    use super::*;
    use types::{Identifier, VarDeclaration};

    #[test]
    fn simple() {
        let input = Input(b"class foo {var int bar;}");
        let expected = Class {
            name: Identifier::new(b"foo"),
            members: vec![VarDeclaration::new(
                Identifier::new(b"int"),
                Identifier::new(b"bar"),
                None,
            )],
        };

        let actual = class(input).unwrap().1;

        assert_eq!(expected, actual);
    }
    #[test]
    fn empty() {
        let input = Input(b"class foo {}");
        let expected = Class {
            name: Identifier::new(b"foo"),
            members: Vec::new(),
        };

        let actual = class(input).unwrap().1;

        assert_eq!(expected, actual);
    }
}
