use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    term::{
        emit,
        termcolor::{self, StandardStream},
    },
};

use crate::diagnostics::diagnostics::Diagnostic as DiagnosticP;
use crate::{
    config::Config,
    errors::PipelineFailure,
    file::FileDb,
    json::{JsonError, ParsiphaeJson},
    parser::errors::{ParsingError, ParsingErrorKind as PEK},
    ppa::errors::TypecheckError,
    types::{parsed::Symbol, Identifier},
};

impl PipelineFailure {
    pub fn render(&self, config: &Config, file_db: &FileDb) {
        match config.json {
            true => self.render_json(file_db),
            false => self.render_terminal(file_db),
        }
    }
    fn render_terminal(&self, file_db: &FileDb) {
        match &self {
            PipelineFailure::IOFailure(_) => todo!(),
            PipelineFailure::SrcFailure(_) => todo!(),
            PipelineFailure::LexingFailure(_) => todo!(),
            PipelineFailure::ParsingFailure(errors) => {
                for (file_id, error) in errors {
                    process_parsing_result(file_db, *file_id, error);
                }
            }
            PipelineFailure::TypecheckFailure(errors) => {
                for error in errors {
                    process_typecheck_error(file_db, error);
                }
            }
        }
    }

    fn render_json(&self, file_db: &FileDb) {
        let errors = match &self {
            PipelineFailure::IOFailure(_) => todo!(),
            PipelineFailure::SrcFailure(_) => todo!(),
            PipelineFailure::LexingFailure(_) => todo!(),
            PipelineFailure::ParsingFailure(errors) => errors
                .iter()
                .map(|(file_id, err)| {
                    let file = file_db.get(*file_id);
                    let span_start = file.tokens[err.token_end - 1].span.0;
                    let span_end = file.tokens[err.token_end].span.1;

                    JsonError {
                        message: err.message(),
                        start: span_start,
                        end: span_end,
                        file_id: *file_id,
                    }
                })
                .collect(),
            PipelineFailure::TypecheckFailure(errors) => errors
                .iter()
                .map(|err| {
                    let file_id = err.file_id;
                    let _file = file_db.get(file_id);

                    #[allow(unreachable_code)]
                    JsonError {
                        message: todo!(), /* err.message() */
                        start: todo!(),
                        end: todo!(),
                        file_id,
                    }
                })
                .collect(),
        };
        let root = ParsiphaeJson {
            errors,
            warnings: Vec::new(),
        };

        let as_json = serde_json::to_string_pretty(&root)
            .expect("Due to implementation choices this cannot fail.");
        println!("{as_json}");
    }
}

pub fn process_parsing_result(file_db: &FileDb, file_id: usize, error: &ParsingError) {
    let ParsingError {
        kind, token_end, ..
    } = error;
    let _message = match &kind {
        PEK::InternalParserFailure => "Oops, something went wrong.".to_string(),
        PEK::ReachedEOF => "Reached End of File during parsing.".to_string(),
        PEK::Expected(poss) => format!("Expected to parse {}", poss.reason()),
        PEK::ExpectedOneOf(_poss_vec) => format!("Expected to parse {}", kind.reason()),
        PEK::ExpectedToken(_token) => format!("Expected to parse a {}", kind.reason()),
        PEK::ExpectedOneOfToken(_tokens) => {
            format!("Expected to parse one of: {}", kind.reason())
        }
        PEK::UnexpectedToken(found, expected) => format!(
            "Expected to find {:?} but found {:?} instead",
            found, expected
        ),
        PEK::MissingFunctionName => "This function needs a name.".to_string(),
        _ => "missing message".to_string(),
    };

    let _label = match &kind {
        PEK::InternalParserFailure => "Oops, something went wrong.".to_string(),
        PEK::ReachedEOF => "Reached End of File during parsing.".to_string(),
        PEK::Expected(poss) => format!("Cannot parse {} after this", poss.reason()),
        PEK::ExpectedOneOf(_poss_vec) => format!("Expected to parse {}", kind.reason()),
        PEK::ExpectedToken(_token) => {
            format!("Expected to parse a {} after this", kind.reason())
        }
        PEK::ExpectedOneOfToken(_tokens) => {
            format!("Expected to parse one of: {}", kind.reason())
        }
        PEK::UnexpectedToken(found, expected) => format!(
            "Expected to find {:?} but found {:?} instead",
            found, expected
        ),
        PEK::MissingFunctionName => "Insert a function name here.".to_string(),
        _ => "Missing label".to_string(),
    };

    match &kind {
        PEK::InternalParserFailure => internal_parser_failure(file_db, file_id),
        PEK::ReachedEOF => reached_eof(file_db, file_id),
        PEK::MissingFunctionName => missing_function_name(file_db, file_id, *token_end),
        PEK::MissingFunctionType => missing_function_type(file_db, file_id, *token_end),
        PEK::MissingInstanceName => missing_instance_name(file_db, file_id, *token_end),
        PEK::MissingInstanceType => missing_instance_type(file_db, file_id, *token_end),
        PEK::StatementWithoutSemicolon => statement_without_semicolon(file_db, file_id, *token_end),
        _ => log::error!("Error occured, but no renderer has been implemented yet: '{kind:?}'."),
    }
}

