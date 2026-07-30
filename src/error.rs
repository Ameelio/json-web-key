use std::{error::Error as StdError, fmt};

#[derive(Clone, Debug)]
pub struct Error {
    col: Option<usize>,
    line: Option<usize>,
    kind: ErrorKind,
    msg: String,
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    JsonParsingError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::JsonParsingError => write!(
                f,
                "Json Parsing Error: [line: {}][col: {}]{}",
                self.line.unwrap(),
                self.col.unwrap(),
                self.msg
            ),
        }
    }
}

impl StdError for Error {}

impl From<serde_json::error::Error> for Error {
    fn from(value: serde_json::error::Error) -> Self {
        let col: Option<usize> = Some(value.column());
        let kind = ErrorKind::JsonParsingError;
        let line: Option<usize> = Some(value.line());
        let msg: String = value.to_string();

        Self {
            col,
            kind,
            line,
            msg,
        }
    }
}
