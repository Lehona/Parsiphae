use crate::lexer::TokenKind;
use crate::types::{Function, Instance, Prototype};

use crate::parser::errors::Result;

use super::errors::{ParsingError, ParsingErrorKind};
impl crate::parser::parser::Parser {
    pub fn func_decl(&mut self) -> Result<Function> {
        self.consume(TokenKind::Func)?;

        let typ = match self.ident() {
            Ok(typ) => typ,
            Err(_e) => {
                return Err(ParsingError::from_token(
                    ParsingErrorKind::MissingFunctionType,
                    self.current_id(),
                    false,
                ))
            }
        };

        let name = match self.ident() {
            Ok(name) => name,
            Err(_e) => {
                return Err(ParsingError::from_token(
                    ParsingErrorKind::MissingFunctionName,
                    self.current_id(),
                    false,
                ))
            }
        };

        let mut params = Vec::new();

        self.consume(TokenKind::ParenOpen)?;

        while self.check(TokenKind::Var) {
            params.push(self.single_var_decl()?);

            if !match_tok!(self, TokenKind::Comma) {
                break;
            }
        }

        self.consume(TokenKind::ParenClose)?;

        self.save_progress();

        let body = self.block()?;

        Ok(Function {
            name,
            typ,
            params,
            body,
        })
    }

    pub fn instance_decl(&mut self) -> Result<Instance> {
        self.consume(TokenKind::Instance)?;

        let name = match self.ident() {
            Ok(name) => name,
            Err(_e) => {
                return Err(ParsingError::from_token(
                    ParsingErrorKind::MissingInstanceName,
                    self.current_id(),
                    false,
                ))
            }
        };
        self.consume(TokenKind::ParenOpen)?;
        let class = match self.ident() {
            Ok(class) => class,
            Err(_e) => {
                return Err(ParsingError::from_token(
                    ParsingErrorKind::MissingInstanceType,
                    self.current_id(),
                    false,
                ))
            }
        };
        self.consume(TokenKind::ParenClose)?;

        let body = if self.check(TokenKind::BracketOpen) {
            self.block()?
        } else {
            Vec::new()
        };

        Ok(Instance { name, class, body })
    }

    pub fn prototype_decl(&mut self) -> Result<Prototype> {
        self.consume(TokenKind::Prototype)?;

        let name = self.ident()?;
        self.consume(TokenKind::ParenOpen)?;
        let class = self.ident()?;
        self.consume(TokenKind::ParenClose)?;
        let body = self.block()?;

        Ok(Prototype { name, class, body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::parser::parser::Parser;
    use crate::types::{
        Assignment, AssignmentOperator, Expression, Identifier, Statement, VarAccess,
        VarDeclaration, IntNode,
    };

    #[test]
    fn empty_void() {
        let lexed = lex(b"func void foo() {}").unwrap();
        let expected = Function {
            typ: Identifier::new(b"void", (5, 9)),
            name: Identifier::new(b"foo", (10, 13)),
            params: Vec::new(),
            body: Vec::new(),
        };

        let mut parser = Parser::new(&lexed);
        let actual = parser.func_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn params_empty_body() {
        let lexed = lex(b"func void foo(var int bar) {}").unwrap();
        let expected = Function {
            typ: Identifier::new(b"void", (5, 9)),
            name: Identifier::new(b"foo", (10, 13)),
            params: vec![VarDeclaration::new(
                Identifier::new(b"int", (18, 21)),
                Identifier::new(b"bar", (22, 25)),
                None,
                (14, 26),
            )],
            body: Vec::new(),
        };

        let mut parser = Parser::new(&lexed);
        let actual = parser.func_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_params_empty_body() {
        let lexed = lex(b"func void foo(var int bar, var int baz) {}").unwrap();
        let expected = Function {
            typ: Identifier::new(b"void", (5, 9)),
            name: Identifier::new(b"foo", (10, 13)),
            params: vec![
                VarDeclaration::new(
                    Identifier::new(b"int", (18, 21)),
                    Identifier::new(b"bar", (22, 25)),
                    None,
                    (14, 26),
                ),
                VarDeclaration::new(
                    Identifier::new(b"int", (31, 34)),
                    Identifier::new(b"baz", (35, 38)),
                    None,
                    (27, 39)
                ),
            ],
            body: Vec::new(),
        };

        let mut parser = Parser::new(&lexed);
        let actual = parser.func_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn body() {
        let lexed = lex(b"func void foo() {3;}").unwrap();
        let expected = Function {
            typ: Identifier::new(b"void", (5, 9)),
            name: Identifier::new(b"foo", (10, 13)),
            params: Vec::new(),
            body: vec![Statement::Exp(Expression::Int(IntNode { value: 3, span: (17, 18)}))],
        };

        let mut parser = Parser::new(&lexed);
        let actual = parser.func_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn simple_proto() {
        let lexed = lex(b"prototype foo (bar) {}");
        let expected = Prototype {
            name: Identifier::new(b"foo", (10, 13)),
            class: Identifier::new(b"bar", (15, 18)),
            body: Vec::new(),
        };

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.prototype_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn init_proto() {
        let lexed = lex(b"prototype foo (bar) {name=3;}");
        let expected = Prototype {
            name: Identifier::new(b"foo", (10, 13)),
            class: Identifier::new(b"bar", (15, 18)),
            body: vec![Statement::Ass(Assignment {
                var: VarAccess::new(Identifier::new(b"name", (21, 25)), None, None, (21, 25)),
                op: AssignmentOperator::Eq,
                exp: Expression::Int(IntNode { value: 3, span: (26, 27) }),
                span: (21, 27),
            })],
        };

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.prototype_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn simple_inst() {
        let lexed = lex(b"instance foo (bar) {}");
        let expected = Instance {
            name: Identifier::new(b"foo", (9, 12)),
            class: Identifier::new(b"bar", (14, 17)),
            body: Vec::new(),
        };

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.instance_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn init_inst() {
        let lexed = lex(b"instance foo (bar) {name=3;}");
        let expected = Instance {
            name: Identifier::new(b"foo", (9, 12)),
            class: Identifier::new(b"bar", (14, 17)),
            body: vec![Statement::Ass(Assignment {
                var: VarAccess::new(Identifier::new(b"name", (20, 24)), None, None, (20, 24)),
                op: AssignmentOperator::Eq,
                exp: Expression::Int(IntNode { value: 3, span: (25, 26)}),
                span: (20, 26),
            })],
        };

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.instance_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn no_init_inst() {
        let lexed = lex(b"instance foo (bar)");
        let expected = Instance {
            name: Identifier::new(b"foo", (9, 12)),
            class: Identifier::new(b"bar", (14, 17)),
            body: Vec::new(),
        };

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.instance_decl().unwrap();

        assert_eq!(expected, actual);
    }
}
