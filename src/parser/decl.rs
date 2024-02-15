use crate::lexer::TokenKind;
use crate::types::{
    ArraySizeDeclaration, Class, ConstArrayDeclaration, ConstArrayInitializer, ConstDeclaration,
    Declaration, Statement, VarDeclaration,
};

use super::errors::{ParsingError, ParsingErrorKind as PEK};
use crate::parser::errors::Result;

impl crate::parser::parser::Parser {
    pub fn var_decl(&mut self) -> Result<Vec<VarDeclaration>> {
        let mut decls = Vec::new();
        decls.push(self.single_var_decl()?);
        self.save_progress();

        while match_tok!(self, TokenKind::Comma) {
            if self.check(TokenKind::Var) {
                decls.push(self.single_var_decl()?);
            } else {
                let start = self.span_start()?;
                let next_name = self.ident()?;

                let next_size_decl = optional_match!(self, self.array_size_decl());

                let end = self.span_end()?;
                decls.push(VarDeclaration::new(
                    decls[0].typ.clone(),
                    next_name,
                    next_size_decl,
                    (start, end),
                ));
            }
            self.save_progress();
        }

        Ok(decls)
    }

    fn array_size_decl(&mut self) -> Result<ArraySizeDeclaration> {
        let arr_size = if match_tok!(self, TokenKind::SquareOpen) {
            if let TokenKind::Integer(size) = self.current_ref()?.kind {
                self.advance();
                ArraySizeDeclaration::Size(size)
            } else {
                let ident = self.ident()?;
                ArraySizeDeclaration::Identifier(ident)
            }
        } else {
            return Err(ParsingError::from_token(
                PEK::ExpectedToken(TokenKind::SquareOpen),
                self.current_id(),
                true,
            ));
        };

        self.consume(TokenKind::SquareClose)?;

        Ok(arr_size)
    }

    pub fn single_var_decl(&mut self) -> Result<VarDeclaration> {
        let start = self.span_start()?;
        self.consume(TokenKind::Var)?;

        let typ = self.ident()?;
        let name = self.ident()?;

        let ice = self.freeze();

        let size_decl = if let Ok(decl) = self.array_size_decl() {
            Some(decl)
        } else {
            self.restore(ice);
            None
        };

        let end = self.span_end()?;

        Ok(VarDeclaration::new(
            typ.clone(),
            name,
            size_decl,
            (start, end),
        ))
    }

    pub fn const_decl_stmt(&mut self) -> Result<Statement> {
        let decl_begin = self.span_start()?;
        self.consume(TokenKind::Const)?;

        let typ = self.ident()?;
        let name = self.ident()?;

        if match_tok!(self, TokenKind::Assign) {
            let initializer = self.expression()?;
            let decl_end = self.span_end()?;

            return Ok(Statement::ConstDeclaration(ConstDeclaration::new(
                typ,
                name,
                initializer,
                (decl_begin, decl_end),
            )));
        } else if self.check(TokenKind::SquareOpen) {
            let array_size = self.array_size_decl()?;

            self.consume(TokenKind::Assign)?;

            self.consume(TokenKind::BracketOpen)?;
            let initializer_start = self.span_start()?;
            let initializer_exps = self.expression_list()?;
            let initializer_end = self.span_end()?;
            let initializer =
                ConstArrayInitializer::new(initializer_exps, (initializer_start, initializer_end));
            self.consume(TokenKind::BracketClose)?;

            let decl_end = self.span_end()?;

            return Ok(Statement::ConstArrayDeclaration(
                ConstArrayDeclaration::new(
                    typ,
                    name,
                    array_size,
                    initializer,
                    (decl_begin, decl_end),
                ),
            ));
        }

        return Err(ParsingError::from_token(
            PEK::ExpectedOneOfToken(vec![TokenKind::SquareOpen, TokenKind::Assign]),
            self.current_id(),
            true,
        ));
    }

    pub fn const_decl_decl(&mut self) -> Result<Declaration> {
        let decl = self.const_decl_stmt()?;

        Ok(match decl {
            Statement::ConstDeclaration(d) => Declaration::Const(d),
            Statement::ConstArrayDeclaration(d) => Declaration::ConstArray(d),
            _ => return Err(ParsingError::internal_error()),
        })
    }

    pub fn class_decl(&mut self) -> Result<Class> {
        self.consume(TokenKind::Class)?;

        let name = self.ident()?;
        let mut decls = Vec::new();

        self.consume(TokenKind::BracketOpen)?;

        while !self.check(TokenKind::BracketClose) {
            decls.extend(self.var_decl()?);
            self.consume(TokenKind::Semicolon)?;
        }

        self.consume(TokenKind::BracketClose)?;

        Ok(Class {
            name,
            members: decls,
        })
    }

