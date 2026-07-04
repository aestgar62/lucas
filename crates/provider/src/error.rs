//

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// The completion stopped because a tool was used.
    #[error("The completion stopped because a tool was used.")]
    ToolUse,
    /// The completion stopped for another reason.
    #[error("The completion stopped for another reason.")]
    Other,
    /// The API request failed.
    #[error("The API request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),
    #[error("The API response returned an error: {0} - {1}")]
    HttpResponse(u16, String),
    /// The API response could not be parsed.
    #[error("The API response could not be parsed: {0}")]
    ParseResponse(#[from] serde_json::Error),
}