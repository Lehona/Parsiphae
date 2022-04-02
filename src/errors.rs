use crate::parser;

pub type Result<O> = ::std::result::Result<O, Error>;

#[derive(Debug)]
pub enum Error {
    ParsingError(parser::errors::ParsingError),
    IOError(::std::io::Error), //LinkingError(LinkerError),
                               //TypeCheckError(TypeError)
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl From<parser::errors::ParsingError> for Error {
    fn from(e: parser::errors::ParsingError) -> Self {
        Error::ParsingError(e)
    }
}
