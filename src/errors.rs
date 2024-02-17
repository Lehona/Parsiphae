use crate::{file::db::FileId, parser::errors::ParsingError, ppa::errors::TypecheckError};
use std::path::PathBuf;

#[derive(Debug)]
pub struct SrcError {
    message: String,
}

impl SrcError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Debug)]
pub enum PipelineError {
    LexingFailed,
    ParsingFailed,
    TypecheckFailed,
}

#[derive(Debug)]
pub struct LexerError {
    message: String,
}

impl From<anyhow::Error> for LexerError {
    fn from(value: anyhow::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum PipelineFailure {
    IOFailure(std::io::Error),
    /// Something went wrong during SrcParsing, which is not a direct IOError
    SrcFailure(SrcError),
    /// Something failed during the lexing stage.
    /// Because files are added to the FileDB after lexing,
    /// at this stage all we have is a Path, not a FileID
    LexingFailure(Vec<(PathBuf, LexerError)>),
    /// Errors occured during the parsing stage.
    ParsingFailure(Vec<(FileId, ParsingError)>),
    /// Errors occured during typechecking.
    TypecheckFailure(Vec<(FileId, TypecheckError)>),
}

impl From<std::io::Error> for PipelineFailure {
    fn from(e: std::io::Error) -> Self {
        PipelineFailure::IOFailure(e)
    }
}

impl From<SrcError> for PipelineFailure {
    fn from(e: SrcError) -> Self {
        PipelineFailure::SrcFailure(e)
    }
}
