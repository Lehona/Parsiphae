#![allow(dead_code)]

extern crate clap;
extern crate time;

extern crate parsiphae;

use clap::{App, Arg};
use std::io::Read;
use time::PreciseTime;

use parsiphae::errors;

fn main() {
    let start_time = PreciseTime::now();

    let mut exitcode = 0;
    if let Err(ref e) = run() {
        match e {
            errors::Error::ParsingError { .. } => {
                exitcode = 2;
            }
            _ => {
                use std::io::Write;
                let stderr = &mut ::std::io::stderr();
                let errmsg = "Error writing to stderr";

                writeln!(stderr, "error: {:#?}", e).expect(errmsg);

                ::std::process::exit(1);
            }
        }
    }

    let ms = start_time.to(PreciseTime::now()).num_milliseconds() as f64;
    println!("parsing took {} seconds", ms / 1000.0);

    ::std::process::exit(exitcode);
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
        .arg(
            Arg::with_name("HAND")
                .help("Uses the new hand-written parser")
                .long("hand"),
        )
        .get_matches();

    let d_path = arguments.value_of("INPUT");
    match d_path {
        Some(path) => {
            if arguments.is_present("HAND") {
                let mut file = ::std::fs::File::open(&path).unwrap();

                let mut content = Vec::new();
                file.read_to_end(&mut content)?;

                let tokens = crate::parsiphae::lexer::lex(&content);

                match tokens {
                    Err(e) => println!("Error: {:?}", e),
                    Ok(tokenlist) => {
                        // println!("{}", tokenlist.iter().map(|t|t.stringified()).collect::<Vec<_>>().join("\n"));
                        let mut parser =
                            parsiphae::parser::parser::Parser::new(&tokenlist);

                        let decls = parser.start();

                        if let Err(e) = decls {
                            println!("{:?}", e);
                        } else {
                            println!("Parse successful: {:#?}", decls.unwrap());
                        }
                    }
                }
            } else {
                parsiphae::processor::process_single_file(path)?;
            }
        }
        None => {
            let path = arguments.value_of("SRC").unwrap();
            parsiphae::processor::process_src(path)?;
        }
    }

    Ok(())
}
