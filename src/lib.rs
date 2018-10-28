#![feature(slice_patterns)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate encoding;
extern crate glob;

pub mod error_handler;
pub mod errors;
pub mod inner_errors;
pub mod parsers;
pub mod ppa;
pub mod src_parser;
mod tests;
pub mod types;

fn test() -> &'static str {
    "hello"
}
