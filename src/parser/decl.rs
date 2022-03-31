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

        while match_tok!(self, TokenKind::Comma) {
            if self.check(TokenKind::Var) {
                decls.push(self.single_var_decl()?);
            } else {
                let next_name = self.ident()?;

                let next_size_decl = optional_match!(self, self.array_size_decl());

                decls.push(VarDeclaration::new(
                    decls[0].typ.clone(),
                    next_name,
                    next_size_decl,
                ));
            }
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
                self.current_ref()?,
            ));
        };

        self.consume(TokenKind::SquareClose)?;

        Ok(arr_size)
    }

    pub fn single_var_decl(&mut self) -> Result<VarDeclaration> {
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

        Ok(VarDeclaration::new(typ.clone(), name, size_decl))
    }

    pub fn const_decl_stmt(&mut self) -> Result<Statement> {
        self.consume(TokenKind::Const)?;

        let typ = self.ident()?;
        let name = self.ident()?;

        if match_tok!(self, TokenKind::Assign) {
            let initializer = self.expression()?;

            return Ok(Statement::ConstDeclaration(ConstDeclaration::new(
                typ,
                name,
                initializer,
            )));
        } else if self.check(TokenKind::SquareOpen) {
            let array_size = self.array_size_decl()?;

            self.consume(TokenKind::Assign)?;

            self.consume(TokenKind::BracketOpen)?;
            let initializer = ConstArrayInitializer::new(self.expression_list()?);
            self.consume(TokenKind::BracketClose)?;

            return Ok(Statement::ConstArrayDeclaration(
                ConstArrayDeclaration::new(typ, name, array_size, initializer),
            ));
        }

        return Err(ParsingError::from_token(
            PEK::ExpectedOneOfToken(vec![TokenKind::SquareOpen, TokenKind::Assign]),
            self.current_ref()?,
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
                self.current_ref()?,
            ));
        };

        Ok(decl)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parser::Parser;
    use crate::lexer::lex;
    use crate::types::{Expression, Identifier, Statement, StringLiteral, UnaryExpression};

    #[test]
    fn int_foo_decl() {
        let lexed = lex(b"var int foo;");
        let expected = vec![VarDeclaration::new(
            Identifier::new(b"int"),
            Identifier::new(b"foo"),
            None,
        )];

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.var_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn single_int_foo_decl() {
        let lexed = lex(b"var int foo");
        let expected = VarDeclaration::new(Identifier::new(b"int"), Identifier::new(b"foo"), None);

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.single_var_decl().unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn int_array_size() {
        let lexed = lex(b"[13]");
        let expected = ArraySizeDeclaration::Size(13);

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.array_size_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn identifier_array_size() {
        let lexed = lex(b"[MAX]");
        let expected = ArraySizeDeclaration::Identifier(Identifier::new(b"MAX"));

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.array_size_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn decl_int_foo() {
        let init = Expression::Int(5);
        let decl = ConstDeclaration::new(Identifier::new(b"int"), Identifier::new(b"foo"), init);
        let lexed = lex(b"const int foo = 5").unwrap();
        let mut parser = Parser::new(lexed);
        let actual = parser.const_decl_stmt().unwrap();

        assert_eq!(Statement::ConstDeclaration(decl), actual);
    }

    #[test]
    pub fn decl_uppercase_const_int_foo() {
        let init = Expression::Int(14);
        let decl = ConstDeclaration::new(Identifier::new(b"int"), Identifier::new(b"foo"), init);
        let lexed = lex(b"CONST int foo= 14").unwrap();
        let mut parser = Parser::new(lexed);
        let actual = parser.const_decl_stmt().unwrap();

        assert_eq!(Statement::ConstDeclaration(decl), actual);
    }

    #[test]
    pub fn decl_zcvob_foo_unary() {
        let init = Expression::Unary(Box::new(UnaryExpression::new(b'!', Expression::Int(5))));
        let decl = ConstDeclaration::new(Identifier::new(b"zCVob"), Identifier::new(b"foo"), init);
        let lexed = lex(b"CONST zCVob foo = !5").unwrap();
        let mut parser = Parser::new(lexed);
        let actual = parser.const_decl_stmt().unwrap();
        assert_eq!(Statement::ConstDeclaration(decl), actual);
    }

    #[test]
    pub fn decl_int_index() {
        let init = vec![
            Expression::Int(5),
            Expression::Int(6),
            Expression::Unary(Box::new(UnaryExpression::new(b'+', Expression::Int(12)))),
        ];
        let array_size = ArraySizeDeclaration::Size(3);
        let decl = ConstArrayDeclaration::new(
            Identifier::new(b"int"),
            Identifier::new(b"foo"),
            array_size,
            ConstArrayInitializer::new(init),
        );

        let lexed = lex(b"const int foo [ 3 ] = {5,6,+12}").unwrap();
        let mut parser = Parser::new(lexed);
        let actual = parser.const_decl_stmt().unwrap();
        assert_eq!(Statement::ConstArrayDeclaration(decl), actual);
    }

    #[test]
    pub fn decl_identifier_index() {
        let init = vec![
            Expression::Int(5),
            Expression::Int(6),
            Expression::Unary(Box::new(UnaryExpression::new(b'+', Expression::Int(12)))),
        ];
        let array_size = ArraySizeDeclaration::Identifier(Identifier::new(b"MAX_SIZE"));
        let decl = ConstArrayDeclaration::new(
            Identifier::new(b"int"),
            Identifier::new(b"foo"),
            array_size,
            ConstArrayInitializer::new(init),
        );

        let lexed = lex(b"const int foo [ MAX_SIZE ] = {5, 6, +12}").unwrap();
        let mut parser = Parser::new(lexed);
        let actual = parser.const_decl_stmt().unwrap();
        assert_eq!(Statement::ConstArrayDeclaration(decl), actual);
    }

    #[test]
    pub fn decl_string_initializer() {
        let init = vec![
            Expression::String(StringLiteral::new(b"hello")),
            Expression::Float(6.0),
            Expression::Unary(Box::new(UnaryExpression::new(b'+', Expression::Int(12)))),
        ];
        let array_size = ArraySizeDeclaration::Identifier(Identifier::new(b"MAX_SIZE"));
        let decl = ConstArrayDeclaration::new(
            Identifier::new(b"int"),
            Identifier::new(b"foo"),
            array_size,
            ConstArrayInitializer::new(init),
        );

        let lexed = lex(b"const int foo[ MAX_SIZE ] = {\"hello\", 6.0, +12}").unwrap();
        let mut parser = Parser::new(lexed);
        let actual = parser.const_decl_stmt().unwrap();
        assert_eq!(Statement::ConstArrayDeclaration(decl), actual);
    }

    #[test]
    fn simple_class() {
        let lexed = lex(b"class foo {var int bar;}").unwrap();
        let expected = Class {
            name: Identifier::new(b"foo"),
            members: vec![VarDeclaration::new(
                Identifier::new(b"int"),
                Identifier::new(b"bar"),
                None,
            )],
        };

        let mut parser = Parser::new(lexed);
        let actual = parser.class_decl().unwrap();
        assert_eq!(expected, actual);
    }
    #[test]
    fn empty_class() {
        let lexed = lex(b"class foo {}").unwrap();
        let expected = Class {
            name: Identifier::new(b"foo"),
            members: Vec::new(),
        };

        let mut parser = Parser::new(lexed);
        let actual = parser.class_decl().unwrap();
        assert_eq!(expected, actual);
    }
}
