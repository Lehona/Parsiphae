use crate::parser::errors::ParsingError;
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
    pub fn process(results: Vec<(usize, ParsingError)>) -> Result<(), &'static str> {
        let errors: Vec<_> = results
            .into_iter()
            .map(|(file_id, err)| JsonError {
                message: err.kind.reason(),
                start: err.token_start,
                end: err.token_end,
                file_id,
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
