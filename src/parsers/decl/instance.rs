use crate::inner_errors::ParserError;
use crate::parsers::{identifier_list, identifier_parser, statement_block};
use crate::types::{Input, Instance};

//
named!(instance_init<Input, Instance, ParserError>, fix_error!(ParserError, gws!(do_parse!(
    tag_no_case_e!("instance") >>
    name: identifier_parser >>
    char_e!('(') >>
    class: identifier_parser >>
    char_e!(')') >>
    body: statement_block >>
    (Instance {name, class, body})
))));

named!(instance_list<Input, Vec<Instance>, ParserError>, fix_error!(ParserError, gws!(do_parse!(
    tag_no_case_e!("instance") >>
    names: identifier_list >>
    char_e!('(') >>
    class: identifier_parser >>
    char_e!(')') >>
    (names.into_iter().map(|name|Instance { name, class: class.clone(), body: Vec::new()}).collect())
))));

named!(pub instance<Input, Vec<Instance>, ParserError>,
    alt!(
    map!(instance_init, |i|vec![i])
    | instance_list
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        Assignment, AssignmentOperator, Expression, Identifier, Statement, VarAccess,
    };

    #[test]
    fn simple() {
        let input = Input(b"instance foo (bar) {}");
        let expected = vec![Instance {
            name: Identifier::new(b"foo"),
            class: Identifier::new(b"bar"),
            body: Vec::new(),
        }];

        let actual = instance(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn init() {
        let input = Input(b"instance foo (bar) {name=3;}");
        let expected = vec![Instance {
            name: Identifier::new(b"foo"),
            class: Identifier::new(b"bar"),
            body: vec![Statement::Ass(Assignment {
                var: VarAccess::new(Identifier::new(b"name"), None, None),
                op: AssignmentOperator::Eq,
                exp: Expression::Int(3),
            })],
        }];

        let actual = instance(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn no_init() {
        let input = Input(b"instance foo (bar)");
        let expected = vec![Instance {
            name: Identifier::new(b"foo"),
            class: Identifier::new(b"bar"),
            body: Vec::new(),
        }];

        let actual = instance(input).unwrap().1;

        assert_eq!(expected, actual);
    }
}
