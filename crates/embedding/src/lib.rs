//

mod embedder;
mod error;

#[cfg(feature = "fastembed")]
mod fastembed;

pub use error::Error;
pub use embedder::Embedder;
