use crate::inner_errors::ParserError;
use crate::parsers::{
    assignment, const_array_decl, const_decl, expression, if_clause, replacements::*, var_decl_list,
};
use crate::types::{Expression, Input, Statement};
use nom::ErrorKind;

named!(semi_statement<Input, Statement, ParserError>, do_parse!(
    state: add_return_error!(ErrorKind::Custom(ParserError::IllegalStatement), alt!(
         map!(var_decl_list, Statement::VarDeclarations)
        |map!(const_array_decl, Statement::ConstArrayDeclaration)
        |map!(const_decl, Statement::ConstDeclaration)
        |map!(assignment, Statement::Ass)
        |map!(expression, Statement::Exp)
        |map!(return_parser, Statement::ReturnStatement)
    )) >> multispace0 >>
    return_error!(ErrorKind::Custom(ParserError::MissingSemi), char_e!(';')) >> multispace0 >>
    (state)
));

named!(if_statement<Input, Statement, ParserError>, do_parse!(
    state: map!(if_clause, |i|Statement::If(Box::new(i))) >> multispace0 >>
    opt!(char_e!(';')) >> multispace0 >>
    (state)
));

named!(pub statement<Input, Statement, ParserError>, fix_error!(ParserError, do_parse!(
    state: alt!(semi_statement | if_statement) >>
   (state)
)));

named!(pub statement_block<Input, Vec<Statement>, ParserError>, do_parse!(
    char_e!('{') >> multispace0 >>
    body: many0!(statement) >> multispace0 >>
    add_return_error!(ErrorKind::Custom(ParserError::IllegalStatement), char_e!('}')) >> multispace0 >>
    (body)
));

named!(return_parser<Input, Option<Expression>, ParserError>, fix_error!(ParserError, gws!(preceded!(
    tag_no_case_e!("return"),
    opt!(expression)
))));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        Assignment, AssignmentOperator, Expression, Identifier, IfBranch, IfStatement,
        UnaryExpression, VarAccess,
    };

    #[test]
    fn assign() {
        let input = Input(b"foo = 3;");
        let expected = Statement::Ass(Assignment {
            var: VarAccess::new(Identifier::new(b"foo"), None, None),
            op: AssignmentOperator::Eq,
            exp: Expression::Int(3),
        });

        let actual = statement(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn expression() {
        let input = Input(b"!3 ;");
        let expected = Statement::Exp(Expression::Unary(Box::new(UnaryExpression::new(
            b'!',
            Expression::Int(3),
        ))));

        let actual = statement(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn if_clause() {
        let input = Input(b"if(3){4;};");
        let expected = Statement::If(Box::new(IfStatement {
            branches: vec![IfBranch {
                cond: Expression::Int(3),
                body: vec![Statement::Exp(Expression::Int(4))],
            }],
            else_branch: None,
        }));

        let actual = statement(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn void_return() {
        let input = Input(b"return;");
        let expected = Statement::ReturnStatement(None);

        let actual = statement(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn int_return() {
        let input = Input(b"return 3;");
        let expected = Statement::ReturnStatement(Some(Expression::Int(3)));

        let actual = statement(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn return_identifier() {
        let input = Input(b"returnVar;");
        let expected = Statement::Exp(Expression::Identifier(Box::new(VarAccess::new(
            Identifier::new(b"returnVar"),
            None,
            None,
        ))));

        let actual = statement(input).unwrap().1;

        assert_eq!(expected, actual);
    }
}
