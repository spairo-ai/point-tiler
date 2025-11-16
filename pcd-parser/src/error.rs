use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("unsupported file extension: {extension}")]
    UnsupportedExtension { extension: String },

    #[error("failed to read LAS/LAZ file: {reason}")]
    LasReadError { reason: String },

    #[error("failed to parse CSV file: {reason}")]
    CsvParseError { reason: String },

    #[error("invalid point data: {reason}")]
    InvalidPointData { reason: String },

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("LAS error: {0}")]
    LasError(#[from] las::Error),

    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),
}

pub type Result<T> = std::result::Result<T, ParseError>;
