use std::io;
use thiserror::Error;

pub type Result<R, E = Error> = std::result::Result<R, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to read '{0}': {1}")]
    FileRead(String, io::Error),

    #[error("missing opening bracket for ']' at location {0}")]
    MissingOpeningBracket(usize),

    #[error("missing closing bracket for '[' at location {0}")]
    MissingClosingBracket(usize),

    #[error("failed to read from stdin: {0}")]
    StdinError(io::Error),

    #[error("failed to write to stdout: {0}")]
    StdoutError(io::Error)
}