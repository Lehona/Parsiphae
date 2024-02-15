use crate::parser::{self, errors::ParsingError};

pub type Result<O> = ::std::result::Result<O, Error>;

#[derive(Debug)]
pub enum Error {
    ParsingError(ParsingError),
    IOError(::std::io::Error),
    SrcError(SrcError),
    //LinkingError(LinkerError),
    //TypeCheckError(TypeError)
}

impl Error {
    pub fn as_parsing_error(self) -> ParsingError {
        match self {
            Error::ParsingError(e) => e,
            _ => panic!("Don't do this"),
        }
    }
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl From<ParsingError> for Error {
    fn from(e: ParsingError) -> Self {
        Error::ParsingError(e)
    }
}

impl From<SrcError> for Error {
    fn from(e: SrcError) -> Self {
        Error::SrcError(e)
    }
}

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
