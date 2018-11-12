#![allow(non_snake_case)]
#[macro_use]
mod util;
#[macro_use]
mod replacements;
mod base;
mod decl;
mod exp;
mod identifier;
mod statement;

pub use self::base::{float_parser, number_parser, string_parser};
pub use self::decl::{
    array_size_decl, class, const_array_decl, const_decl, declaration, func, instance, prototype,
    var_decl, var_decl_list, var_decl_list_0,
};
pub use self::exp::{call_parser, expression, var_access, Bit, Boolean, Cmp, Mul, Unary, Value};
pub use self::identifier::{identifier_list, identifier_parser};
pub use self::statement::{assignment, if_clause, statement, statement_block};

pub use self::util::whitespace;

use self::replacements::multispace0;
use inner_errors::ParserError;
use nom::ErrorKind;
use types::{Input, AST};
named!(pub start<Input, AST, ParserError>, do_parse!(
    multispace0 >>
    decls: many0!(terminated!(declaration, multispace0)) >>
    return_error!(ErrorKind::Custom(ParserError::Declaration), fix_error!(ParserError, eof!())) >>
    (AST {declarations: decls})
));
