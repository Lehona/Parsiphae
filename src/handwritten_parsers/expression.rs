use crate::lexer::TokenKind;
use crate::types::{Expression, Identifier, VarAccess};
use anyhow::{bail, Result};

use std::convert::TryInto;

impl crate::handwritten_parsers::parser::Parser {
    fn unary(&mut self) -> Result<Expression> {
        if match_tok!(
            self,
            TokenKind::Not,
            TokenKind::BitNot,
            TokenKind::Plus,
            TokenKind::Minus
        ) {
            let op = self.previous()?.kind;
            let right = self.unary()?;

            Ok(crate::types::Expression::Unary(Box::new(
                crate::types::UnaryExpression::new_token(op.try_into()?, right)?,
            )))
        } else {
            self.value()
        }
    }

    binary_parser!(
        self,
        bit,
        unary,
        TokenKind::BitNot,
        TokenKind::BitAnd,
        TokenKind::BitOr,
        TokenKind::ShiftLeft,
        TokenKind::ShiftRight
    );
    binary_parser!(
        self,
        mul,
        bit,
        TokenKind::Multiply,
        TokenKind::Divide,
        TokenKind::Modulo
    );
    binary_parser!(self, add, mul, TokenKind::Plus, TokenKind::Minus);
    binary_parser!(
        self,
        cmp,
        add,
        TokenKind::Greater,
        TokenKind::GreaterEquals,
        TokenKind::Lower,
        TokenKind::LowerEquals,
        TokenKind::Equals,
        TokenKind::NotEquals
    );
    binary_parser!(self, boolean, cmp, TokenKind::And, TokenKind::Or);

    pub fn expression_list(&mut self) -> Result<Vec<Expression>> {
        let mut expressions = Vec::new();

        while !self.check(TokenKind::ParenClose) {
            expressions.push(self.expression()?);

            if !match_tok!(self, TokenKind::Comma) {
                break;
            }
        }

        Ok(expressions)
    }

    pub fn call(&mut self) -> Result<Expression> {
        let ident = self.ident()?;

        self.consume(TokenKind::ParenOpen)?;

        let params = self.expression_list()?;
        self.consume(TokenKind::ParenClose)?;

        Ok(Expression::Call(Box::new(crate::types::Call {
            func: ident,
            params,
        })))
    }

    pub fn ident(&mut self) -> Result<Identifier> {
        let id = match self.current_ref()?.kind {
            TokenKind::Identifier(ref ident) => Identifier::new(ident),
            // Daedalus does not reserve keywords, e.g. 'var' is a valid identifier
            TokenKind::Var => Identifier::new(b"var"),
            TokenKind::Return => Identifier::new(b"return"),
            TokenKind::Class => Identifier::new(b"class"),
            TokenKind::Prototype => Identifier::new(b"prototype"),
            TokenKind::Instance => Identifier::new(b"instance"),
            TokenKind::Func => Identifier::new(b"func"),
            _ => bail!("Expected Identifier, found {:?}", self.current_ref()),
        };

        self.advance();
        Ok(id)
    }

    fn parentheses(&mut self) -> Result<Expression> {
        self.consume(TokenKind::ParenOpen)?;
        let expr = self.expression()?;
        self.consume(TokenKind::ParenClose)?;
        Ok(expr)
    }

    fn value(&mut self) -> Result<Expression> {
        let ice = self.freeze();

        match self.call() {
            Ok(exp) => return Ok(exp),
            _ => self.restore(ice),
        }

        match self.var_access() {
            Ok(va) => return Ok(Expression::Identifier(Box::new(va))),
            _ => self.restore(ice),
        }

        match self.current_ref()?.kind {
            TokenKind::Integer(i) => {
                self.advance();
                return Ok(Expression::Int(i));
            }
            TokenKind::Decimal(f) => {
                self.advance();
                return Ok(Expression::Float(f as f32));
            }
            TokenKind::ParenOpen => return self.parentheses(),
            _ => (),
        }

        bail!("Unable to parse value. Expected one of: <Call>, <Variable>, <Integer>, <Decimal>");
    }

    pub fn var_access(&mut self) -> Result<VarAccess> {
        let ident = self.ident()?;

        let (name, instance) = if match_tok!(self, TokenKind::Period) {
            let ident2 = self.ident()?;

            (ident2, Some(ident))
        } else {
            (ident, None)
        };

        let index_exp = if match_tok!(self, TokenKind::SquareOpen) {
            let exp = self.expression()?;
            self.consume(TokenKind::SquareClose)?;
            Some(exp)
        } else {
            None
        };

        let va = crate::types::VarAccess {
            name,
            instance,
            index: index_exp,
        };
        Ok(va)
    }

