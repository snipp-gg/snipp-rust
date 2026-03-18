use std::fmt;

/// Errors that can occur when interacting with the Snipp API.
#[derive(Debug, thiserror::Error)]
pub enum SnippError {
    /// An HTTP request failed.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// The API returned a non-success status code.
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
    },

    /// Failed to deserialize a response body.
    #[error("Deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),

    /// Failed to read a file for upload.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// A privacy level was invalid.
#[derive(Debug, Clone)]
pub struct ParsePrivacyError(pub String);

impl fmt::Display for ParsePrivacyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid privacy value: {:?}", self.0)
    }
}

impl std::error::Error for ParsePrivacyError {}
