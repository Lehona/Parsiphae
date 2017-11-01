extern crate pom;
extern crate itertools;
extern crate regex;
extern crate glob;
extern crate time;
extern crate bitflags;

pub mod walker;
pub mod ast;
pub mod parsers;
mod src_parser;
pub mod ast_converter;
pub mod symbols;

use src_parser::*;

use time::PreciseTime;


fn main() {
    let start_time = PreciseTime::now();

    parse_src("E:\\Gothic 2 LeGo\\_work\\data\\Scripts\\Content\\GothicTest.src");

    let end_time = PreciseTime::now();
    println!("parsing the src took {} seconds", start_time.to(end_time));
}

