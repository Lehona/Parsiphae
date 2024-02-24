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
