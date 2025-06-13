use thiserror::Error;

/// Errors that can occur during webpack graph processing
#[derive(Error, Debug)]
pub enum WebpackGraphError {
    #[error("Failed to parse JavaScript: {0}")]
    ParseError(String),

    #[error("Invalid webpack bundle format: {0}")]
    InvalidBundleFormat(String),

    #[error("Module not found: {0}")]
    ModuleNotFound(String),

    #[error("Failed to extract webpack modules: {0}")]
    ModuleExtractionError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
} 