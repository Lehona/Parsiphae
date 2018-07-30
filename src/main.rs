#![allow(dead_code)]
#![feature(slice_patterns)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate clap;
extern crate encoding;
extern crate glob;
extern crate time;

mod error_handler;
mod errors;
mod inner_errors;
mod parsers;
mod processor;
mod src_parser;
mod tests;
mod types;

use clap::{App, Arg};
use time::PreciseTime;

fn main() {
    let start_time = PreciseTime::now();

    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {:#?}", e).expect(errmsg);

        ::std::process::exit(1);
    }

    let ms = start_time.to(PreciseTime::now()).num_milliseconds() as f64;
    println!("parsing took {} seconds", ms / 1000.0);
}

fn run() -> errors::Result<()> {
    let arguments = App::new("Parsiphae (nom)")
        .version("0.2")
        .author("Leon von Mulert <leonvonmulert@gmail.com")
        .about("An experimental Daedalus parser using nom")
        .arg(
            Arg::with_name("SRC")
                .help("Sets the input src to use")
                .short("s")
                .long("src")
                .value_name("FILE")
                .required_unless("INPUT"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input .d-file to use")
                .short("i")
                .long("input")
                .value_name("FILE")
                .required_unless("SRC"),
        )
        .get_matches();

    let d_path = arguments.value_of("INPUT");
    match d_path {
        Some(path) => {
            processor::process_single_file(path)?;
        }
        None => {
            let path = arguments.value_of("SRC").unwrap();
            processor::process_src(path)?;
        }
    }

    Ok(())
}

named!(test2<&[u8], &[u8]>, do_parse!(
    things: take!(4) >>
    (things)
));

named!(test<&[u8], &[u8]>, do_parse!(
    stuff: take!(4) >>
    stuff2: flat_map!(stuff, test2) >>
    (stuff2)
));