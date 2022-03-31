use crate::parser::errors::{
    ParsePossibility as PP, ParsingError, ParsingErrorKind as PEK, Result,
};
use crate::lexer::TokenKind;
use crate::types::{Assignment, IfBranch, IfStatement, Statement, VarDeclaration};
use std::convert::TryInto;

impl crate::parser::parser::Parser {
    pub fn statement(&mut self) -> Result<Statement> {
        let ice = self.freeze();

        let stmt = (|| {
            if self.check(TokenKind::If) {
                return Ok(Statement::If(Box::new(self.if_statement()?)));
            } else if self.check(TokenKind::Var) {
                return Ok(Statement::VarDeclarations(self.var_decl()?));
            } else if self.check(TokenKind::Const) {
                return Ok(self.const_decl_stmt()?);
            } else if self.check(TokenKind::Return) {
                return Ok(self.return_statement()?);
            } else {
                match self.assignment() {
                    Ok(ass) => return Ok(Statement::Ass(ass)),
                    _ => self.restore(ice),
                }

                match self.expression() {
                    Ok(exp) => return Ok(Statement::Exp(exp)),
                    _ => self.restore(ice),
                }
                return Err(ParsingError::from_token(
                    PEK::ExpectedOneOf(vec![
                        PP::IfClause,
                        PP::Assignment,
                        PP::Expression,
                        PP::Declaration,
                    ]),
                    self.current_ref()?,
                ));
            }
        })()?;

        self.consume(TokenKind::Semicolon)?;

        Ok(stmt)
    }

    pub fn block(&mut self) -> Result<Vec<Statement>> {
        self.consume(TokenKind::BracketOpen)?;
        let mut body = Vec::new();

        while !match_tok!(self, TokenKind::BracketClose) {
            body.push(self.statement()?);
        }

        Ok(body)
    }

    pub fn if_statement(&mut self) -> Result<IfStatement> {
        self.consume(TokenKind::If)?;

        let condition = self.expression()?;
        let body = self.block()?;

        let mut branches = vec![IfBranch {
            cond: condition,
            body,
        }];
        let mut else_branch = None;

        while match_tok!(self, TokenKind::Else) {
            if match_tok!(self, TokenKind::If) {
                let condition2 = self.expression()?;
                let body2 = self.block()?;

                branches.push(IfBranch {
                    cond: condition2,
                    body: body2,
                });
            } else {
                else_branch = Some(self.block()?);
                break;
            }
        }

        let stmt = IfStatement {
            branches,
            else_branch,
        };
        Ok(stmt)
    }

    pub fn assignment(&mut self) -> Result<Assignment> {
        let left_side = self.var_access()?;

        if match_tok!(
            self,
            TokenKind::Assign,
            TokenKind::PlusAssign,
            TokenKind::MinusAssign,
            TokenKind::DivideAssign,
            TokenKind::MultiplyAssign
        ) {
            let op = self.previous()?.kind;
            let exp = self.expression()?;

            Ok(Assignment {
                var: left_side,
                op: op.try_into().unwrap(), // TODO: fix unwrap
                exp,
            })
        } else {
            return Err(ParsingError::from_token(
                PEK::ExpectedOneOfToken(vec![
                    TokenKind::Assign,
                    TokenKind::PlusAssign,
                    TokenKind::MinusAssign,
                    TokenKind::DivideAssign,
                    TokenKind::MultiplyAssign,
                ]),
                self.current_ref()?,
            ));
        }
    }

    pub fn decl_statement(&mut self) -> Result<Vec<VarDeclaration>> {
        let mut decls = Vec::new();

        while self.check(TokenKind::Var) {
            let decl = self.var_decl()?;

            decls.extend(decl);

            if !match_tok!(self, TokenKind::Comma) {
                break;
            }
        }

        Ok(decls)
    }

