use std::fmt::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// An error response from the OpenAI API.
#[derive(Error, Debug, serde::Deserialize)]
pub struct OpenAIErrorResponse {
    /// An error from the OpenAI API.
    pub error: OpenAIError,
}

impl Display for OpenAIErrorResponse {
    /// Formats the value using the given formatter.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OpenAI Error {}: {}",
            self.error.error_type, self.error.message
        )
    }
}

/// An error from the OpenAI API.
#[derive(Debug, serde::Deserialize)]
pub struct OpenAIError {
    /// error message.
    pub message: String,

    #[serde(rename = "type")]
    /// error type.
    pub error_type: String,
}

/// A custom error type for this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// An error response from the OpenAI API.
    #[error("OpenAI Error: {0}")]
    ApiError(OpenAIErrorResponse),

    /// An error from the Reqwest crate.
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
