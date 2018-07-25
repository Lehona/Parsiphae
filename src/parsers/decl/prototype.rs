use inner_errors::ParserError;
use parsers::{identifier_parser, statement};
use types::{Input, Prototype};

named!(pub prototype<Input, Prototype, ParserError>, fix_error!(ParserError, gws!(do_parse!(
    tag_no_case_e!("prototype") >>
    name: identifier_parser >>
    char_e!('(') >>
    class: identifier_parser >>
    char_e!(')') >>
    char_e!('{') >>
    body: many0!(statement) >>
    char_e!('}') >>
    (Prototype {name, class, body})
))));

#[cfg(test)]
mod tests {
    use super::*;
    use types::{Assignment, AssignmentOperator, Expression, Identifier, Statement, VarAccess};

    #[test]
    fn simple() {
        let input = Input(b"prototype foo (bar) {}");
        let expected = Prototype {
            name: Identifier::new(b"foo"),
            class: Identifier::new(b"bar"),
            body: Vec::new(),
        };

        let actual = prototype(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn init() {
        let input = Input(b"prototype foo (bar) {name=3;}");
        let expected = Prototype {
            name: Identifier::new(b"foo"),
            class: Identifier::new(b"bar"),
            body: vec![Statement::Ass(Assignment {
                var: VarAccess::new(Identifier::new(b"name"), None, None),
                op: AssignmentOperator::Eq,
                exp: Expression::Int(3),
            })],
        };

        let actual = prototype(input).unwrap().1;

        assert_eq!(expected, actual);
    }
}
