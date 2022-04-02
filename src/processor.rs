use crate::error_handler::process_parsing_result;
use crate::file::{FileDb, File};
use crate::ppa::symbol_collector::SymbolCollector;
use crate::types::SymbolCollection;
use crate::{errors::*, ppa, src_parser, types};
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub struct ParsingResult {
    pub file_id: usize,
    pub result: Result<types::AST>,
}

impl ParsingResult {
    pub fn new(file_id: usize, result: Result<types::AST>) -> Self {
        ParsingResult {
            file_id,
            result,
        }
    }

    pub fn print(&self) {
        match self.result {
            Ok(_) => {}
            Err(ref e) => match e {
                Error::ParsingError(_err) => {
                    // let msg = err.description();
                    eprintln!("Error in file {:?} in line {}: {}", self.file_id, 1337, "kapuuut"); // TODO: fix
                }
                _ => unreachable!(),
            },
        }
    }

    pub fn is_ok(&self) -> bool {
        self.result.is_ok()
    }
}

pub fn get_line_number(content: &[u8], offset: usize) -> usize {
    content[0..offset].iter().filter(|b| **b == b'\n').count() + 1
}


// TODO: Figure out new error handling!
fn process_file<P: AsRef<Path>>(file_db: &mut FileDb, path: P) -> Result<ParsingResult> {
    let mut file = ::std::fs::File::open(&path).unwrap();

    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let tokens = crate::lexer::lex(&content).expect("Unable to tokenize");
    let mut parser = crate::parser::parser::Parser::new(&tokens);

    let file_obj = File::new(path.as_ref().to_owned(), content, Some(tokens));
    let file_id = file_db.add(file_obj);

    let result = parser
        .start()
        .map(|declarations| types::AST { declarations })
        .map_err(|mut e| { e.token_start = parser.progress() + 1; e })
        .map_err(|e|e.into());

    Ok(ParsingResult::new(file_id, result))
}

pub fn process_single_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut file_db = FileDb::new();
    let res = process_file(&mut file_db, path)?;

    let mut visitor = SymbolCollector::new();
    {
        if let Ok(ref ast) = res.result {
            crate::ppa::visitor::visit_ast(&ast, &mut visitor);
        } else {
            process_parsing_result(&file_db, res)
        }

        let symbols = SymbolCollection::new(visitor.syms);
        let mut typechk = ppa::typecheck::TypeChecker::new(&symbols);
        typechk.typecheck();
    }
    Ok(())
}

pub fn process_src<P: AsRef<Path>>(path: P) -> Result<()> {
    let d_paths = src_parser::parse_src(&path)?;

    let mut file_db = FileDb::new();
    let results: Vec<ParsingResult> = d_paths.iter().map(|p|process_file(&mut file_db, p)).collect::<Result<_>>()?;

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
