use crate::{file::FileDb, parser::errors::ParsingError};
use serde::Serialize;

#[derive(Serialize)]
pub struct ParsiphaeJson {
    pub errors: Vec<JsonError>,
    pub warnings: Vec<JsonWarning>,
}

#[derive(Serialize)]
pub struct JsonError {
    pub message: String,
    pub start: usize,
    pub end: usize,
    pub file_id: usize,
}

#[derive(Serialize)]
pub struct JsonWarning {
    pub message: String,
    pub start: usize,
    pub end: usize,
}

pub struct MachineReadableOutput;

impl MachineReadableOutput {
    pub fn process(
        file_db: &FileDb,
        results: Vec<(usize, ParsingError)>,
    ) -> Result<(), &'static str> {
        let errors: Vec<_> = results
            .into_iter()
            .map(|(file_id, err)| {
                let file = file_db.get(file_id);
                let span_start = file.tokens.as_ref().unwrap()[err.token_end - 1].span.0;
                let span_end = file.tokens.as_ref().unwrap()[err.token_end].span.1;

                JsonError {
                    message: err.kind.reason(),
                    start: span_start,
                    end: span_end,
                    file_id,
                }
            })
            .collect();
        let root = ParsiphaeJson {
            errors,
            warnings: Vec::new(),
        };

        let as_json = serde_json::to_string_pretty(&root).map_err(|_| "Something went wrong")?;
        println!("{as_json}");
        Ok(())
    }
}
