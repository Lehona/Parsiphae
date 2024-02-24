use crate::config::{Config, InputFile};

use crate::errors::PipelineFailure;
use crate::file::{File, FileDb};

use crate::lexer::Lexer;
use crate::parser::errors::ParsingError;
use crate::parser::parser::Parser;
use crate::ppa::symbol_collector::SymbolCollector;
use crate::ppa::typecheck::TypeChecker;
use crate::src_parser::SrcParser;
use crate::types::{gothic2_externals, SymbolCollection, AST};
use std::io::Read;
use std::path::Path;

type Result<O> = std::result::Result<O, PipelineFailure>;

pub struct Parsiphae {
    pub file_db: FileDb,
    pub config: Config,
    pub symbols: SymbolCollection,
}

impl Parsiphae {
    pub fn new(config: Config) -> Self {
        Parsiphae {
            file_db: FileDb::new(),
            symbols: SymbolCollection::new(),
            config,
        }
    }
    /// This is essentially the entry point to Parsiphae. You pass
    /// in a configuration, and Parsiphae will start the parsing process.
    /// Currently the only output are log lines.
    ///
    /// # Parameters
    /// `config`: Parsiphae configuration
    ///
    /// # Returns
    /// Ok() if processing was succesful.
    pub fn process(&mut self) -> std::result::Result<(), PipelineFailure> {
        // (1) Get input paths/files
        match self.config.input_file.clone() {
            InputFile::SingleFile(path) => self.load_single_file(path)?,
            InputFile::Src(path) => self.load_src(&path)?,
        };

        // (2) Parse files into AST
        let (successful, parsing_errors) = self.parse_files();
        log::info!("Parsed {} files.", successful.len() + parsing_errors.len());

        // (2.1) Report errors
        if !parsing_errors.is_empty() {
            return Err(PipelineFailure::ParsingFailure(parsing_errors));
        }

        log::info!("No syntax errors detected.");

        // (3) Turn AST into symbols
        let mut visitor = SymbolCollector::new();
        visitor.add_externals(&gothic2_externals());
        for (file_id, ast) in &successful {
            visitor.file_id = *file_id;
            crate::ppa::visitor::visit_ast(ast, &mut visitor);
        }

        self.symbols.set_symbols(visitor.syms);

        // (4) Run typechecker
        let mut typechk = TypeChecker::new(&self.symbols);
        let _ = typechk.typecheck();

        // (4.1) Report errors
        if !typechk.errors.is_empty() {
            return Err(PipelineFailure::TypecheckFailure(typechk.errors.clone()));
        }

        log::info!("Typechecking succeeded.");
        Ok(())
    }

    /// Load an SRC file into a FileDB, attempting to parse as many files as possible
    /// and accumulating the errors.
    ///
    /// # Parameters
    /// `src`: Path to the .SRC file
    ///
    /// # Returns
    /// `FileDb` on success, Vector of errors otherwise.
    fn load_src<P: AsRef<Path>>(&mut self, src: P) -> Result<()> {
        let paths = SrcParser::parse_src(src.as_ref())?;

        let mut errors = Vec::new();

        for path in paths {
            let mut file = std::fs::File::open(&path).unwrap();

            let mut content = Vec::new();
            file.read_to_end(&mut content)?;

            let tokens = match Lexer::lex(&content) {
                Ok(tok) => tok,
                Err(e) => {
                    errors.push((path.to_owned(), e));
                    continue;
                }
            };

            let file_obj = File::new(path.to_owned(), content, tokens);
            self.file_db.add(file_obj);
        }

        if !errors.is_empty() {
            return Err(PipelineFailure::LexingFailure(errors));
        }

        Ok(())
    }

    /// Load a single file into a FileDB.
    ///
    /// # Parameters
    /// `path`: Path to the file
    ///
    /// # Returns
    /// `FileDb` on success, Error otherwise.
    fn load_single_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let mut file = std::fs::File::open(&path).unwrap();

        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        let tokens = Lexer::lex(&content)
            .map_err(|e| PipelineFailure::LexingFailure(vec![(path.as_ref().to_owned(), e)]))?;

        let file_obj = File::new(path.as_ref().to_owned(), content, tokens);
        self.file_db.add(file_obj);

        Ok(())
    }

    /// Parse all files in the FileDB.
    fn parse_files(&self) -> (Vec<(usize, AST)>, Vec<(usize, ParsingError)>) {
        let mut successful_parses = Vec::new();
        let mut erroneous_parses = Vec::new();
        for (file_id, file) in self.file_db.iter() {
            log::trace!("Parsing file {}.", file.path.display());
            let mut parser = Parser::new(&file.tokens);
            match parser
                .start()
                .map(|declarations| AST { declarations })
                .map_err(|e| e.with_token_start(parser.progress() + 1))
            {
                Ok(parse) => successful_parses.push((file_id, parse)),
                Err(e) => erroneous_parses.push((file_id, e)),
            }
        }

        (successful_parses, erroneous_parses)
    }
}