    pub fn expression(&mut self) -> Result<Expression> {
        if let TokenKind::StringLit(ref s) = self.current_ref()?.kind {
            let exp = Expression::String(crate::types::StringLiteral {
                data: crate::types::PrintableByteVec(s.to_vec()),
            });
            self.advance();
            return Ok(exp);
        }

        let binary = self.boolean();

        match binary {
			Ok(_) => return binary,
			Err(e) => bail!("Unable to parse value. Expected one of: <Call>, <Variable>, <Integer>, <Decimal>, <String>. Context: {}", e),
		}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handwritten_parsers::parser::Parser;
    use crate::lexer::lex;
    use crate::types::{Call, Identifier, StringLiteral, VarAccess};

    macro_rules! test_binary_eval {
        ($name:ident, $input:literal = $output:literal) => {
            #[test]
            fn $name() {
                let lexed = lex($input).unwrap();
                println!("Lexed as: {:#?}", lexed);
                let mut parser = Parser::new(lexed);
                let expr = parser.expression().unwrap();

                println!("Parsed as: {:?}", expr);
                let actual = expr.evaluate_int().unwrap();

                assert_eq!($output, actual);
            }
        };
    }

    test_binary_eval!(addition, b"2+3" = 5);
    test_binary_eval!(multiply, b"2*3" = 6);
    test_binary_eval!(greater, b"2>3" = 0);
    test_binary_eval!(greater2, b"2<3" = 1);
    test_binary_eval!(bitand, b"3 & 1" = 1);
    test_binary_eval!(bitand2, b"5 & 3" = 1);
    test_binary_eval!(bitor, b"2 | 4" = 6);
    test_binary_eval!(paren, b"(2)" = 2);

    test_binary_eval!(left_associative, b"5-2-1" = 2);

    test_binary_eval!(complex0, b"4+8/2*9" = 40);
    test_binary_eval!(complex1, b"7+4/4*2" = 9);
    test_binary_eval!(complex2, b"3+11*4+12/3" = 51);
    test_binary_eval!(complex3, b"12-12+8*9-8" = 64);
    test_binary_eval!(complex4, b"13+15-12*5/4-13" = 0);
    test_binary_eval!(complex5, b"6/3-3+12+10*3" = 41);
    test_binary_eval!(complex6, b"3*5&1" = 3);
    test_binary_eval!(complex7, b"3<<1+7" = 13);
    test_binary_eval!(complex8, b"1||0&&1||0" = 1);
    test_binary_eval!(complex9, b"1&&1||1&&0" = 0);
    test_binary_eval!(complex10, b"7*-3" = -21);
    test_binary_eval!(complex11, b"7*-3+5" = -16);
    test_binary_eval!(complex12, b"7*-(3+5)" = -56);

    test_binary_eval!(repeated_unary, b"!!!!!!!!15" = 1);

    #[test]
    fn no_param() {
        let lexed = lex(b"foo()");
        let expected = Call {
            func: Identifier::new(b"foo"),
            params: Vec::new(),
        };

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.expression().unwrap();

        match actual {
            Expression::Call(call) => assert_eq!(expected, *call),
            _ => {
                println!("Actually got {:?}", actual);
                assert!(false);
            }
        }
    }

    #[test]
    fn single_int_param() {
        let lexed = lex(b"foo(3)");
        let expected = Call {
            func: Identifier::new(b"foo"),
            params: vec![Expression::Int(3)],
        };

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.expression().unwrap();

        match actual {
            Expression::Call(call) => assert_eq!(expected, *call),
            _ => {
                println!("Actually got {:?}", actual);
                assert!(false);
            }
        }
    }

    #[test]
    fn multi_mixed_params() {
        let lexed = lex(b"foo(3, \"hello\")");
        let expected = Call {
            func: Identifier::new(b"foo"),
            params: vec![
                Expression::Int(3),
                Expression::String(StringLiteral::new(b"hello")),
            ],
        };

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.expression().unwrap();

        match actual {
            Expression::Call(call) => assert_eq!(expected, *call),
            _ => {
                println!("Actually got {:?}", actual);
                assert!(false);
            }
        }
    }

    #[test]
    fn simple() {
        let lexed = lex(b"foo");
        let expected = VarAccess::new(Identifier::new(b"foo"), None, None);

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.expression().unwrap();

        match actual {
            Expression::Identifier(va) => assert_eq!(expected, *va),
            _ => {
                println!("Actually got {:?}", actual);
                assert!(false);
            }
        }
    }

    #[test]
    fn instance() {
        let lexed = lex(b"foo.bar");
        let expected = VarAccess::new(Identifier::new(b"foo"), Some(Identifier::new(b"bar")), None);

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.expression().unwrap();

        match actual {
            Expression::Identifier(va) => assert_eq!(expected, *va),
            _ => {
                println!("Actually got {:?}", actual);
                assert!(false);
            }
        }
    }

    #[test]
    fn simple_array_int() {
        let lexed = lex(b"foo[3]");
        let expected = VarAccess::new(Identifier::new(b"foo"), None, Some(Expression::Int(3)));

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.expression().unwrap();

        match actual {
            Expression::Identifier(va) => assert_eq!(expected, *va),
            _ => {
                println!("Actually got {:?}", actual);
                assert!(false);
            }
        }
    }

    #[test]
    fn instance_array_int() {
        let lexed = lex(b"foo.bar[3]");
        let expected = VarAccess::new(
            Identifier::new(b"foo"),
            Some(Identifier::new(b"bar")),
            Some(Expression::Int(3)),
        );

        let mut parser = Parser::new(lexed.unwrap());
        let actual = parser.expression().unwrap();

        match actual {
            Expression::Identifier(va) => assert_eq!(expected, *va),
            _ => {
                println!("Actually got {:?}", actual);
                assert!(false);
            }
        }
    }
}
