use std::fmt::Display;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, serde::Deserialize)]
/// An error response from the OpenAI API.
pub struct OpenAIErrorResponse {
    pub error: OpenAIError,
}

impl Display for OpenAIErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OpenAI Error {}: {}", self.error.error_type, self.error.message)
    }
}

#[derive(Debug, serde::Deserialize)]
/// An error from the OpenAI API.
pub struct OpenAIError {
    /// error message.
    pub message: String,

    #[serde(rename = "type")]
    /// error type.
    pub error_type: String,
}

#[derive(Error, Debug)]
pub enum Error {
    ///show message as error
    #[error("OpenAI Error: {0}")]
    ApiError(OpenAIErrorResponse),

    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
