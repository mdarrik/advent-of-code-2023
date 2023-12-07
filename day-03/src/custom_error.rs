use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(code(aoc::ndarray_error))]
    NdArrayError(#[from] ndarray::ShapeError),

    #[error("Error parsing input")]
    #[diagnostic(code(aoc::nom::error))]
    NomParseError(String),
}
