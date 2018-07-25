use inner_errors::ParserError;

pub type Result<O> = ::std::result::Result<O, Error>;

#[derive(Debug)]
pub enum Error {
    ParsingError { err: ParserError, line: usize },
    IOError(::std::io::Error), //LinkingError(LinkerError),
                               //TypeCheckError(TypeError)
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Error::IOError(e)
    }
}
