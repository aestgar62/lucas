//

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Embedding error: {0}")]
    Embedding(String),
    #[error("FastEmbed error: {0}")]
    FastEmbed(#[from] fastembed::Error),
    #[error("Invalid model name: {0}")]
    InvalidModelName(String),
}