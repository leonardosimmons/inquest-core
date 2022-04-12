#![allow(unused)]

#[derive(Debug)]
pub enum ErrorType {
    Simple(ErrorKind),
    SimpleMessage(ErrorKind, &'static &'static str),
}

#[derive(Debug)]
pub enum ErrorKind {
    Create,
    Delete,
    EndOfStream,
    Failed,
    FileNotFound,
    Html,
    InvalidData,
    InvalidHtmlTag,
    InvalidInput,
    InvalidParameters,
    InvalidPath,
    InvalidSearch,
    InvalidUrl,
    InvalidUtf8,
    NotFound,
    Parse,
    Unimplemented,
    Unknown,
}

impl ErrorKind {
    pub(crate) fn to_str(&self) -> &'static str {
        use ErrorKind::*;

        match *self {
            Create => "unable to create",
            Delete => "unable to delete",
            EndOfStream => "end of stream",
            Failed => "failed",
            FileNotFound => "file not found",
            Html => "html error",
            InvalidData => "invalid data",
            InvalidHtmlTag => "invalid HTML tag",
            InvalidInput => "invalid input",
            InvalidParameters => "invalid parameters",
            InvalidPath => "invalid path",
            InvalidSearch => "invalid search parameter",
            InvalidUrl => "invalid url",
            InvalidUtf8 => "invalid UTF-8",
            NotFound => "not found",
            Parse => "parsing error",
            Unimplemented => "unimplemented",
            Unknown => "unexpected error has occurred",
        }
    }
}

#[derive(Debug)]
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
