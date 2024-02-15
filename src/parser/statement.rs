use crate::lexer::TokenKind;
use crate::parser::errors::{
    ParsePossibility as PP, ParsingError, ParsingErrorKind as PEK, Result,
};
use crate::types::{Assignment, IfBranch, IfStatement, ReturnStatement, Statement, VarDeclaration};
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
                // TODO: Add handling of recoverability
                match self.assignment() {
                    Ok(ass) => return Ok(Statement::Ass(ass)),
                    Err(e) if !e.recoverable => return Err(e),
                    _ => self.restore(ice),
                }

                match self.expression() {
                    Ok(exp) => return Ok(Statement::Exp(exp)),
                    Err(e) if !e.recoverable => return Err(e),
                    _ => self.restore(ice),
                }
                return Err(ParsingError::from_token(
                    PEK::ExpectedOneOf(vec![
                        PP::IfClause,
                        PP::Assignment,
                        PP::Expression,
                        PP::Declaration,
                    ]),
                    self.current_id(),
                    false,
                ));
            }
        })()?;

        if let Err(_e) = self.consume(TokenKind::Semicolon) {
            return Err(ParsingError::from_token(
                PEK::StatementWithoutSemicolon,
                self.current_id(),
                false,
            ));
        };

        Ok(stmt)
    }

    pub fn block(&mut self) -> Result<Vec<Statement>> {
        self.consume(TokenKind::BracketOpen)?;
        self.save_progress();
        let mut body = Vec::new();

        while !match_tok!(self, TokenKind::BracketClose) {
            body.push(self.statement()?);
            self.save_progress();
        }

        Ok(body)
    }

    pub fn if_statement(&mut self) -> Result<IfStatement> {
        self.consume(TokenKind::If)?;
        let if_begin = self.previous()?.span.0;

        let condition = self.expression()?;
        let body = self.block()?;
        let body_end = self.previous()?.span.1;

        let mut branches = vec![IfBranch {
            cond: condition,
            body,
            span: (if_begin, body_end),
        }];
        let mut else_branch = None;

        while match_tok!(self, TokenKind::Else) {
            let else_begin = self.previous()?.span.0;
            if match_tok!(self, TokenKind::If) {
                let condition2 = self.expression()?;
                let body2 = self.block()?;
                let body_end = self.previous()?.span.0;

                branches.push(IfBranch {
                    cond: condition2,
                    body: body2,
                    span: (else_begin, body_end),
                });
            } else {
                else_branch = Some(self.block()?);
                break;
            }
        }

        let if_end = self.previous()?.span.1;
        let stmt = IfStatement {
            branches,
            else_branch,
            span: (if_begin, if_end),
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

            let span = (left_side.span.0, exp.get_span().1);
            Ok(Assignment {
                var: left_side,
                op: op.try_into().unwrap(), // TODO: fix unwrap
                exp,
                span,
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
                self.current_id(),
                true,
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
        let start = self.span_start()?;
        self.consume(TokenKind::Return)?;

        let exp = if !self.check(TokenKind::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        let end = self.span_end()?;
        Ok(Statement::ReturnStatement(ReturnStatement {
            exp,
            span: (start, end),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::parser::Parser;
    use crate::types::{
        ArraySizeDeclaration, AssignmentOperator, BinaryExpression, BinaryOperator, Call,
        Expression, Identifier, IntNode, Statement, VarAccess,
    };

    #[test]
    fn simple_empty() {
        let lexed = Lexer::lex(b"if (3) {}").unwrap();
        println!("Lexed as: {:#?}", lexed);
        let expected = IfBranch {
            cond: Expression::Int(IntNode {
                value: 3,
                span: (4, 5),
            }),
            body: Vec::new(),
            span: (0, 9),
        };

        let mut parser = Parser::new(&lexed);
        let actual = &parser.if_statement().unwrap().branches[0];

        assert_eq!(&expected, actual);
    }

    #[test]
    fn single_statement() {
        let lexed = Lexer::lex(b"if (3) {5;}").unwrap();
        println!("Lexed as: {:#?}", lexed);
        let expected = IfBranch {
            cond: Expression::Int(IntNode {
                value: 3,
                span: (4, 5),
            }),
            body: vec![Statement::Exp(Expression::Int(IntNode {
                value: 5,
                span: (8, 9),
            }))],
            span: (0, 11),
        };

        let mut parser = Parser::new(&lexed);
        let actual = &parser.if_statement().unwrap().branches[0];

        assert_eq!(&expected, actual);
    }

    #[test]
    fn complex_cond() {
        let lexed = Lexer::lex(b"if (foo()||bar()) {5;}").unwrap();
        let cond = Expression::Binary(Box::new(BinaryExpression::new(
            BinaryOperator::Or,
            Expression::Call(Box::new(Call {
                func: Identifier::new(b"foo", (4, 7)),
                params: Vec::new(),
                span: (4, 9),
            })),
            Expression::Call(Box::new(Call {
                func: Identifier::new(b"bar", (11, 14)),
                params: Vec::new(),
                span: (11, 16),
            })),
            (4, 16),
        )));

        let expected = IfBranch {
            cond,
            body: vec![Statement::Exp(Expression::Int(IntNode {
                value: 5,
                span: (19, 20),
            }))],
            span: (0, 22),
        };

        let mut parser = Parser::new(&lexed);
        let actual = &parser.if_statement().unwrap().branches[0];

        assert_eq!(&expected, actual);
    }

    #[test]
    fn many_statements() {
        let lexed = Lexer::lex(b"if (3) {5;3;6;}").unwrap();
        println!("Lexed as: {:#?}", lexed);
        let expected = IfBranch {
            cond: Expression::Int(IntNode {
                value: 3,
                span: (4, 5),
            }),
            body: vec![
                Statement::Exp(Expression::Int(IntNode {
                    value: 5,
                    span: (8, 9),
                })),
                Statement::Exp(Expression::Int(IntNode {
                    value: 3,
                    span: (10, 11),
                })),
                Statement::Exp(Expression::Int(IntNode {
                    value: 6,
                    span: (12, 13),
                })),
            ],
            span: (0, 15),
        };

        let mut parser = Parser::new(&lexed);
        let actual = &parser.if_statement().unwrap().branches[0];

        assert_eq!(&expected, actual);
    }

    #[test]
    fn else_branch_empty() {
        let lexed = Lexer::lex(b"if 0 {} else{}").unwrap();
        let expected: Vec<Statement> = Vec::new();

        let mut parser = Parser::new(&lexed);
        let actual = &parser.if_statement().unwrap().else_branch.unwrap();

        assert_eq!(&expected, actual);
    }

    #[test]
    fn else_branch_single_statement() {
        let lexed = Lexer::lex(b"if 0 {} else {5;}").unwrap();
        let expected: Vec<Statement> = vec![Statement::Exp(Expression::Int(IntNode {
            value: 5,
            span: (14, 15),
        }))];

        let mut parser = Parser::new(&lexed);
        let actual = &parser.if_statement().unwrap().else_branch.unwrap();

        assert_eq!(&expected, actual);
    }

    #[test]
    fn if_else_empty() {
        let lexed = Lexer::lex(b"if 3 {} else {}").unwrap();
        println!("Lexed as: {:#?}", lexed);
        let expected = IfStatement {
            branches: vec![IfBranch {
                cond: Expression::Int(IntNode {
                    value: 3,
                    span: (3, 4),
                }),
                body: Vec::new(),
                span: (0, 7),
            }],
            else_branch: Some(Vec::new()),
            span: (0, 15),
        };

        let mut parser = Parser::new(&lexed);
        let actual = &parser.if_statement().unwrap();

        assert_eq!(&expected, actual);
    }

    #[test]
    fn double_if() {
        let lexed = Lexer::lex(b"if(3){} else if (2) {}").unwrap();
        println!("Lexed as: {:#?}", lexed);
        let expected = IfStatement {
            branches: vec![
                IfBranch {
                    cond: Expression::Int(IntNode {
                        value: 3,
                        span: (3, 4),
                    }),
                    body: Vec::new(),
                    span: (0, 7),
                },
                IfBranch {
                    cond: Expression::Int(IntNode {
                        value: 2,
                        span: (17, 18),
                    }),
                    body: Vec::new(),
                    span: (8, 21),
                },
            ],
            else_branch: None,
            span: (0, 22),
        };

        let mut parser = Parser::new(&lexed);
        let actual = &parser.if_statement().unwrap();

        assert_eq!(&expected, actual);
    }

    #[test]
    fn double_if_else() {
        let lexed = Lexer::lex(b"if(3){} else if (2) {} else {}").unwrap();
        let expected = IfStatement {
            branches: vec![
                IfBranch {
                    cond: Expression::Int(IntNode {
                        value: 3,
                        span: (3, 4),
                    }),
                    body: Vec::new(),
                    span: (0, 7),
                },
                IfBranch {
                    cond: Expression::Int(IntNode {
                        value: 2,
                        span: (17, 18),
                    }),
                    body: Vec::new(),
                    span: (8, 21),
                },
            ],
            else_branch: Some(Vec::new()),
            span: (0, 30),
        };

        let mut parser = Parser::new(&lexed);
        let actual = &parser.if_statement().unwrap();

        assert_eq!(&expected, actual);
    }

    #[test]
    fn simple_eq_int() {
        let lexed = Lexer::lex(b"foo=3").unwrap();
        let expected = Assignment {
            var: VarAccess::new(Identifier::new(b"foo", (0, 3)), None, None, (0, 3)),
            op: AssignmentOperator::Eq,
            exp: Expression::Int(IntNode {
                value: 3,
                span: (4, 5),
            }),
            span: (0, 5),
        };
        let mut parser = Parser::new(&lexed);
        let actual = parser.assignment().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn instance_eq_int() {
        let lexed = Lexer::lex(b"foo.bar=3").unwrap();
        let expected = Assignment {
            var: VarAccess::new(
                Identifier::new(b"foo", (0, 3)),
                Some(Identifier::new(b"bar", (4, 7))),
                None,
                (0, 7),
            ),
            op: AssignmentOperator::Eq,
            exp: Expression::Int(IntNode {
                value: 3,
                span: (8, 9),
            }),
            span: (0, 9),
        };
        let mut parser = Parser::new(&lexed);
        let actual = parser.assignment().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn instance_diveq_int() {
        let lexed = Lexer::lex(b"foo.bar/=3").unwrap();
        let expected = Assignment {
            var: VarAccess::new(
                Identifier::new(b"foo", (0, 3)),
                Some(Identifier::new(b"bar", (4, 7))),
                None,
                (0, 7),
            ),
            op: AssignmentOperator::DivideEq,
            exp: Expression::Int(IntNode {
                value: 3,
                span: (9, 10),
            }),
            span: (0, 10),
        };
        let mut parser = Parser::new(&lexed);
        let actual = parser.assignment().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn array_assign() {
        let lexed = Lexer::lex(b"foo[0]/=3").unwrap();
        let expected = Assignment {
            var: VarAccess::new(
                Identifier::new(b"foo", (0, 3)),
                None,
                Some(Expression::Int(IntNode {
                    value: 0,
                    span: (4, 5),
                })),
                (0, 6),
            ),
            op: AssignmentOperator::DivideEq,
            exp: Expression::Int(IntNode {
                value: 3,
                span: (8, 9),
            }),
            span: (0, 9),
        };
        let mut parser = Parser::new(&lexed);
        let actual = parser.assignment().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn array_assign_stmt() {
        let lexed = Lexer::lex(b"foo[0]/=3;").unwrap();
        let expected = Statement::Ass(Assignment {
            var: VarAccess::new(
                Identifier::new(b"foo", (0, 3)),
                None,
                Some(Expression::Int(IntNode {
                    value: 0,
                    span: (4, 5),
                })),
                (0, 6),
            ),
            op: AssignmentOperator::DivideEq,
            exp: Expression::Int(IntNode {
                value: 3,
                span: (8, 9),
            }),
            span: (0, 9),
        });
        let mut parser = Parser::new(&lexed);
        let actual = parser.statement().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn double_if_else_stmt() {
        let lexed = Lexer::lex(b"if(3){} else if (2) {} else {};").unwrap();
        println!("Lexed as: {:#?}", lexed);
        let expected = Statement::If(Box::new(IfStatement {
            branches: vec![
                IfBranch {
                    cond: Expression::Int(IntNode {
                        value: 3,
                        span: (3, 4),
                    }),
                    body: Vec::new(),
                    span: (0, 7),
                },
                IfBranch {
                    cond: Expression::Int(IntNode {
                        value: 2,
                        span: (17, 18),
                    }),
                    body: Vec::new(),
                    span: (8, 21),
                },
            ],
            else_branch: Some(Vec::new()),
            span: (0, 30),
        }));

        let mut parser = Parser::new(&lexed);
        let actual = &parser.statement().unwrap();

        assert_eq!(&expected, actual);
    }

    #[test]
    fn multi_var_decl() {
        let lexed = Lexer::lex(b"var int foo, var zCVob bar;");
        let expected = Statement::VarDeclarations(vec![
            VarDeclaration::new(
                Identifier::new(b"int", (4, 7)),
                Identifier::new(b"foo", (8, 11)),
                None,
                (0, 12),
            ),
            VarDeclaration::new(
                Identifier::new(b"zCVob", (17, 22)),
                Identifier::new(b"bar", (23, 26)),
                None,
                (13, 27),
            ),
        ]);

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.statement().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_var_decl_one_array() {
        let lexed = Lexer::lex(b"var int foo[3], var zCVob bar;");
        let expected = Statement::VarDeclarations(vec![
            VarDeclaration::new(
                Identifier::new(b"int", (4, 7)),
                Identifier::new(b"foo", (8, 11)),
                Some(ArraySizeDeclaration::Size(3)),
                (0, 15),
            ),
            VarDeclaration::new(
                Identifier::new(b"zCVob", (20, 25)),
                Identifier::new(b"bar", (26, 29)),
                None,
                (16, 30),
            ),
        ]);

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.statement().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_var_decl_var_camel_case() {
        /* the parser might parse this as "var int foo[3], vAr;, which mustn't happen */

        let lexed = Lexer::lex(b"var int foo[3], vAr zCVob bar;");
        let expected = vec![
            VarDeclaration::new(
                Identifier::new(b"int", (4, 7)),
                Identifier::new(b"foo", (8, 11)),
                Some(ArraySizeDeclaration::Size(3)),
                (0, 15),
            ),
            VarDeclaration::new(
                Identifier::new(b"zCVob", (20, 25)),
                Identifier::new(b"bar", (26, 29)),
                None,
                (16, 30),
            ),
        ];

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.var_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_var_decl_both_array() {
        let lexed = Lexer::lex(b"var int foo[3], var zCVob bar [MAX];");
        let expected = Statement::VarDeclarations(vec![
            VarDeclaration::new(
                Identifier::new(b"int", (4, 7)),
                Identifier::new(b"foo", (8, 11)),
                Some(ArraySizeDeclaration::Size(3)),
                (0, 15),
            ),
            VarDeclaration::new(
                Identifier::new(b"zCVob", (20, 25)),
                Identifier::new(b"bar", (26, 29)),
                Some(ArraySizeDeclaration::Identifier(Identifier::new(
                    b"MAX",
                    (31, 34),
                ))),
                (16, 36),
            ),
        ]);

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.statement().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn void_return() {
        let lexed = Lexer::lex(b"return;");
        let expected = Statement::ReturnStatement(ReturnStatement {
            exp: None,
            span: (0, 7),
        });

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.statement().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn int_return() {
        let lexed = Lexer::lex(b"return 3;");
        let expected = Statement::ReturnStatement(ReturnStatement {
            exp: Some(Expression::Int(IntNode {
                value: 3,
                span: (7, 8),
            })),
            span: (0, 9),
        });

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.statement().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn return_identifier() {
        let lexed = Lexer::lex(b"returnVar;");
        let expected = Statement::Exp(Expression::Identifier(Box::new(VarAccess::new(
            Identifier::new(b"returnVar", (0, 9)),
            None,
            None,
            (0, 9),
        ))));

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.statement().unwrap();

        assert_eq!(expected, actual);
    }
}
