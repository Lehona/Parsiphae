use ast::*;
use pom::parser::*;
use ast_converter::*;

pub mod statement_parsers;
pub mod util;
pub mod declaration;
pub mod base;

use parsers::util::*;
use parsers::base::*;


fn call_parser() -> Parser<'static, u8, Call> {
    (identifier()
        - one_of_s(b"(")
        + (call(expression_list)|empty().map(|_|ExpressionList {expressions: Vec::new()}))
        - one_of_s(b")")
    )
        .map(|(id, exps)| Call {func: id, params: exps})
}

fn array_index() -> Parser<'static, u8, ArrayIndex> {
    number_parser().map(|i|ArrayIndex::Number(i))
    | identifier().map(|s|ArrayIndex::Identifier(s))
}

fn variable() -> Parser<'static, u8, VarAccess> {
    let parser = identifier() + (sym(b'.') * identifier()).opt() + (sym_s(b'[') * array_index() - sym_s(b']')).opt();

    parser.map(|((first_string, second_string), index)| make_var_access(first_string, second_string, index))
}

fn unary() -> Parser<'static, u8, Expression> {
    let parser = one_of_s(b"!~-+") + call(unary);

    parser.map(exp_un_op_switch) | value()

}


fn value() -> Parser<'static, u8, Expression> {
    float().map(|f|Expression::Float(f))
        | number_parser().map(|i|Expression::Value(i))
        | call_parser().map(|c|Expression::Call(Box::new(c)))
        | variable().map(|var|Expression::Variable(var))
        | parentheses()
}

fn bit() -> Parser<'static, u8, Expression> {
    let ops =   boundary(seq(b"&")
        |seq(b"|")
        |seq(b"<<")
        |seq(b">>"))
        .map(BinaryOperator::from);

    let exp = unary() + (ops + call(bit)).opt();

    exp.convert(exp_bin_op_switch)
}

fn mul() -> Parser<'static, u8, Expression> {
    let ops =   boundary(seq(b"*")
                |seq(b"/")
                |seq(b"%"))
        .map(BinaryOperator::from);

    let exp = bit() + (ops + call(mul)).opt();

    exp.convert(exp_bin_op_switch)
}

fn add() -> Parser<'static, u8, Expression> {
    let ops =   boundary(seq(b"+")
                |seq(b"-"))
        .map(BinaryOperator::from);
    let exp = mul() + (ops + call(add)).opt();

    exp.convert(exp_bin_op_switch)
}


fn cmp() -> Parser<'static, u8, Expression> {
    let ops =   boundary(seq(b"==")
        |seq(b"!=")
        |seq(b"<=")
        |seq(b">=")
        |seq(b"<")
        |seq(b">"))
        .map(BinaryOperator::from);

    let exp = add() + (ops + call(cmp)).opt();

    exp.convert(exp_bin_op_switch)
}


fn boolean() -> Parser<'static, u8, Expression> {
    let ops =   (seq_s(b"||")
        |seq_s(b"&&"))
        .map(BinaryOperator::from);
    let exp = cmp() + (ops + call(boolean)).opt();

    exp.convert(exp_bin_op_switch)
}

fn parentheses() -> Parser<'static, u8, Expression> {
    one_of_s(b"(") * call(expression_parser) - one_of_s(b")")
}

pub fn expression_parser() ->  Parser<'static, u8, Expression> {
    string_parser().map(|s|Expression::String(s)) | boolean()
}

pub fn expression_list() -> Parser<'static, u8, ExpressionList> {
    list(expression_parser(), one_of_s(b",")).map(|list| ExpressionList {expressions:list})
}



















