use reqwest::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EtlError {
    #[error("API request failed: {0}")]
    ApiError(String),

    #[error("HTTP error {0}: {1}")]
    HttpError(u16, String),

    #[error("Request error: {0}")]
    RequestError(String),

    #[error("Data parsing error: {0}")]
    ParseError(String),

    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("CSV processing error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("JSON processing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Zip archive error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Transformation error: {0}")]
    TransformError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub type Result<T> = std::result::Result<T, EtlError>;

impl From<reqwest::Error> for EtlError {
    fn from(value: Error) -> Self {
        EtlError::ApiError(value.to_string())
    }
}

impl From<backoff::Error<reqwest::Error>> for EtlError {
    fn from(value: backoff::Error<reqwest::Error>) -> Self {
        EtlError::ApiError(value.to_string())
    }
}