    pub fn return_statement(&mut self) -> Result<Statement> {
        self.consume(TokenKind::Return)?;

        let exp = if !self.check(TokenKind::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        Ok(Statement::ReturnStatement(exp))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parser::Parser;
    use crate::lexer::lex;
    use crate::types::{
        ArraySizeDeclaration, AssignmentOperator, BinaryExpression, BinaryOperator, Call,
        Expression, Identifier, Statement, VarAccess,
    };

    #[test]
    fn simple_empty() {
        let lexed = lex(b"if (3) {}").unwrap();
        println!("Lexed as: {:#?}", lexed);
        let expected = IfBranch {
            cond: Expression::Int(3),
            body: Vec::new(),
        };

        let mut parser = Parser::new(lexed);
        let actual = &parser.if_statement().unwrap().branches[0];

        assert_eq!(&expected, actual);
    }

    #[test]
    fn single_statement() {
        let lexed = lex(b"if (3) {5;}").unwrap();
        println!("Lexed as: {:#?}", lexed);
        let expected = IfBranch {
            cond: Expression::Int(3),
            body: vec![Statement::Exp(Expression::Int(5))],
        };

        let mut parser = Parser::new(lexed);
        let actual = &parser.if_statement().unwrap().branches[0];

        assert_eq!(&expected, actual);
    }

    #[test]
    fn complex_cond() {
        let lexed = lex(b"if (foo()||bar()) {5;}").unwrap();
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

        println!("Lexed as: {:#?}", lexed);
        let expected = IfBranch {
            cond,
            body: vec![Statement::Exp(Expression::Int(5))],
        };

        let mut parser = Parser::new(lexed);
        let actual = &parser.if_statement().unwrap().branches[0];

        assert_eq!(&expected, actual);
    }

    #[test]
    fn many_statements() {
        let lexed = lex(b"if (3) {5;3;6;}").unwrap();
        println!("Lexed as: {:#?}", lexed);
        let expected = IfBranch {
            cond: Expression::Int(3),
            body: vec![
                Statement::Exp(Expression::Int(5)),
                Statement::Exp(Expression::Int(3)),
                Statement::Exp(Expression::Int(6)),
            ],
        };

        let mut parser = Parser::new(lexed);
        let actual = &parser.if_statement().unwrap().branches[0];

        assert_eq!(&expected, actual);
    }

    #[test]
    fn else_branch_empty() {
        let lexed = lex(b"if 0 {} else{}").unwrap();
        let expected: Vec<Statement> = Vec::new();

        let mut parser = Parser::new(lexed);
        let actual = &parser.if_statement().unwrap().else_branch.unwrap();

        assert_eq!(&expected, actual);
    }

    #[test]
    fn else_branch_single_statement() {
        let lexed = lex(b"if 0 {} else {5;}").unwrap();
        let expected: Vec<Statement> = vec![Statement::Exp(Expression::Int(5))];

        let mut parser = Parser::new(lexed);
        let actual = &parser.if_statement().unwrap().else_branch.unwrap();

        assert_eq!(&expected, actual);
    }

    #[test]
    fn if_else_empty() {
        let lexed = lex(b"if 3 {} else {}").unwrap();
        println!("Lexed as: {:#?}", lexed);
        let expected = IfStatement {
            branches: vec![IfBranch {
                cond: Expression::Int(3),
                body: Vec::new(),
            }],
            else_branch: Some(Vec::new()),
        };

        let mut parser = Parser::new(lexed);
        let actual = &parser.if_statement().unwrap();

        assert_eq!(&expected, actual);
    }

    #[test]
    fn double_if() {
        let lexed = lex(b"if(3){} else if (2) {}").unwrap();
        println!("Lexed as: {:#?}", lexed);
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

        let mut parser = Parser::new(lexed);
        let actual = &parser.if_statement().unwrap();

        assert_eq!(&expected, actual);
    }

    #[test]
    fn double_if_else() {
        let lexed = lex(b"if(3){} else if (2) {} else {}").unwrap();
        println!("Lexed as: {:#?}", lexed);
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

        let mut parser = Parser::new(lexed);
        let actual = &parser.if_statement().unwrap();

        assert_eq!(&expected, actual);
    }

    #[test]
    fn simple_eq_int() {
        let lexed = lex(b"foo=3").unwrap();
        let expected = Assignment {
            var: VarAccess::new(Identifier::new(b"foo"), None, None),
            op: AssignmentOperator::Eq,
            exp: Expression::Int(3),
        };
        let mut parser = Parser::new(lexed);
        let actual = parser.assignment().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn instance_eq_int() {
        let lexed = lex(b"foo.bar=3").unwrap();
        let expected = Assignment {
            var: VarAccess::new(Identifier::new(b"foo"), Some(Identifier::new(b"bar")), None),
            op: AssignmentOperator::Eq,
            exp: Expression::Int(3),
        };
        let mut parser = Parser::new(lexed);
        let actual = parser.assignment().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn instance_diveq_int() {
        let lexed = lex(b"foo.bar/=3").unwrap();
        let expected = Assignment {
            var: VarAccess::new(Identifier::new(b"foo"), Some(Identifier::new(b"bar")), None),
            op: AssignmentOperator::DivideEq,
            exp: Expression::Int(3),
        };
        let mut parser = Parser::new(lexed);
        let actual = parser.assignment().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn array_assign() {
        let lexed = lex(b"foo[0]/=3").unwrap();
        let expected = Assignment {
            var: VarAccess::new(Identifier::new(b"foo"), None, Some(Expression::Int(0))),
            op: AssignmentOperator::DivideEq,
            exp: Expression::Int(3),
        };
        let mut parser = Parser::new(lexed);
        let actual = parser.assignment().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn array_assign_stmt() {
        let lexed = lex(b"foo[0]/=3;").unwrap();
        let expected = Statement::Ass(Assignment {
            var: VarAccess::new(Identifier::new(b"foo"), None, Some(Expression::Int(0))),
            op: AssignmentOperator::DivideEq,
            exp: Expression::Int(3),
        });
        let mut parser = Parser::new(lexed);
        let actual = parser.statement().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn double_if_else_stmt() {
        let lexed = lex(b"if(3){} else if (2) {} else {};").unwrap();
        println!("Lexed as: {:#?}", lexed);
        let expected = Statement::If(Box::new(IfStatement {
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
        }));

        let mut parser = Parser::new(lexed);
        let actual = &parser.statement().unwrap();

        assert_eq!(&expected, actual);
    }

    #[test]
    fn multi_var_decl() {
        let lexed = lex(b"var int foo, var zCVob bar;");
        let expected = Statement::VarDeclarations(vec![
            VarDeclaration::new(Identifier::new(b"int"), Identifier::new(b"foo"), None),
            VarDeclaration::new(Identifier::new(b"zCVob"), Identifier::new(b"bar"), None),
        ]);

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.statement().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_var_decl_one_array() {
        let lexed = lex(b"var int foo[3], var zCVob bar;");
        let expected = Statement::VarDeclarations(vec![
            VarDeclaration::new(
                Identifier::new(b"int"),
                Identifier::new(b"foo"),
                Some(ArraySizeDeclaration::Size(3)),
            ),
            VarDeclaration::new(Identifier::new(b"zCVob"), Identifier::new(b"bar"), None),
        ]);

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.statement().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_var_decl_var_camel_case() {
        /* the parser might parse this as "var int foo[3], vAr;, which mustn't happen */
        
        let lexed = lex(b"var int foo[3], vAr zCVob bar;");
        let expected = vec![
            VarDeclaration::new(
                Identifier::new(b"int"),
                Identifier::new(b"foo"),
                Some(ArraySizeDeclaration::Size(3)),
            ),
            VarDeclaration::new(Identifier::new(b"zCVob"), Identifier::new(b"bar"), None),
        ];

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.var_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_var_decl_both_array() {
        let lexed = lex(b"var int foo[3], var zCVob bar [MAX];");
        let expected = Statement::VarDeclarations(vec![
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
        ]);

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.statement().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn void_return() {
        let lexed = lex(b"return;");
        let expected = Statement::ReturnStatement(None);

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.statement().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn int_return() {
        let lexed = lex(b"return 3;");
        let expected = Statement::ReturnStatement(Some(Expression::Int(3)));

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.statement().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn return_identifier() {
        let lexed = lex(b"returnVar;");
        let expected = Statement::Exp(Expression::Identifier(Box::new(VarAccess::new(
            Identifier::new(b"returnVar"),
            None,
            None,
        ))));

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.statement().unwrap();

        assert_eq!(expected, actual);
    }
}
