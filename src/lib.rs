#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate encoding;
extern crate glob;
#[macro_use]
extern crate derive_more;

pub mod error_handler;
pub mod errors;
pub mod handwritten_parsers;
pub mod inner_errors;
pub mod lexer;
pub mod parsers;
pub mod ppa;
pub mod src_parser;
mod tests;
pub mod types;

fn test() -> &'static str {
    "hello"
}
