#![allow(unused)]
pub enum ErrorType {
    Simple(ErrorKind),
    SimpleMessage(ErrorKind, &'static &'static str),
}

#[derive(Debug)]
pub enum ErrorKind {
    EndOfStream,
    Html,
    InvalidData,
    InvalidHtmlTag,
    InvalidInput,
    InvalidParameters,
    InvalidSearch,
    InvalidUrl,
    Failed,
    Parse,
    Unknown,
}

impl ErrorKind {
    pub(crate) fn to_str(&self) -> &'static str {
        use ErrorKind::*;

        match *self {
            EndOfStream => "end of stream",
            Html => "html error",
            InvalidData => "invalid data",
            InvalidHtmlTag => "invalid HTML tag",
            InvalidInput => "invalid input",
            InvalidParameters => "invalid parameters",
            InvalidSearch => "invalid search parameter",
            InvalidUrl => "invalid url",
            Failed => "failed",
            Parse => "parsing error",
            Unknown => "unknown error has occurred",
        }
    }
}

pub struct Error {
    repr: ErrorType,
}

impl Error {
    pub fn new(error: ErrorType) -> Error {
        Error { repr: error }
    }

    pub fn to_string(self) -> String {
        use ErrorType::*;

        match self.repr {
            Simple(err) => format!("{}", err.to_str()),
            SimpleMessage(err, &msg) => format!("{}: {}", err.to_str(), msg),
        }
    }
}

impl From<ErrorKind> for Error {
    #[inline]
    fn from(kind: ErrorKind) -> Self {
        Error {
            repr: ErrorType::Simple(kind),
        }
    }
}
