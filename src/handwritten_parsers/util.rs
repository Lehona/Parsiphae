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
        fn $name(&mut $self) -> anyhow::Result<Expression> {
            use std::convert::TryFrom;

            let mut expr = $self.$next()?;

            while match_tok!($self, $first $(, $tail )* ) {
                let op = $self.previous()?.kind;
                let right = $self.$next()?;

                expr = crate::types::Expression::Binary(Box::new(crate::types::BinaryExpression::new(
                                        crate::types::BinaryOperator::try_from(op).map_err(|e|anyhow::anyhow!("Unable to cast Token to BinaryOperator. {}", e))?,
                                        expr,
                                        right,
                                    )));
            }

            return Ok(expr);
        }
    }
}
