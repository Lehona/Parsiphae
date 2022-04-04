use crate::lexer::TokenKind;
use crate::parser::errors::{
    ParsePossibility as PP, ParsingError, ParsingErrorKind as PEK, Result,
};
use crate::types::{Expression, Identifier, VarAccess, FloatNode, IntNode};

use std::convert::TryInto;

impl crate::parser::parser::Parser {
    fn unary(&mut self) -> Result<Expression> {
        if match_tok!(
            self,
            TokenKind::Not,
            TokenKind::BitNot,
            TokenKind::Plus,
            TokenKind::Minus
        ) {
            let prev = self.previous()?;
            let span_start = prev.span.0;
            let right = self.unary()?;
            let span_end = right.get_span().1;

            Ok(crate::types::Expression::Unary(Box::new(
                crate::types::UnaryExpression::from_token(prev.kind.try_into().unwrap(), right, (span_start, span_end))?, // TODO: fix unwrap
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

        let span = (ident.span.0, self.previous().unwrap().span.1);
        Ok(Expression::Call(Box::new(crate::types::Call {
            func: ident,
            params,
            span,
        })))
    }

    pub fn ident(&mut self) -> Result<Identifier> {
        let current_tok = self.current_ref()?;
        let id = match current_tok.kind {
            TokenKind::Identifier(ref ident) => Identifier::new(ident, current_tok.span),
            // Daedalus does not reserve keywords, e.g. 'var' is a valid identifier
            TokenKind::Var => Identifier::new(b"var", current_tok.span),
            TokenKind::Return => Identifier::new(b"return", current_tok.span),
            TokenKind::Class => Identifier::new(b"class", current_tok.span),
            TokenKind::Prototype => Identifier::new(b"prototype", current_tok.span),
            TokenKind::Instance => Identifier::new(b"instance", current_tok.span),
            TokenKind::Func => Identifier::new(b"func", current_tok.span),
            _ => {
                return Err(ParsingError::from_token(
                    PEK::ExpectedToken(TokenKind::Identifier(vec![])),
                    self.current_id(),
                    true,
                ))
            }
        };

        self.advance();
        Ok(id)
    }

    fn parentheses(&mut self) -> Result<Expression> {
        self.consume(TokenKind::ParenOpen)?;
        let expr = self.expression();
        if let Err(_e) = expr {
            return Err(ParsingError::from_token(
                PEK::Expected(PP::Expression),
                self.current_id() + 1,
                false,
            ));
        }
        self.consume(TokenKind::ParenClose)?;
        Ok(expr.unwrap())
    }

    fn value(&mut self) -> Result<Expression> {
        let ice = self.freeze();

        // TODO: implement handling of recoverability
        match self.call() {
            Ok(exp) => return Ok(exp),
            Err(e) if !e.recoverable => return Err(e),
            _ => self.restore(ice),
        }

        match self.var_access() {
            Ok(va) => return Ok(Expression::Identifier(Box::new(va))),
            Err(e) if !e.recoverable => return Err(e),
            _ => self.restore(ice),
        }

        let current_span = self.current_ref()?.span;

        match self.current_ref()?.kind {
            TokenKind::Integer(i) => {
                self.advance();
                return Ok(Expression::Int(IntNode { value: i as i64, span: current_span }));
            }
            TokenKind::Decimal(f) => {
                self.advance();
                return Ok(Expression::Float(FloatNode { value: f as f64, span: current_span }));
            }
            TokenKind::ParenOpen => return self.parentheses(),
            _ => (),
        }

        return Err(ParsingError::from_token(
            PEK::ExpectedOneOf(vec![PP::Call, PP::VariableAccess, PP::Integer, PP::Decimal]),
            self.current_id(),
            true,
        ));
    }

    pub fn var_access(&mut self) -> Result<VarAccess> {
        let ident = self.ident()?;
        let span_start = ident.span.0;
        let mut span_end = ident.span.1;

        let (name, instance) = if match_tok!(self, TokenKind::Period) {
            let ident2 = self.ident().map_err(|mut e| {
                e.recoverable = false;
                e
            })?;
            span_end = ident2.span.1;

            (ident2, Some(ident))
        } else {
            (ident, None)
        };

        let index_exp = if match_tok!(self, TokenKind::SquareOpen) {
            let exp = self.expression();
            if let Err(_e) = exp {
                return Err(ParsingError::from_token(
                    PEK::Expected(PP::Expression),
                    self.current_id(),
                    false,
                ));
            }
            self.consume(TokenKind::SquareClose)?;
            span_end = self.previous().unwrap().span.1;
            Some(exp.unwrap())
        } else {
            None
        };

        let va = crate::types::VarAccess {
            name,
            instance,
            index: index_exp,
            span: (span_start, span_end),
        };
        Ok(va)
    }

    pub fn expression(&mut self) -> Result<Expression> {
        let current_tok = self.current_ref()?;
        if let TokenKind::StringLit(ref s) = current_tok.kind {
            let exp = Expression::String(crate::types::StringLiteral {
                data: crate::types::PrintableByteVec(s.to_vec()),
                span: current_tok.span,
            });
            self.advance();
            return Ok(exp);
        }

        self.boolean()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::parser::parser::Parser;
    use crate::types::{Call, Identifier, StringLiteral, VarAccess, IntNode};

    macro_rules! test_binary_eval {
        ($name:ident, $input:literal = $output:literal) => {
            #[test]
            fn $name() {
                let lexed = lex($input).unwrap();
                println!("Lexed as: {:#?}", lexed);
                let mut parser = Parser::new(&lexed);
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
            func: Identifier::new(b"foo", (0, 3)),
            params: Vec::new(),
            span: (0, 5),
        };

        let mut parser = Parser::new(&lexed.unwrap());
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
            func: Identifier::new(b"foo", (0, 3)),
            params: vec![Expression::Int(IntNode { value: 3, span: (4, 5)})],
            span: (0, 6),
        };

        let mut parser = Parser::new(&lexed.unwrap());
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
            func: Identifier::new(b"foo", (0, 3)),
            params: vec![
                Expression::Int(IntNode { value: 3, span: (4, 5)}),
                Expression::String(StringLiteral::new(b"hello", (7, 14))),
            ],
            span: (0, 15),
        };

        let mut parser = Parser::new(&lexed.unwrap());
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
        let expected = VarAccess::new(Identifier::new(b"foo", (0, 3)), None, None, (0, 3));

        let mut parser = Parser::new(&lexed.unwrap());
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
        let expected = VarAccess::new(
            Identifier::new(b"foo", (0, 3)),
            Some(Identifier::new(b"bar", (4, 7))),
            None,
            (0, 7),
        );

        let mut parser = Parser::new(&lexed.unwrap());
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
        let expected = VarAccess::new(
            Identifier::new(b"foo", (0, 3)),
            None,
            Some(Expression::Int(IntNode { value: 3, span: (4, 5)})),
            (0, 6),
        );

        let mut parser = Parser::new(&lexed.unwrap());
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
            Identifier::new(b"foo", (0, 3)),
            Some(Identifier::new(b"bar", (4, 7))),
            Some(Expression::Int(IntNode { value: 3, span: (8, 9)})),
            (0, 10),
        );

        let mut parser = Parser::new(&lexed.unwrap());
        let actual = parser.expression().unwrap();

        match actual {
            Expression::Identifier(va) => assert_eq!(expected, *va),
            _ => {
                println!("Actually got {:?}", actual);
                assert!(false);
            }
        }
    }
    // TODO fix tests
    //     #[test]
    //     fn empty_array_index() {
    //         let lexed = lex(b"foo[]");
    //         let expected = ParsingError::from_span(PEK::Expected(PP::Expression), (4,5), false);
    //         let mut parser = Parser::new(&lexed.unwrap());
    //         let actual = parser.expression().unwrap_err();

    //         assert_eq!(expected, actual);
    //     }

    //     #[test]
    //     fn missing_member() {
    //         let lexed = lex(b"foo.");
    //         let expected = ParsingError::from_span(PEK::ExpectedToken(TokenKind::Identifier(vec![])), (4,4), false);
    //         let mut parser = Parser::new(&lexed.unwrap());
    //         let actual = parser.expression().unwrap_err();

    //         assert_eq!(expected, actual);
    //     }

    //     #[test]
    //     fn missing_paren() {
    //         let lexed = lex(b"3+(4");
    //         let expected = ParsingError::from_span(PEK::ExpectedToken(TokenKind::ParenClose), (4,4), false);
    //         let mut parser = Parser::new(&lexed.unwrap());
    //         let actual = parser.expression().unwrap_err();

    //         assert_eq!(expected, actual);
    //     }

    //     #[test]
    //     fn empty_paren() {
    //         let lexed = lex(b"3+()");
    //         let expected = ParsingError::from_span(PEK::Expected(PP::Expression), (3,4), false);
    //         let mut parser = Parser::new(&lexed.unwrap());
    //         let actual = parser.expression().unwrap_err();

    //         assert_eq!(expected, actual);
    //     }

    //     #[test]
    //     fn double_operator() {
    //         let lexed = lex(b"3**4");
    //         let expected = ParsingError::from_span(PEK::ExpectedOneOf(vec![PP::Call, PP::VariableAccess, PP::Integer, PP::Decimal]), (2,3), false);
    //         let mut parser = Parser::new(&lexed.unwrap());
    //         let actual = parser.expression().unwrap_err();

    //         assert_eq!(expected, actual);
    //     }
}
