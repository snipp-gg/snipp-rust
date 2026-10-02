use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum SnippError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// A non-2xx response. `kind` is the response's `error.type`, such as
    /// `not_found` or `quota_exceeded`, and `message` its `error.message`;
    /// when the response did not carry them, `kind` is `None` and `message`
    /// is the HTTP status text. `body` is the parsed JSON response body, or
    /// `None` when the response was not JSON.
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        kind: Option<String>,
        message: String,
        body: Option<serde_json::Value>,
    },

    #[error("invalid input: {0}")]
    Validation(String),

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
