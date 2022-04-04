macro_rules! match_tok {
    ($self:ident, $first:pat $(, $tail:pat)*) => {
        {
            let matched = $self.has_more() && match $self.current_ref()?.kind {
                $first => true,
                $($tail => true, )*
                _ => false
            };
            if matched {
                $self.advance();
            }
            matched
        }
    }
}

macro_rules! optional_match {
    ($self:ident, $expr:expr) => {{
        let ice = $self.freeze();
        let res = if let Ok(val) = $expr {
            Some(val)
        } else {
            $self.restore(ice);
            None
        };
        res
    }};
}

macro_rules! binary_parser {
    ($self:ident, $name:ident, $next:ident, $first:pat $(, $tail:pat)*) => {
        fn $name(&mut $self) -> crate::parser::errors::Result<Expression> {
            use std::convert::TryFrom;

            let mut expr = $self.$next()?;

            while match_tok!($self, $first $(, $tail )* ) {
                $self.save_progress();
                let op = $self.previous()?.kind;
                let right = $self.$next().map_err(|mut e| { e.recoverable = false; e })?;
                let span = (expr.get_span().0, right.get_span().1);

                expr = crate::types::Expression::Binary(Box::new(crate::types::BinaryExpression::new(
                                        crate::types::BinaryOperator::try_from(op).map_err(|_| ParsingError::internal_error())?,
                                        expr,
                                        right,
                                        span,
                                    )));
            }

            return Ok(expr);
        }
    }
}
