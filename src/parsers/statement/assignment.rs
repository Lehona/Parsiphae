use inner_errors::ParserError;
use parsers::{expression, var_access};
use types::{Assignment, AssignmentOperator, Input};

named!(pub assignment<Input, Assignment, ParserError>, fix_error!(ParserError, gws!(do_parse!(
    lhs: var_access >> multispace0 >>
    op: alt!(
            tag_e!("+=")
          | tag_e!("-=")
          | tag_e!("*=")
          | tag_e!("/=")
          | tag_e!("=")
         ) >> multispace0 >>
    rhs: expression >> multispace0 >>
    (Assignment {
        var: lhs,
        op: AssignmentOperator::from(&op.0),
        exp: rhs
    })
))));

#[cfg(test)]
mod tests {
    use super::*;
    use types::{Expression, Identifier, VarAccess};

    #[test]
    fn simple_eq_int() {
        let input = Input(b"foo=3");
        let expected = Assignment {
            var: VarAccess::new(Identifier::new(b"foo"), None, None),
            op: AssignmentOperator::Eq,
            exp: Expression::Int(3),
        };

        let actual = assignment(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn instance_eq_int() {
        let input = Input(b"foo.bar=3");
        let expected = Assignment {
            var: VarAccess::new(Identifier::new(b"foo"), Some(Identifier::new(b"bar")), None),
            op: AssignmentOperator::Eq,
            exp: Expression::Int(3),
        };

        let actual = assignment(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn instance_diveq_int() {
        let input = Input(b"foo.bar/=3");
        let expected = Assignment {
            var: VarAccess::new(Identifier::new(b"foo"), Some(Identifier::new(b"bar")), None),
            op: AssignmentOperator::DivideEq,
            exp: Expression::Int(3),
        };

        let actual = assignment(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn array_assign() {
        let input = Input(b"foo[0]/=3");
        let expected = Assignment {
            var: VarAccess::new(Identifier::new(b"foo"), None, Some(Expression::Int(0))),
            op: AssignmentOperator::DivideEq,
            exp: Expression::Int(3),
        };

        let actual = assignment(input).unwrap().1;

        assert_eq!(expected, actual);
    }
}
