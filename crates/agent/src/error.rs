//

use thiserror::Error;
use markdown_frontmatter::Error as FrontMatterError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to deserialize agent: {0}")]
    Deserialize(#[from] FrontMatterError),
}