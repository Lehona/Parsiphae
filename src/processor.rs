use crate::ppa::symbol_collector::SymbolCollector;
use crate::types::SymbolCollection;
use crate::{errors::*, ppa, src_parser, types};
use std::io::Read;
use std::path::{Path, PathBuf};

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
                    eprintln!("Error in file {:?} in line {}: {}", self.file, line, msg);
                }
                _ => unreachable!(),
            },
        }
    }

    pub fn is_ok(&self) -> bool {
        self.result.is_ok()
    }
}

// TODO: Figure out new error handling!
fn process_file<P: AsRef<Path>>(path: P) -> Result<ParsingResult> {
    let mut file = ::std::fs::File::open(&path).unwrap();

    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let tokens = crate::lexer::lex(&content).expect("Unable to tokenize");
    let mut parser = crate::parser::parser::Parser::new(tokens);

    let result = parser
        .start()
        .map(|declarations| types::AST { declarations })
        .map_err(|e| Error::ParsingError {
            err: crate::inner_errors::ParserError::FromNom,
            line: crate::error_handler::get_line_number(&content, e.span.0),
        });

    Ok(ParsingResult::new(path, result))
}

pub fn process_single_file<P: AsRef<Path>>(path: P) -> Result<types::AST> {
    let res = process_file(path)?;

    let mut visitor = SymbolCollector::new();
    {
        if let Ok(ref ast) = res.result {
            crate::ppa::visitor::visit_ast(&ast, &mut visitor);
        }

        let symbols = SymbolCollection::new(visitor.syms);
        let mut typechk = ppa::typecheck::TypeChecker::new(&symbols);
        typechk.typecheck();
    }

    return res.result;
}

pub fn process_src<P: AsRef<Path>>(path: P) -> Result<()> {
    let d_paths = src_parser::parse_src(&path)?;

    let results: Vec<ParsingResult> = d_paths.iter().map(process_file).collect::<Result<_>>()?;

    let mut visitor = SymbolCollector::new();

    {
        let okay_results = results.iter().filter_map(|res| res.result.as_ref().ok());

        for ast in okay_results {
            crate::ppa::visitor::visit_ast(&ast, &mut visitor);
        }
    }

    println!("Parsed {} files", results.len());
    if results.iter().all(ParsingResult::is_ok) {
        println!("No syntax errors detected!");
        return Ok(());
    } else {
        let mut err = Ok(());
        for result in results {
            result.print();
            if let Err(e) = result.result {
                err = Err(e);
            }
        }
        return err;
    }
}
