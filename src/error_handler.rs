use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    term::{
        emit,
        termcolor::{self, StandardStream},
    },
};

use crate::{
    errors::Error,
    file::FileDb,
    parser::errors::{ParsingError, ParsingErrorKind as PEK},
    processor::ParsingResult,
};

pub fn process_parsing_result(file_db: &FileDb, result: ParsingResult) {
    if let Err(Error::ParsingError(ParsingError {
        kind,
        token_start,
        token_end,
        ..
    })) = result.result
    {
        let message = match &kind {
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

        let label = match &kind {
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

        println!("{:?}", &kind);
        match &kind {
            PEK::InternalParserFailure => internal_parser_failure(file_db, result.file_id),
            PEK::ReachedEOF => reached_eof(file_db, result.file_id),
            PEK::MissingFunctionName => missing_function_name(file_db, result.file_id, token_end),
            PEK::MissingFunctionType => missing_function_type(file_db, result.file_id, token_end),
            PEK::MissingInstanceName => missing_instance_name(file_db, result.file_id, token_end),
            PEK::MissingInstanceType => missing_instance_type(file_db, result.file_id, token_end),
            PEK::StatementWithoutSemicolon => {
                statement_without_semicolon(file_db, result.file_id, token_end)
            }
            _ => {}
        }

        // let file = file_db.get(result.file_id);
        // let span_start = file.tokens.as_ref().unwrap()[token_start].span.0; // TODO: Fix edge case where file is empty etc.
        // let span_end = file.tokens.as_ref().unwrap()[token_end].span.1;

        // let label = Label::primary(result.file_id, span_start..span_end).with_message(label); // TODO
        // let diagnostic = Diagnostic::error().with_message(message).with_labels(vec![label]);

        // let mut writer = StandardStream::stderr(termcolor::ColorChoice::Auto);
        // let config = codespan_reporting::term::Config {end_context_lines: 3, ..Default::default()};

        // emit(&mut writer, &config, file_db, &diagnostic).expect("Failed to print error");
    }
}

fn emit_source_error(file_db: &FileDb, diagnostic: &Diagnostic<usize>) {
    let mut writer = StandardStream::stderr(termcolor::ColorChoice::Auto);
    let config = codespan_reporting::term::Config {
        end_context_lines: 3,
        ..Default::default()
    };

    emit(&mut writer, &config, file_db, &diagnostic).expect("Failed to print error");
}

fn internal_parser_failure(file_db: &FileDb, file_id: usize) {
    println!(
        "Oops, internal parser failure in file {:?}",
        file_db.get(file_id).path
    );
}

fn reached_eof(file_db: &FileDb, file_id: usize) {
    let label = Label::primary(file_id, 0..0).with_message("End of File here"); // TODO: fix span
    let diagnostic = Diagnostic::error()
        .with_message("Reachd End of File")
        .with_labels(vec![label]);

    emit_source_error(file_db, &diagnostic);
}

fn missing_function_name(file_db: &FileDb, file_id: usize, token_end: usize) {
    let file = file_db.get(file_id);
    let span_start = file.tokens.as_ref().unwrap()[token_end - 1].span.1;
    let span_end = file.tokens.as_ref().unwrap()[token_end].span.0;

    let label =
        Label::primary(file_id, span_start..span_end).with_message("The name is missing here");
    let diagnostic = Diagnostic::error()
        .with_message("A function is missing a name")
        .with_labels(vec![label]);

    emit_source_error(file_db, &diagnostic);
}

fn missing_function_type(file_db: &FileDb, file_id: usize, token_end: usize) {
    let file = file_db.get(file_id);
    let span_start = file.tokens.as_ref().unwrap()[token_end - 1].span.1;
    let span_end = file.tokens.as_ref().unwrap()[token_end].span.0;

    let label =
        Label::primary(file_id, span_start..span_end).with_message("The type is missing here");
    let diagnostic = Diagnostic::error()
        .with_message("A function is missing a return type")
        .with_labels(vec![label]);

    emit_source_error(file_db, &diagnostic);
}

fn missing_instance_name(file_db: &FileDb, file_id: usize, token_end: usize) {
    let file = file_db.get(file_id);
    let span_start = file.tokens.as_ref().unwrap()[token_end - 1].span.1;
    let span_end = file.tokens.as_ref().unwrap()[token_end].span.0;

    let label =
        Label::primary(file_id, span_start..span_end).with_message("The name is missing here");
    let diagnostic = Diagnostic::error()
        .with_message("An instance is missing a name")
        .with_labels(vec![label]);

    emit_source_error(file_db, &diagnostic);
}
fn missing_instance_type(file_db: &FileDb, file_id: usize, token_end: usize) {
    let file = file_db.get(file_id);
    let span_start = file.tokens.as_ref().unwrap()[token_end - 1].span.1;
    let span_end = file.tokens.as_ref().unwrap()[token_end].span.0;

    let label =
        Label::primary(file_id, span_start..span_end).with_message("The type is missing here");
    let diagnostic = Diagnostic::error()
        .with_message("An instance is missing a type")
        .with_labels(vec![label]);

    emit_source_error(file_db, &diagnostic);
}

fn statement_without_semicolon(file_db: &FileDb, file_id: usize, token_end: usize) {
    let file = file_db.get(file_id);
    let span_start = file.tokens.as_ref().unwrap()[token_end - 1].span.1;

    let label = Label::primary(file_id, span_start..span_start)
        .with_message("The semicolon is missing here");
    let diagnostic = Diagnostic::error()
        .with_message("A statement is missing a semicolon at the end.")
        .with_labels(vec![label]);

    emit_source_error(file_db, &diagnostic);
}
