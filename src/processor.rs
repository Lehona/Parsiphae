use parsiphae::ppa::symbol_collector::ClassCollector;
use parsiphae::types::Expression;
use parsiphae::{error_handler, errors::*, inner_errors::ParserError, ppa, src_parser, types};
use std::io::Read;
use std::path::{Path, PathBuf};

struct TestVisitor;
impl ppa::visitor::Visitor for TestVisitor {
    fn visit_expression(&mut self, exp: &Expression, scope: Option<&types::Identifier>) {
        let res = exp.evaluate_int();

        if let Ok(val) = res {
            println!(
                "I found an expression that evaluated to {}: {:#?}",
                val, exp
            )
        }
    }

    fn visit_var_decl(&mut self, decl: &types::VarDeclaration, scope: Option<&types::Identifier>) {
        println!(
            "A variable was declared: {} in scope {}",
            &decl.name,
            scope.unwrap_or(&types::Identifier::new(b""))
        );
    }
}

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

    use parsiphae::parsers::*;
    let result = start(types::Input(&content))
        .map_err(|err| error_handler::map_err(&content, err))
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

    let results: Vec<ParsingResult> = d_paths.iter().map(process_file).collect::<Result<_>>()?;

    let mut visitor = ClassCollector::new();

    {
        let okay_results = results
            .iter()
            .filter_map(|res| res.result.as_ref().ok());

        for ast in okay_results {
            ::parsiphae::ppa::visitor::visit_ast(&ast, &mut visitor);
        }

        // println!("{:#?}", visitor);
    }

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
