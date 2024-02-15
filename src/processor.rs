use crate::config::{Config, InputFile};
use crate::error_handler::process_parsing_result;
use crate::file::{File, FileDb};
use crate::json::MachineReadableOutput;
use crate::lexer::Lexer;
use crate::parser::errors::ParsingError;
use crate::parser::parser::Parser;
use crate::ppa::symbol_collector::SymbolCollector;
use crate::src_parser::SrcParser;
use crate::types::SymbolCollection;
use crate::{errors::*, ppa, types};
use itertools::Either;
use itertools::Itertools;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub struct ParsingResult {
    pub file_id: usize,
    pub result: Result<types::AST>,
}

impl ParsingResult {
    pub fn new(file_id: usize, result: Result<types::AST>) -> Self {
        ParsingResult { file_id, result }
    }

    pub fn print(&self) {
        match self.result {
            Ok(_) => {}
            Err(ref e) => match e {
                Error::ParsingError(_err) => {
                    // let msg = err.description();
                    eprintln!(
                        "Error in file {:?} in line {}: {}",
                        self.file_id, 1337, "kapuuut"
                    ); // TODO: fix
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

pub struct Parsiphae;

impl Parsiphae {
    /// This is essentially the entry point to Parsiphae. You pass
    /// in a configuration, and Parsiphae will start the parsing process.
    /// Currently the only output are log lines.
    ///
    /// # Parameters
    /// `config`: Parsiphae configuration
    ///
    /// # Returns
    /// Ok() if processing was succesful.
    pub fn process(config: Config) -> Result<()> {
        let files = match config.input_file {
            InputFile::SingleFile(path) => vec![path],
            InputFile::Src(path) => SrcParser::parse_src(&path)?,
        };

        let mut file_db = FileDb::new();
        let mut visitor = SymbolCollector::new();

        let (successful_parses, errors): (Vec<types::AST>, Vec<(usize, ParsingError)>) = files
            .into_iter()
            .map(|file| Self::process_file(&mut file_db, file))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .partition_map(|r| match r.result {
                Ok(ast) => Either::Left(ast),
                Err(e) => Either::Right((r.file_id, e.as_parsing_error())),
            });

        for ast in &successful_parses {
            crate::ppa::visitor::visit_ast(&ast, &mut visitor);
        }

        let symbols = SymbolCollection::new(visitor.syms);
        let mut typechk = ppa::typecheck::TypeChecker::new(&symbols);
        let _tc_result = typechk.typecheck();
        let amount_errors = errors.len();

        for (file_id, error) in &errors {
            process_parsing_result(&file_db, *file_id, error);
        }

        if config.json {
            MachineReadableOutput::process(&file_db, errors).expect("Failed...");
        }

        log::info!("Parsed {} files.", successful_parses.len() + amount_errors);
        if amount_errors == 0 {
            log::info!("No syntax errors detected.");
        }

        Ok(())
    }

    // TODO: Figure out new error handling!
    fn process_file<P: AsRef<Path>>(file_db: &mut FileDb, path: P) -> Result<ParsingResult> {
        let mut file = std::fs::File::open(&path).unwrap();

        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        let tokens = Lexer::lex(&content).expect("Unable to tokenize");
        let mut parser = Parser::new(&tokens);

        let file_obj = File::new(path.as_ref().to_owned(), content, Some(tokens));
        let file_id = file_db.add(file_obj);

        let result = parser
            .start()
            .map(|declarations| types::AST { declarations })
            .map_err(|e| e.with_token_start(parser.progress() + 1).into());

        Ok(ParsingResult::new(file_id, result))
    }
}
