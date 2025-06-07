use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error opening or reading Excel file '{path}': {source}")]
    ExcelError {
        path: PathBuf,
        source: calamine::XlsxError,
    },

    #[error("Calamine Error: {0}")]
    Calamine(#[from] calamine::Error),

    #[error("Failed to parse string '{value}' into an integer")]
    ParseInt {
        value: String,
        source: std::num::ParseIntError,
    },

    #[error("Failed to parse string '{value}' into a float")]
    ParseFloat {
        value: String,
        source: std::num::ParseFloatError,
    },

    #[error("Failed to parse time '{value}': {source}")]
    ParseTime {
        value: String,
        source: chrono::ParseError,
    },

    #[error("Cell '{cell}' not found in sheet '{sheet}'")]
    CellNotFound { sheet: String, cell: String },

    #[error("A custom error occurred: {0}")]
    Custom(String),
}
