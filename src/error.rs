use std::{path::PathBuf, result};

use {std::io, thiserror};

pub type Result<T> = result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("cannot open file {0}: {1}")]
    CannottOpenFile(PathBuf, #[source] io::Error),
    #[error("cannot write to file {0}: {1}")]
    CannottWriteFile(PathBuf, #[source] io::Error),
    #[error("input failed: {0}")]
    InputFailure(#[source] io::Error),
    #[error("output failed: {0}")]
    OutputFailure(#[source] io::Error),
    #[error("unexpected closing bracket")]
    UnexpectedClosingBracket,
    #[error("missing closing bracket")]
    MissingClosingBracket,
}
