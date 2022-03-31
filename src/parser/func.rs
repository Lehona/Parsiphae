use crate::lexer::TokenKind;
use crate::types::{Function, Instance, Prototype};

use crate::parser::errors::Result;
impl crate::parser::parser::Parser {
    pub fn func_decl(&mut self) -> Result<Function> {
        self.consume(TokenKind::Func)?;

        let typ = self.ident()?;
        let name = self.ident()?;
        let mut params = Vec::new();

        self.consume(TokenKind::ParenOpen)?;

        while self.check(TokenKind::Var) {
            params.push(self.single_var_decl()?);

            if !match_tok!(self, TokenKind::Comma) {
                break;
            }
        }

        self.consume(TokenKind::ParenClose)?;

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

        let name = self.ident()?;
        self.consume(TokenKind::ParenOpen)?;
        let class = self.ident()?;
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
    use crate::parser::parser::Parser;
    use crate::lexer::lex;
    use crate::types::{
        Assignment, AssignmentOperator, Expression, Identifier, Statement, VarAccess,
        VarDeclaration,
    };

    #[test]
    fn empty_void() {
        let lexed = lex(b"func void foo() {}").unwrap();
        let expected = Function {
            typ: Identifier::new(b"void"),
            name: Identifier::new(b"foo"),
            params: Vec::new(),
            body: Vec::new(),
        };

        let mut parser = Parser::new(lexed);
        let actual = parser.func_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn params_empty_body() {
        let lexed = lex(b"func void foo(var int bar) {}").unwrap();
        let expected = Function {
            typ: Identifier::new(b"void"),
            name: Identifier::new(b"foo"),
            params: vec![VarDeclaration::new(
                Identifier::new(b"int"),
                Identifier::new(b"bar"),
                None,
            )],
            body: Vec::new(),
        };

        let mut parser = Parser::new(lexed);
        let actual = parser.func_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_params_empty_body() {
        let lexed = lex(b"func void foo(var int bar, var int baz) {}").unwrap();
        let expected = Function {
            typ: Identifier::new(b"void"),
            name: Identifier::new(b"foo"),
            params: vec![
                VarDeclaration::new(Identifier::new(b"int"), Identifier::new(b"bar"), None),
                VarDeclaration::new(Identifier::new(b"int"), Identifier::new(b"baz"), None),
            ],
            body: Vec::new(),
        };

        let mut parser = Parser::new(lexed);
        let actual = parser.func_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn body() {
        let lexed = lex(b"func void foo() {3;}").unwrap();
        let expected = Function {
            typ: Identifier::new(b"void"),
            name: Identifier::new(b"foo"),
            params: Vec::new(),
            body: vec![Statement::Exp(Expression::Int(3))],
        };

        let mut parser = Parser::new(lexed);
        let actual = parser.func_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn simple_proto() {
        let lexed = lex(b"prototype foo (bar) {}");
        let expected = Prototype {
            name: Identifier::new(b"foo"),
            class: Identifier::new(b"bar"),
            body: Vec::new(),
        };

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.prototype_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn init_proto() {
        let lexed = lex(b"prototype foo (bar) {name=3;}");
        let expected = Prototype {
            name: Identifier::new(b"foo"),
            class: Identifier::new(b"bar"),
            body: vec![Statement::Ass(Assignment {
                var: VarAccess::new(Identifier::new(b"name"), None, None),
                op: AssignmentOperator::Eq,
                exp: Expression::Int(3),
            })],
        };

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.prototype_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn simple_inst() {
        let lexed = lex(b"instance foo (bar) {}");
        let expected = Instance {
            name: Identifier::new(b"foo"),
            class: Identifier::new(b"bar"),
            body: Vec::new(),
        };

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.instance_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn init_inst() {
        let lexed = lex(b"instance foo (bar) {name=3;}");
        let expected = Instance {
            name: Identifier::new(b"foo"),
            class: Identifier::new(b"bar"),
            body: vec![Statement::Ass(Assignment {
                var: VarAccess::new(Identifier::new(b"name"), None, None),
                op: AssignmentOperator::Eq,
                exp: Expression::Int(3),
            })],
        };

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.instance_decl().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn no_init_inst() {
        let lexed = lex(b"instance foo (bar)");
        let expected = Instance {
            name: Identifier::new(b"foo"),
            class: Identifier::new(b"bar"),
            body: Vec::new(),
        };

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.instance_decl().unwrap();

        assert_eq!(expected, actual);
    }
}
