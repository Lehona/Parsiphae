#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate encoding;
extern crate glob;
#[macro_use]
extern crate derive_more;

pub mod config;
pub mod dat;
pub mod diagnostics;
pub mod errors;
pub mod file;
pub mod json;
pub mod lexer;
pub mod parser;
pub mod ppa;
pub mod processor;
pub mod src_parser;
pub mod types;
