use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum SnippError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
    },

    #[error("Deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct ParsePrivacyError(pub String);

impl fmt::Display for ParsePrivacyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid privacy value: {:?}", self.0)
    }
}

impl std::error::Error for ParsePrivacyError {}
