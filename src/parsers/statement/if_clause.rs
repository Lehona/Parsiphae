use inner_errors::ParserError;
use parsers::{expression, replacements::*, statement_block};
use types::{IfBranch, IfStatement, Input, Statement};

named!(pub if_branch<Input, IfBranch, ParserError>, do_parse!(
    tag_no_case_e!("if") >> multispace0 >>
    branch: return_error!(ErrorKind::Custom(ParserError::IfClause), if_branch_real) >> multispace0 >>
    (branch)
));

named!(if_branch_real<Input, IfBranch, ParserError>, fix_error!(ParserError, do_parse!(
    cond: return_error!(ErrorKind::Custom(ParserError::IllegalExpression), expression) >> multispace0 >>
    body: statement_block >> multispace0 >>
    (IfBranch {cond, body})
)));

named!(pub else_branch<Input, Vec<Statement>, ParserError>, do_parse!(
    tag_no_case_e!("else") >> multispace0 >>
    branch: return_error!(ErrorKind::Custom(ParserError::ElseClause), else_branch_real) >>
    (branch)
));

named!(else_branch_real<Input, Vec<Statement>, ParserError>, fix_error!(ParserError, do_parse!(
    body: statement_block >> multispace0 >>
    (body)
)));

named!(pub if_clause<Input, IfStatement, ParserError>, fix_error!(ParserError, do_parse!(
    branches: separated_nonempty_list!(
        gws!(tag_no_case_e!("else")),
        if_branch) >> multispace0 >>

    else_branch: opt!(else_branch) >> multispace0 >>

    (IfStatement { branches, else_branch})
)));

#[cfg(test)]
mod tests {
    use super::*;
    use types::{BinaryExpression, BinaryOperator, Call, Expression, Identifier, Statement};

    #[test]
    fn simple_empty() {
        let input = Input(b"if (3) {}");
        let expected = IfBranch {
            cond: Expression::Int(3),
            body: Vec::new(),
        };

        let actual = if_branch(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn single_statement() {
        let input = Input(b"if (3) {5;}");
        let expected = IfBranch {
            cond: Expression::Int(3),
            body: vec![Statement::Exp(Expression::Int(5))],
        };

        let actual = if_branch(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn complex_cond() {
        let input = Input(b"if (foo()||bar()) {5;}");
        let cond = Expression::Binary(Box::new(BinaryExpression::new(
            BinaryOperator::Or,
            Expression::Call(Box::new(Call {
                func: Identifier::new(b"foo"),
                params: Vec::new(),
            })),
            Expression::Call(Box::new(Call {
                func: Identifier::new(b"bar"),
                params: Vec::new(),
            })),
        )));

        let expected = IfBranch {
            cond,
            body: vec![Statement::Exp(Expression::Int(5))],
        };

        let actual = if_branch(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn many_statements() {
        let input = Input(b"if (3) {5;3;6;}");
        let expected = IfBranch {
            cond: Expression::Int(3),
            body: vec![
                Statement::Exp(Expression::Int(5)),
                Statement::Exp(Expression::Int(3)),
                Statement::Exp(Expression::Int(6)),
            ],
        };

        let actual = if_branch(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn else_branch_empty() {
        let input = Input(b"else{}");
        let expected: Vec<Statement> = Vec::new();

        let actual = else_branch(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn else_branch_single_statement() {
        let input = Input(b"else{5;}");
        let expected: Vec<Statement> = vec![Statement::Exp(Expression::Int(5))];

        let actual = else_branch(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn if_else_empty() {
        let input = Input(b"if3{}else{}");
        let expected = IfStatement {
            branches: vec![IfBranch {
                cond: Expression::Int(3),
                body: Vec::new(),
            }],
            else_branch: Some(Vec::new()),
        };

        let actual = if_clause(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn double_if() {
        let input = Input(b"if(3){} else if (2) {}");
        let expected = IfStatement {
            branches: vec![
                IfBranch {
                    cond: Expression::Int(3),
                    body: Vec::new(),
                },
                IfBranch {
                    cond: Expression::Int(2),
                    body: Vec::new(),
                },
            ],
            else_branch: None,
        };

        let actual = if_clause(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn double_if_else() {
        let input = Input(b"if(3){} else if (2) {} else {}");
        let expected = IfStatement {
            branches: vec![
                IfBranch {
                    cond: Expression::Int(3),
                    body: Vec::new(),
                },
                IfBranch {
                    cond: Expression::Int(2),
                    body: Vec::new(),
                },
            ],
            else_branch: Some(Vec::new()),
        };

        let actual = if_clause(input).unwrap().1;

        assert_eq!(expected, actual);
    }
}
