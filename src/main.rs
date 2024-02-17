#![allow(dead_code)]

use clap::{App, Arg};
use env_logger::Env;
use time::PreciseTime;

use anyhow::{anyhow, Result};
use parsiphae::{
    config::{Config, InputFile},
    processor::Parsiphae,
};

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

    let start_time = PreciseTime::now();

    let mut exitcode = 0;
    if let Err(ref e) = run() {
        match e {
            _ => {
                exitcode = 2;
            } // _ => {
              //     use std::io::Write;
              //     let stderr = &mut std::io::stderr();
              //     let errmsg = "Error writing to stderr";

              //     writeln!(stderr, "error: {:#?}", e).expect(errmsg);

              //     std::process::exit(1);
              // }
        }
    }

    let ms = start_time.to(PreciseTime::now()).num_milliseconds() as f64;
    log::info!("parsing took {} seconds", ms / 1000.0);

    std::process::exit(exitcode);
}

fn run() -> Result<()> {
    let config = make_config()?;
    let mut parsiphae = Parsiphae::new(config);
    if let Err(e) = parsiphae.process() {
        e.render(&parsiphae.config, &parsiphae.file_db);
        Err(anyhow!("Pipeline failed"))
    } else {
        Ok(())
    }
}

fn make_config() -> Result<Config> {
    let arguments = App::new("Parsiphae")
        .version("0.2")
        .author("Leon von Mulert <leonvonmulert@gmail.com")
        .about("An experimental Daedalus parser")
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
            Arg::with_name("JSON")
                .help("Whether output should be formatted as JSON")
                .long("json"),
        )
        .get_matches();

    let input_file = match (arguments.value_of("INPUT"), arguments.value_of("SRC")) {
        (Some(_), Some(_)) => {
            return Err(anyhow!(
                "You specified both a single file and an SRC to parse, please choose one.\n{}",
                arguments.usage()
            ))
        }
        (Some(input), _) => InputFile::SingleFile(input.into()),
        (_, Some(src)) => InputFile::Src(src.into()),
        (_, _) => {
            return Err(anyhow!(
                "You specified neither a single file nor an SRC to parse, please choose one.\n{}",
                arguments.usage()
            ))
        }
    };
    let json = arguments.is_present("JSON");
    Ok(Config { input_file, json })
}
