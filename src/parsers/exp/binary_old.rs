use crate::parsers::Unary;
use crate::types::{BinaryExpression, BinaryOperator, Expression, Input};
use crate::types::PResult;

named!(pub Bit<Input, Expression>, gws!(alt!(
    do_parse!(
        left: Unary >>
        op: alt!(
              tag!("|")
            | tag!("&")
            | tag!(">>")
            | tag!("<<")
        ) >>
        right: Bit >>
        (Expression::Binary(Box::new(
            BinaryExpression::new(BinaryOperator::from(&op.0), left, right)
        )))
    )
    | Unary
)));

named!(pub Mul<Input, Expression>, gws!(alt!(
    do_parse!(
        left: Bit >>
        op: alt!(
              tag!("*")
            | tag!("/")
            | tag!("%")
        ) >>
        right: Mul >>
        (Expression::Binary(Box::new(
            BinaryExpression::new(BinaryOperator::from(&op.0), left, right)
        )))
    )
    | Bit
)));

named!(pub Add<Input, Expression>, gws!(alt!(
    do_parse!(
        left: Mul >>
        op: alt!(
              tag!("+")
            | tag!("-")
        ) >>
        right: Add >>
        (Expression::Binary(Box::new(
            BinaryExpression::new(BinaryOperator::from(&op.0), left, right)
        )))
    )
    | Mul
)));

named!(pub Cmp<Input, Expression>, gws!(alt!(
    do_parse!(
        left: Add >>
        op: alt!(
              tag!("<=")
            | tag!(">=")
            | tag!("!=")
            | tag!("==")
            | tag!(">")
            | tag!("<")
        ) >>
        right: Cmp >>
        (Expression::Binary(Box::new(
            BinaryExpression::new(BinaryOperator::from(&op.0), left, right)
        )))
    )
    | Add
)));

named!(pub Boolean<Input, Expression>, gws!(alt!(
    do_parse!(
        left: Cmp >>
        op: alt!(
              tag!("||")
            | tag!("&&")
        ) >>
        right: Boolean >>
        (Expression::Binary(Box::new(
            BinaryExpression::new(BinaryOperator::from(&op.0), left, right)
        )))
    )
    | Cmp
)));
