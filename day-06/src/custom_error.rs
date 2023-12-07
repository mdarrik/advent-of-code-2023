use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error("Error parsing input")]
    #[diagnostic(code(aoc::nom::error))]
    NomParseError(String),

    #[error("Error converting from usize to u32")]
    #[diagnostic(code(std::convert::TryFrom::Error))]
    TryFromUsizeError(String)
}