fn emit_source_error(file_db: &FileDb, diagnostic: &Diagnostic<usize>) {
    let mut writer = StandardStream::stderr(termcolor::ColorChoice::Auto);
    let config = codespan_reporting::term::Config {
        before_label_lines: 2,
        after_label_lines: 1,
        ..Default::default()
    };

    emit(&mut writer, &config, file_db, diagnostic).expect("Failed to print error");
}

fn internal_parser_failure(file_db: &FileDb, file_id: usize) {
    log::error!(
        "Oops, internal parser failure in file {:?}",
        file_db.get(file_id).path
    );
}

fn reached_eof(file_db: &FileDb, file_id: usize) {
    let label = Label::primary(file_id, 0..0).with_message("End of File here"); // TODO: fix span
    let diagnostic = Diagnostic::error()
        .with_message("Reached End of File")
        .with_labels(vec![label]);

    emit_source_error(file_db, &diagnostic);
}

fn missing_function_name(file_db: &FileDb, file_id: usize, token_end: usize) {
    let file = file_db.get(file_id);
    let span_start = file.tokens[token_end - 1].span.1;
    let span_end = file.tokens[token_end].span.0;

    let label =
        Label::primary(file_id, span_start..span_end).with_message("The name is missing here");
    let diagnostic = Diagnostic::error()
        .with_message("A function is missing a name")
        .with_labels(vec![label]);

    emit_source_error(file_db, &diagnostic);
}

fn missing_function_type(file_db: &FileDb, file_id: usize, token_end: usize) {
    let file = file_db.get(file_id);
    let span_start = file.tokens[token_end - 1].span.1;
    let span_end = file.tokens[token_end].span.0;

    let label =
        Label::primary(file_id, span_start..span_end).with_message("The type is missing here");
    let diagnostic = Diagnostic::error()
        .with_message("A function is missing a return type")
        .with_labels(vec![label]);

    emit_source_error(file_db, &diagnostic);
}

fn missing_instance_name(file_db: &FileDb, file_id: usize, token_end: usize) {
    let file = file_db.get(file_id);
    let span_start = file.tokens[token_end - 1].span.1;
    let span_end = file.tokens[token_end].span.0;

    let label =
        Label::primary(file_id, span_start..span_end).with_message("The name is missing here");
    let diagnostic = Diagnostic::error()
        .with_message("An instance is missing a name")
        .with_labels(vec![label]);

    emit_source_error(file_db, &diagnostic);
}
fn missing_instance_type(file_db: &FileDb, file_id: usize, token_end: usize) {
    let file = file_db.get(file_id);
    let span_start = file.tokens[token_end - 1].span.1;
    let span_end = file.tokens[token_end].span.0;

    let label =
        Label::primary(file_id, span_start..span_end).with_message("The type is missing here");
    let diagnostic = Diagnostic::error()
        .with_message("An instance is missing a type")
        .with_labels(vec![label]);

    emit_source_error(file_db, &diagnostic);
}

fn statement_without_semicolon(file_db: &FileDb, file_id: usize, token_end: usize) {
    let file = file_db.get(file_id);
    let span_start = file.tokens[token_end - 1].span.1;

    let label = Label::primary(file_id, span_start..span_start)
        .with_message("The semicolon is missing here");
    let diagnostic = Diagnostic::error()
        .with_message("A statement is missing a semicolon at the end.")
        .with_labels(vec![label]);

    emit_source_error(file_db, &diagnostic);
}

pub fn process_typecheck_error(file_db: &FileDb, error: &TypecheckError) {
    emit_diagnostics(error.to_diagnostic(), file_db);
}

pub fn emit_diagnostics(diagnostics: DiagnosticP, file_db: &FileDb) {
    let labels = diagnostics
        .labels
        .into_iter()
        .map(|lbl| lbl.to_report())
        .collect();
    let report = Diagnostic::error()
        .with_message(diagnostics.message)
        .with_code(diagnostics.code)
        .with_labels(labels);

    emit_source_error(file_db, &report);
}
