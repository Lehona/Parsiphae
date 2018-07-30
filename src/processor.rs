use errors::*;
use src_parser;
use std::io::Read;
use std::path::{Path, PathBuf};
use types;

#[derive(Debug)]
pub struct ParsingResult {
    file: PathBuf,
    result: Result<types::AST>,
}

impl ParsingResult {
    pub fn new<P: AsRef<Path>>(path: P, result: Result<types::AST>) -> Self {
        ParsingResult {
            file: path.as_ref().to_owned(),
            result,
        }
    }

    pub fn print(&self) {
        match self.result {
            Ok(_) => {}
            Err(ref e) => match e {
                Error::ParsingError { err, line } => {
                    let msg = err.description();
                    println!("Error in file {:?} in line {}: {}", self.file, line, msg);
                }
                _ => unreachable!(),
            },
        }
    }

    pub fn is_ok(&self) -> bool {
        return match self.result {
            Ok(_) => true,
            Err(_) => false,
        };
    }
}

fn process_file<P: AsRef<Path>>(path: P) -> Result<ParsingResult> {
    let mut file = ::std::fs::File::open(&path).unwrap();

    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    use parsers::*;
    let result = start(types::Input(&content))
        .map_err(|err| ::error_handler::map_err(&content, err))
        .map(|tuple| tuple.1);

    Ok(ParsingResult::new(path, result))
}

pub fn process_single_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let res = process_file(path)?;

    res.print();

    Ok(())
}

pub fn process_src<P: AsRef<Path>>(path: P) -> Result<()> {
    let d_paths = src_parser::parse_src(&path)?;

    let results: Vec<_> = d_paths.iter().map(process_file).collect::<Result<_>>()?;

    println!("Parsed {} files", results.len());
    if results.iter().all(ParsingResult::is_ok) {
        println!("No syntax errors detected!");
    } else {
        for result in results {
            result.print();
        }
    }
    Ok(())
}