    pub fn global_declaration(&mut self) -> Result<Declaration> {
        let decl = if self.check(TokenKind::Func) {
            Declaration::Func(self.func_decl()?)
        } else if self.check(TokenKind::Var) {
            Declaration::Var(self.var_decl()?)
        } else if self.check(TokenKind::Const) {
            self.const_decl_decl()?
        } else if self.check(TokenKind::Instance) {
            Declaration::Inst(vec![self.instance_decl()?])
        } else if self.check(TokenKind::Prototype) {
            Declaration::Proto(self.prototype_decl()?)
        } else if self.check(TokenKind::Class) {
            Declaration::Class(self.class_decl()?)
        } else {
            return Err(ParsingError::from_token(
                PEK::ExpectedOneOfToken(vec![
                    TokenKind::Func,
                    TokenKind::Var,
                    TokenKind::Const,
                    TokenKind::Prototype,
                    TokenKind::Class,
                ]),
                self.current_id(),
                false,
            ));
        };

        Ok(decl)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::parser::Parser;
    use crate::types::{
        Expression, FloatNode, Identifier, IntNode, Statement, StringLiteral, UnaryExpression,
    };

    #[test]
    fn int_foo_decl() {
        let lexed = Lexer::lex(b"var int foo;");
        let expected = vec![VarDeclaration::new(
            Identifier::new(b"int", (4, 7)),
            Identifier::new(b"foo", (8, 11)),
            None,
            (0, 12),
        )];

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.var_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn single_int_foo_decl() {
        let lexed = Lexer::lex(b"var int foo");
        let expected = VarDeclaration::new(
            Identifier::new(b"int", (4, 7)),
            Identifier::new(b"foo", (8, 11)),
            None,
            (0, 11),
        );

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.single_var_decl().unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn int_array_size() {
        let lexed = Lexer::lex(b"[13]");
        let expected = ArraySizeDeclaration::Size(13);

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.array_size_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn identifier_array_size() {
        let lexed = Lexer::lex(b"[MAX]");
        let expected = ArraySizeDeclaration::Identifier(Identifier::new(b"MAX", (1, 4)));

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.array_size_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn decl_int_foo() {
        let init = Expression::Int(IntNode {
            value: 5,
            span: (16, 17),
        });
        let decl = ConstDeclaration::new(
            Identifier::new(b"int", (6, 9)),
            Identifier::new(b"foo", (10, 13)),
            init,
            (0, 17),
        );
        let lexed = Lexer::lex(b"const int foo = 5").unwrap();
        let mut parser = Parser::new(&lexed);
        let actual = parser.const_decl_stmt().unwrap();

        assert_eq!(Statement::ConstDeclaration(decl), actual);
    }

    #[test]
    pub fn decl_uppercase_const_int_foo() {
        let init = Expression::Int(IntNode {
            value: 14,
            span: (15, 17),
        });
        let decl = ConstDeclaration::new(
            Identifier::new(b"int", (6, 9)),
            Identifier::new(b"foo", (10, 13)),
            init,
            (0, 17),
        );
        let lexed = Lexer::lex(b"CONST int foo= 14").unwrap();
        let mut parser = Parser::new(&lexed);
        let actual = parser.const_decl_stmt().unwrap();

        assert_eq!(Statement::ConstDeclaration(decl), actual);
    }

    #[test]
    pub fn decl_zcvob_foo_unary() {
        let init = Expression::Unary(Box::new(UnaryExpression::new(
            b'!',
            Expression::Int(IntNode {
                value: 5,
                span: (19, 20),
            }),
            (18, 20),
        )));
        let decl = ConstDeclaration::new(
            Identifier::new(b"zCVob", (6, 11)),
            Identifier::new(b"foo", (12, 15)),
            init,
            (0, 20),
        );
        let lexed = Lexer::lex(b"CONST zCVob foo = !5").unwrap();
        let mut parser = Parser::new(&lexed);
        let actual = parser.const_decl_stmt().unwrap();
        assert_eq!(Statement::ConstDeclaration(decl), actual);
    }

    #[test]
    pub fn decl_int_index() {
        let init = vec![
            Expression::Int(IntNode {
                value: 5,
                span: (23, 24),
            }),
            Expression::Int(IntNode {
                value: 6,
                span: (25, 26),
            }),
            Expression::Unary(Box::new(UnaryExpression::new(
                b'+',
                Expression::Int(IntNode {
                    value: 12,
                    span: (28, 30),
                }),
                (27, 30),
            ))),
        ];
        let array_size = ArraySizeDeclaration::Size(3);
        let decl = ConstArrayDeclaration::new(
            Identifier::new(b"int", (6, 9)),
            Identifier::new(b"foo", (10, 13)),
            array_size,
            ConstArrayInitializer::new(init, (23, 31)),
            (0, 31),
        );

        let lexed = Lexer::lex(b"const int foo [ 3 ] = {5,6,+12}").unwrap();
        let mut parser = Parser::new(&lexed);
        let actual = parser.const_decl_stmt().unwrap();
        assert_eq!(Statement::ConstArrayDeclaration(decl), actual);
    }

    #[test]
    pub fn decl_identifier_index() {
        let init = vec![
            Expression::Int(IntNode {
                value: 5,
                span: (30, 31),
            }),
            Expression::Int(IntNode {
                value: 6,
                span: (33, 34),
            }),
            Expression::Unary(Box::new(UnaryExpression::new(
                b'+',
                Expression::Int(IntNode {
                    value: 12,
                    span: (37, 39),
                }),
                (36, 39),
            ))),
        ];
        let array_size = ArraySizeDeclaration::Identifier(Identifier::new(b"MAX_SIZE", (16, 24)));
        let decl = ConstArrayDeclaration::new(
            Identifier::new(b"int", (6, 9)),
            Identifier::new(b"foo", (10, 13)),
            array_size,
            ConstArrayInitializer::new(init, (30, 40)),
            (0, 40),
        );

        let lexed = Lexer::lex(b"const int foo [ MAX_SIZE ] = {5, 6, +12}").unwrap();
        let mut parser = Parser::new(&lexed);
        let actual = parser.const_decl_stmt().unwrap();
        assert_eq!(Statement::ConstArrayDeclaration(decl), actual);
    }

    #[test]
    pub fn decl_string_initializer() {
        let init = vec![
            Expression::String(StringLiteral::new(b"hello", (29, 36))),
            Expression::Float(FloatNode {
                value: 6.0,
                span: (38, 41),
            }),
            Expression::Unary(Box::new(UnaryExpression::new(
                b'+',
                Expression::Int(IntNode {
                    value: 12,
                    span: (44, 46),
                }),
                (43, 46),
            ))),
        ];
        let array_size = ArraySizeDeclaration::Identifier(Identifier::new(b"MAX_SIZE", (15, 23)));
        let decl = ConstArrayDeclaration::new(
            Identifier::new(b"int", (6, 9)),
            Identifier::new(b"foo", (10, 13)),
            array_size,
            ConstArrayInitializer::new(init, (29, 47)),
            (0, 47),
        );

        let lexed = Lexer::lex(b"const int foo[ MAX_SIZE ] = {\"hello\", 6.0, +12}").unwrap();
        let mut parser = Parser::new(&lexed);
        let actual = parser.const_decl_stmt().unwrap();
        assert_eq!(Statement::ConstArrayDeclaration(decl), actual);
    }

    #[test]
    fn simple_class() {
        let lexed = Lexer::lex(b"class foo {var int bar;}").unwrap();
        let expected = Class {
            name: Identifier::new(b"foo", (6, 9)),
            members: vec![VarDeclaration::new(
                Identifier::new(b"int", (15, 18)),
                Identifier::new(b"bar", (19, 22)),
                None,
                (11, 23),
            )],
        };

        let mut parser = Parser::new(&lexed);
        let actual = parser.class_decl().unwrap();
        assert_eq!(expected, actual);
    }
    #[test]
    fn empty_class() {
        let lexed = Lexer::lex(b"class foo {}").unwrap();
        let expected = Class {
            name: Identifier::new(b"foo", (6, 9)),
            members: Vec::new(),
        };

        let mut parser = Parser::new(&lexed);
        let actual = parser.class_decl().unwrap();
        assert_eq!(expected, actual);
    }

    // #[test]
    // fn error_in_class() {
    //     let lexed = Lexer::lex(b"class foo { foo foo foo }");
    //     let expected = ParsingError::from_span(PEK::ExpectedToken(TokenKind::Var), (12,15), true);
    //     let mut parser = Parser::new(&lexed.unwrap());
    //     let actual = parser.class_decl().unwrap_err();

    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn incomplete_var_decl_in_class() {
    //     let lexed = Lexer::lex(b"class foo { var foo }");
    //     let expected = ParsingError::from_span(PEK::ExpectedToken(TokenKind::Identifier(vec![])), (20,21), true);
    //     let mut parser = Parser::new(&lexed.unwrap());
    //     let actual = parser.class_decl().unwrap_err();

    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn incomplete_func_decl() {
    //     let lexed = Lexer::lex(b"func void () {}");
    //     let expected = ParsingError::from_span(PEK::ExpectedToken(TokenKind::Identifier(vec![])), (10,11), true);
    //     let mut parser = Parser::new(&lexed.unwrap());
    //     let actual = parser.func_decl().unwrap_err();

    //     assert_eq!(expected, actual);
    // }
}
